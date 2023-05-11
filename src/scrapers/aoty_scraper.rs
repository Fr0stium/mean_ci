use reqwest;
use scraper;
use std::{error::Error, time::Instant};
use tokio::time::{sleep, Duration};

use crate::evaluate;

const DELAY: u128 = 1000; // How often to send a request (in milliseconds).

async fn get_ratings(id: i32) -> Result<Vec<f64>, Box<dyn Error>> {
    let mut ratings = Vec::<f64>::new();
    let mut current_len = ratings.len();

    let mut i = 1;

    loop {
        let time_start = Instant::now();
        print!("Scraping page {i} of album {id}...");

        let url =
            format!("https://www.albumoftheyear.org/album/{id}/user-reviews/?p={i}&type=ratings");

        let client = reqwest::Client::new();
        let response = client
            .get(url)
            .header(reqwest::header::USER_AGENT, "CI")
            .send()
            .await?
            .text()
            .await?;

        let document = scraper::Html::parse_document(&response);
        let user_rating_selector = scraper::Selector::parse("div.rating")?;

        document
            .select(&user_rating_selector)
            .map(|x| x.inner_html())
            .skip(1) // The first element is the overall track rating.
            .for_each(|rating| ratings.push(rating.parse::<f64>().unwrap()));

        let diff = ratings.len() - current_len;

        if diff == 0 {
            println!(" Added 0 ratings.");
            break;
        }

        current_len = ratings.len();
        i += 1;

        println!(" Added {diff} ratings.");

        // Only send a request once every DELAY milliseconds:
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
    let id = args[1]
        .parse::<i32>()
        .expect("Could not convert ID into a number");
    let alpha = args[2]
        .parse::<f64>()
        .expect("Could not convert 'alpha' into a number");
    let min_support = args[3]
        .parse::<f64>()
        .expect("Could not convert 'min_support' into a number");
    let max_support = args[4]
        .parse::<f64>()
        .expect("Could not convert 'max_support' into a number");

    let ratings = get_ratings(id).await.unwrap();

    let confidence_level = 100. * (1. - alpha);
    let n = ratings.len() as f64;
    let mean = ratings.iter().sum::<f64>() / n;
    let mean_ci = evaluate::get_mean_ci(ratings, alpha, min_support, max_support);

    println!("\nNumber of Ratings: {n}\nMean: {mean}\n{confidence_level}% Confidence Interval: {mean_ci:?}")
}
