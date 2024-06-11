use crate::evaluate::{self, ConfidenceInterval};
use regex::Regex;
use reqwest;
use scraper::{Html, Selector};
use std::fmt;
use tokio::time::{sleep, Duration};

const MIN_SUPPORT: f64 = 0.0;
const MAX_SUPPORT: f64 = 100.0;
const DELAY: u64 = 0; // How often to send a request (in milliseconds).
const RATINGS_PER_PAGE: usize = 80; // Maximum number of ratings per page.
const ALBUMS_PER_PAGE: usize = 25; // Maximum number of albums per page (on the charts).

struct Album {
    id: i32,
    artist: String,
    title: String,
}

impl fmt::Debug for Album {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}, \"{} - {}\"", self.id, self.artist, self.title)
    }
}

async fn get_webpage(url: &String) -> String {
    let client = reqwest::Client::new();
    loop {
        match client
            .get(url)
            .header(reqwest::header::USER_AGENT, "CI")
            .send()
            .await
        {
            Ok(webpage) => return webpage.text().await.unwrap(), // Should always contain text.
            Err(err) => println!("Retrying due to error (Ctrl + C to exit): {err}"),
        }
    }
}

/// Given an album with id `id`, this returns the HTML code as a String
/// for the album.
async fn get_album_page(id: i32) -> String {
    let url = format!("https://www.albumoftheyear.org/album/{id}");

    get_webpage(&url).await
}

/// Given an album with id `id`, this returns the HTML code as a String
/// for the webpage containing user ratings at page `page_number`.
async fn get_album_ratings_page(id: i32, page_number: i32) -> String {
    let url = format!(
        "https://www.albumoftheyear.org/album/{id}/user-reviews/?p={page_number}&type=ratings"
    );

    get_webpage(&url).await
}

/// Given a chart of year `year`, this returns the HTML code as a String
/// for the webpage containing the albums at page `page_number`.
async fn get_chart_page(year: i32, page_number: i32) -> String {
    if year < 1950 {
        panic!("Year should be at least 1950.")
    }
    let url =
        format!("https://www.albumoftheyear.org/ratings/user-highest-rated/{year}/{page_number}/");

    get_webpage(&url).await
}

/// Returns info for the album with id `id`.
async fn get_album(id: i32) -> Album {
    let album_artist_selector =
        Selector::parse("div.albumHeadline > h1 > div.artist > span > span > a").unwrap();
    let album_title_selector =
        Selector::parse("div.albumHeadline > h1 > div.albumTitle > span").unwrap();
    let webpage_text = get_album_page(id).await;
    let document = Html::parse_document(&webpage_text);
    let artist = document
        .select(&album_artist_selector)
        .map(|element| element.inner_html())
        .next()
        .unwrap_or_else(|| String::from("Various Artists")); // Could fail if there's no link in album title.
    let title = document
        .select(&album_title_selector)
        .map(|element| element.inner_html())
        .next()
        .unwrap_or_else(|| String::from("???")); // Should not fail, but you never know...

    Album { id, artist, title }
}

/// Returns a list of ratings for an album with id `id`.
async fn get_album_ratings(id: i32) -> Vec<f64> {
    let mut ratings = Vec::<f64>::new();
    for page_number in 1.. {
        let mut page_ratings_count = 0;
        let webpage_text = get_album_ratings_page(id, page_number).await;
        let document = Html::parse_document(&webpage_text);
        let user_rating_selector = Selector::parse("div.rating").unwrap();
        document
            .select(&user_rating_selector)
            .map(|element| element.inner_html())
            .skip(1)
            .filter_map(|element| element.parse::<f64>().ok())
            .for_each(|rating| {
                ratings.push(rating);
                page_ratings_count += 1;
            });
        println!(
            "Scraping page {page_number} of album {id}... Added {page_ratings_count} ratings."
        );
        // Less than 80 ratings on the page means this is the last page.
        if page_ratings_count < RATINGS_PER_PAGE {
            break;
        }
        // Wait DELAY milliseconds before sending the next request.
        sleep(Duration::from_millis(DELAY)).await;
    }
    ratings.reverse(); // Ratings collected from scraping are in descending order, so reverse them.

    ratings
}

/// Returns a ranking of albums for the year `year` by computing a confidence interval
/// of the user score of each album with significance level `alpha`, and then sorting
/// the albums by the lower bound of each album's CI in descending order.
pub async fn output_chart_rankings(year: i32, alpha: f64) {
    let re = Regex::new(r"^\/album\/([0-9]+)-+(.+)\.php").unwrap(); // Valid regex pattern. Extracts the Album ID from a link.
    let mut album_ids = Vec::<i32>::new();
    let mut count = album_ids.len();
    for page in 1.. {
        let webpage_text = get_chart_page(year, page).await;
        let document = Html::parse_document(&webpage_text);
        let link_selector = Selector::parse("span > a").unwrap(); // Valid selector.
        document
            .select(&link_selector)
            .filter_map(|n| n.value().attr("href"))
            .filter(|link| link.starts_with("/album/"))
            .map(|f| re.captures(f).unwrap().get(1).unwrap().as_str())
            .for_each(|f| album_ids.push(f.parse::<i32>().unwrap()));
        let new_count = album_ids.len();
        if new_count - count < ALBUMS_PER_PAGE {
            break;
        }
        count = new_count;
    }
    let mut info = Vec::<(usize, Album, ConfidenceInterval)>::new();
    for (i, &album_id) in album_ids.iter().enumerate() {
        let album = get_album(album_id).await;
        let ratings = get_album_ratings(album_id).await;
        let rank = i + 1;
        let mean_ci = evaluate::get_mean_ci(&ratings, alpha, MIN_SUPPORT, MAX_SUPPORT);
        info.push((rank, album, mean_ci));
    }
    info.sort_by(|(_, _, c1), (_, _, c2)| c2.lower_bound.partial_cmp(&c1.lower_bound).unwrap());
    let mut new_rank = 1;
    println!();
    for (old_rank, album, ci) in info {
        if let (n, lower_bound, Some(mean), upper_bound) =
            (ci.n, ci.lower_bound, ci.mean, ci.upper_bound)
        {
            match (new_rank as i32).cmp(&(old_rank as i32)) {
                std::cmp::Ordering::Less => {
                    let difference = old_rank - new_rank;
                    println!(
                        "{new_rank}, +{difference}, {:?}, {mean}, {lower_bound}, {upper_bound}, {n}",
                        album
                    );
                }
                std::cmp::Ordering::Equal => {
                    println!(
                        "{new_rank}, 0, {:?}, {mean}, {lower_bound}, {upper_bound}, {n}",
                        album
                    );
                }
                std::cmp::Ordering::Greater => {
                    let difference = new_rank - old_rank;
                    println!(
                        "{new_rank}, -{difference}, {:?}, {mean}, {lower_bound}, {upper_bound}, {n}",
                        album
                    );
                }
            };
            new_rank += 1;
        }
    }
}

pub async fn output_album_ci(id: i32, alpha: f64) {
    let album = get_album(id).await;
    let ratings = get_album_ratings(id).await;
    let confidence_level = 100. * (1. - alpha);
    let n = ratings.len() as f64;
    let mean = ratings.iter().sum::<f64>() / n;
    let mean_ci = evaluate::get_mean_ci(&ratings, alpha, MIN_SUPPORT, MAX_SUPPORT);
    println!("\n{:?}\nNumber of Ratings: {n}\nMean: {mean}\n{confidence_level}% Confidence Interval: {mean_ci}", album);
}
