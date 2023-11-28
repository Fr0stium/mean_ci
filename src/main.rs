use std::env;

mod data;
mod evaluate;
mod websites;

#[tokio::main]
async fn main() {
    let args = env::args().collect::<Vec<String>>();
    match args.len() {
        4 => websites::aoty::output(&args).await,
        5 => data::output(&args),
        _ => panic!("Wrong number of arguments specified."),
    }
}
