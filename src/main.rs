use std::env;

mod evaluate;

fn main() {
    let args = env::args().collect::<Vec<String>>();

    evaluate::output(args);
}
