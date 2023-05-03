use special_fun::cephes_double::incbi;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

const MIN_SUPPORT: f64 = 0.;
const MAX_SUPPORT: f64 = 100.;

/// Converts the data in data.txt to a vector of floats. data.txt must have a format of comma-separated numbers.
fn get_ratings() -> Vec<f64> {
    let file = match File::open("data.txt") {
        Ok(file) => file,
        Err(err) => panic!("Problem opening the file: {err}"),
    };

    let reader = BufReader::new(file);
    let mut ratings = Vec::<f64>::new();

    for line in reader.lines() {
        match line {
            Ok(line) => {
                ratings.append(
                    &mut line
                        .split(',')
                        .into_iter()
                        .filter_map(|x| x.parse::<f64>().ok())
                        .collect::<Vec<f64>>(),
                );
            }
            Err(err) => panic!("Something went wrong: {err}"),
        };
    }

    ratings.sort_by(|a, b| a.total_cmp(b));
    ratings
}

/// Gets the 100(1-α)% confidence interval for the mean of the dataset.
fn get_confidence_interval(alpha: f64) -> (f64, f64) {
    let ratings = get_ratings();
    let n = ratings.len() as f64;

    if n < 1. {
        return (MIN_SUPPORT, MAX_SUPPORT);
    }

    let mut unique_ratings = ratings.clone();
    unique_ratings.dedup();

    let mut min = MAX_SUPPORT;
    let mut max = MIN_SUPPORT;

    for &rating in ratings.iter() {
        if rating < min {
            min = rating;
        }
        if rating > max {
            max = rating;
        }
    }

    let get_cdf_ci = |x: f64| -> (f64, f64) {
        let ratings_below_x = ratings.iter().filter(|&&rating| rating <= x).count() as f64;
        let (mut lower_cdf_ci, mut upper_cdf_ci) = (0., 1.);

        if x >= min {
            lower_cdf_ci = incbi(ratings_below_x, n - ratings_below_x + 1., alpha / 2.);
        }

        if x < max {
            upper_cdf_ci = incbi(1. + ratings_below_x, n - ratings_below_x, 1. - alpha / 2.);
        }

        (lower_cdf_ci, upper_cdf_ci)
    };

    let mut lower_ci_sum = 0.;
    let mut upper_ci_sum = 0.;

    for i in 0..unique_ratings.len() - 1 {
        let (lower_cdf_ci, upper_cdf_ci) = get_cdf_ci(unique_ratings[i]);
        lower_ci_sum += (unique_ratings[i + 1] - unique_ratings[i]) * upper_cdf_ci;
        upper_ci_sum += (unique_ratings[i + 1] - unique_ratings[i]) * lower_cdf_ci;
    }

    let lower_ci = max - (min - MIN_SUPPORT) * get_cdf_ci(MIN_SUPPORT).1 - lower_ci_sum;
    let upper_ci = MAX_SUPPORT - (MAX_SUPPORT - max) * get_cdf_ci(max).0 - upper_ci_sum;

    (lower_ci, upper_ci)
}

fn main() {
    let alpha = 0.05;
    let confidence_level = 100. * (1. - alpha);

    let n = get_ratings().len();
    let mean = get_ratings().iter().sum::<f64>() / (n as f64);
    let confidence_interval = get_confidence_interval(alpha);

    println!("Number of ratings: {n}\nMean: {mean}\n{confidence_level}% confidence interval: {confidence_interval:?}")
}
