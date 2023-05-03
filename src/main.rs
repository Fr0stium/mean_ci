use special_fun::cephes_double::incbi;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

const MIN_SUPPORT: f64 = 0.;
const MAX_SUPPORT: f64 = 100.;

/// Converts the data in data.txt to a vector of floats. data.txt must have a format of comma-separated values.
fn get_ratings() -> Vec<f64> {
    let mut ratings = Vec::new();

    let file = File::open("data.txt").unwrap();
    let reader = BufReader::new(file);

    for line in reader.lines().flatten() {
        let info: Vec<&str> = line.split(',').collect();
        for rating in info {
            let rating: f64 = rating.parse().unwrap();
            ratings.push(rating);
        }
    }

    ratings.sort_by(|a, b| a.partial_cmp(b).unwrap());
    ratings
}

/// Gets the (1-α)% confidence interval for the mean of the dataset.
fn get_ci(alpha: &f64) -> (f64, f64) {
    let ratings = get_ratings();
    let mut unique_ratings = ratings.clone();
    unique_ratings.dedup();

    let max = ratings.iter().max_by(|x, y| x.total_cmp(y)).unwrap();
    let min = ratings.iter().min_by(|x, y| x.total_cmp(y)).unwrap();

    let n = ratings.len() as f64;

    let lower_cdf = |x: &f64| {
        let nfn = ratings.iter().filter(|rating| rating <= &&x).count() as f64;

        if x < min {
            return 0.;
        }

        incbi(nfn, n - nfn + 1., alpha / 2.)
    };

    let upper_cdf = |x: &f64| {
        let nfn = ratings.iter().filter(|rating| rating <= &&x).count() as f64;

        if x >= max {
            return 1.;
        }

        incbi(1. + nfn, n - nfn, 1. - alpha / 2.)
    };

    let mut lower_ci_sum = 0.;
    let mut upper_ci_sum = 0.;

    for i in 0..unique_ratings.len() - 1 {
        lower_ci_sum += (unique_ratings[i + 1] - unique_ratings[i]) * upper_cdf(&unique_ratings[i]);
        upper_ci_sum += (unique_ratings[i + 1] - unique_ratings[i]) * lower_cdf(&unique_ratings[i]);
    }

    let lower_ci = max - (min - MIN_SUPPORT) * upper_cdf(&MIN_SUPPORT) - lower_ci_sum;
    let upper_ci = MAX_SUPPORT - (MAX_SUPPORT - max) * lower_cdf(&max) - upper_ci_sum;

    (lower_ci, upper_ci)
}

fn main() {
    let alpha = 0.05;
    let significance = 100. * (1. - alpha);

    let mean = get_ratings().iter().sum::<f64>() / (get_ratings().len() as f64);
    let ci = get_ci(&alpha);

    println!("Mean: {mean}\n{significance}% Confidence Interval: {ci:?}")
}
