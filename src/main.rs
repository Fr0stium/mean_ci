use std::env;

mod evaluate;
mod scrapers;

#[tokio::main]
async fn main() {
    let args = env::args().collect::<Vec<String>>();
    match args.len() {
        5 => {
            evaluate::output(args);
        }
        6 => {
            scrapers::aoty_scraper::output(args).await;
        }
        _ => {
            println!("Wrong number of arguments specified.")
        }
    }
}
