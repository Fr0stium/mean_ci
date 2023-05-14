use crate::evaluate;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

/// Converts the data in "data.txt" to a vector of floats. "data.txt" must have a format of comma-separated numbers.
fn get_ratings(data_file: &str) -> Vec<f64> {
    let file = File::open(data_file).expect("Could not open file.");
    let reader = BufReader::new(file);
    let mut ratings = Vec::<f64>::new();
    for line in reader.lines() {
        ratings.append(
            &mut line
                .unwrap()
                .split(',')
                .filter_map(|str| str.trim().parse::<f64>().ok())
                .collect::<Vec<f64>>(),
        );
    }
    ratings.sort_by(|a, b| a.total_cmp(b));
    ratings
}

pub fn output(args: &[String]) {
    let data_file = &args[1];
    let alpha = args[2]
        .parse::<f64>()
        .expect("Could not convert 'alpha' into a number.");
    let min_support = args[3]
        .parse::<f64>()
        .expect("Could not convert 'min_support' into a number.");
    let max_support = args[4]
        .parse::<f64>()
        .expect("Could not convert 'max_support' into a number.");
    if alpha <= 0. || alpha >= 1. {
        panic!("The argument 'alpha' ({alpha}) must be greater than 0 and less than 1.");
    }
    let ratings = get_ratings(data_file);
    let confidence_level = 100. * (1. - alpha);
    let n = ratings.len() as f64;
    let mean = ratings.iter().sum::<f64>() / n;
    let mean_ci = evaluate::get_mean_ci(&ratings, alpha, min_support, max_support);
    println!("\nNumber of Ratings: {n}\nMean: {mean}\n{confidence_level}% Confidence Interval: {mean_ci}");
}
