use crate::evaluate;
use reqwest;
use scraper;
use tokio::time::{sleep, Duration};

const DELAY: u64 = 500; // How often to send a request (in milliseconds).
const RATINGS_PER_PAGE: usize = 80; // Maximum number of ratings per page.

enum MusicType {
    Song,
    Album,
}

/// Given an album with id `id`, this returns the HTML code as a String
/// for the webpage containing user ratings at page `page_number`.
async fn get_webpage_text(music_type: &MusicType, id: i32, page_number: i32) -> String {
    let url = match music_type {
        MusicType::Song => todo!(),
        MusicType::Album => format!(
            "https://www.albumoftheyear.org/album/{id}/user-reviews/?p={page_number}&type=ratings"
        ),
    };
    let client = reqwest::Client::new();
    let webpage = client
        .get(url)
        .header(reqwest::header::USER_AGENT, "CI")
        .send()
        .await
        .unwrap();
    webpage.text().await.unwrap()
}

/// Returns a list of ratings for an album with id `id`.
async fn get_ratings(music_type: &MusicType, id: i32) -> Vec<f64> {
    let mut ratings = Vec::<f64>::new();
    for page_number in 1.. {
        let mut page_ratings_count = 0;
        let webpage_text = get_webpage_text(music_type, id, page_number).await;
        let document = scraper::Html::parse_document(&webpage_text);
        let user_rating_selector = scraper::Selector::parse("div.rating").unwrap();
        document
            .select(&user_rating_selector)
            .map(|element| element.inner_html())
            .skip(1)
            .map(|element| element.parse::<f64>().unwrap())
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

pub async fn output(args: &[String]) {
    let music_type = match args[1].as_str() {
        "song" => MusicType::Song,
        "album" => MusicType::Album,
        _ => panic!("Specify either a song or an album"),
    };
    let id = args[2]
        .parse::<i32>()
        .expect("Could not convert ID into a number");
    let alpha = args[3]
        .parse::<f64>()
        .expect("Could not convert 'alpha' into a number");
    let min_support = args[4]
        .parse::<f64>()
        .expect("Could not convert 'min_support' into a number");
    let max_support = args[5]
        .parse::<f64>()
        .expect("Could not convert 'max_support' into a number");
    let ratings = get_ratings(&music_type, id).await;
    let confidence_level = 100. * (1. - alpha);
    let n = ratings.len() as f64;
    let mean = ratings.iter().sum::<f64>() / n;
    let mean_ci = evaluate::get_mean_ci(&ratings, alpha, min_support, max_support);
    println!("\nNumber of Ratings: {n}\nMean: {mean}\n{confidence_level}% Confidence Interval: {mean_ci:?}");
}
