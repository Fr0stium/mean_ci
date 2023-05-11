use std::env;

mod evaluate;
mod scrapers;

#[tokio::main]
async fn main() {
    let args = env::args().collect::<Vec<String>>();
    match args.len() {
        5 => {
            let id_result = args[1].parse::<i32>();
            match id_result {
                Ok(_) => scrapers::aoty_scraper::output(args).await, // First argument is an ID for AOTY.
                Err(_) => evaluate::output(args), // First argument is a text file path.
            }
        }
        _ => {
            println!("Wrong number of arguments specified.")
        }
    }
}
