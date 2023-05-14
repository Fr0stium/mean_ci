use std::env;

mod data;
mod evaluate;
mod websites;

#[tokio::main]
async fn main() {
    let args = env::args().collect::<Vec<String>>();
    match args.len() {
        5 => data::output(&args),
        6 => websites::aoty::output(&args).await,
        _ => panic!("Wrong number of arguments specified."),
    }
}
