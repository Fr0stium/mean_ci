use reqwest;
use scraper;
use std::{error::Error, time::Instant};
use tokio::time::{sleep, Duration};

use crate::evaluate;

const DELAY: u128 = 1000; // How often to send a request (in milliseconds).
const RATINGS_PER_PAGE: usize = 80; // Maximum number of ratings per page.

enum MusicType {
    Song,
    Album,
}

async fn get_ratings(music_type: MusicType, id: i32) -> Result<Vec<f64>, Box<dyn Error>> {
    let mut ratings = Vec::<f64>::new();

    for i in 1.. {
        let time_start = Instant::now();

        let url = match music_type {
            MusicType::Song => todo!(),
            MusicType::Album => format!(
                "https://www.albumoftheyear.org/album/{id}/user-reviews/?p={i}&type=ratings"
            ),
        };

        let client = reqwest::Client::new();
        let webpage_text = client
            .get(url)
            .header(reqwest::header::USER_AGENT, "CI")
            .send()
            .await?
            .text()
            .await?;

        let document = scraper::Html::parse_document(&webpage_text);
        let user_rating_selector = scraper::Selector::parse("div.rating")?;

        let page_ratings = document
            .select(&user_rating_selector)
            .map(|element| element.inner_html())
            .skip(1)
            .map(|element| element.parse::<f64>().unwrap())
            .collect::<Vec<f64>>();

        for &rating in page_ratings.iter() {
            ratings.push(rating)
        }

        let page_ratings_count = page_ratings.len();

        println!("Scraping page {i} of album {id}... Added {page_ratings_count} ratings.");

        // Less than 80 ratings on the page means this is the last page.
        if page_ratings_count < RATINGS_PER_PAGE {
            break;
        }

        // Wait DELAY milliseconds before sending the next request.
        let time_difference = time_start.elapsed().as_millis();
        if time_difference < DELAY {
            let sleep_time = DELAY - time_difference;
            sleep(Duration::from_millis(sleep_time as u64)).await;
        }
    }

    ratings.sort_by(|a, b| a.total_cmp(b));

    Ok(ratings)
}

pub async fn output(args: Vec<String>) {
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

    let ratings = get_ratings(music_type, id).await.unwrap();

    let confidence_level = 100. * (1. - alpha);
    let n = ratings.len() as f64;
    let mean = ratings.iter().sum::<f64>() / n;
    let mean_ci = evaluate::get_mean_ci(ratings, alpha, min_support, max_support);

    println!("\nNumber of Ratings: {n}\nMean: {mean}\n{confidence_level}% Confidence Interval: {mean_ci:?}")
}
