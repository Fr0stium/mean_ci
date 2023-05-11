use special_fun::cephes_double::incbi;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

/// Converts the data in "data.txt" to a vector of floats. "data.txt" must have a format of comma-separated numbers.
fn get_ratings(data_file: &String) -> Vec<f64> {
    let file = match File::open(data_file) {
        Ok(file) => file,
        Err(err) => panic!("{err}"),
    };

    let reader = BufReader::new(file);
    let mut ratings = Vec::<f64>::new();

    for line in reader.lines() {
        match line {
            Ok(line) => {
                ratings.append(
                    &mut line
                        .split(',')
                        .filter_map(|x| x.trim().parse::<f64>().ok())
                        .collect::<Vec<f64>>(),
                );
            }
            Err(err) => panic!("{err}. Is your file a text file?"),
        };
    }

    ratings.sort_by(|a, b| a.total_cmp(b));
    ratings
}

/// Gets the 100(1-ALPHA)% confidence interval for the mean of the dataset.
fn get_mean_ci(data_file: &String, alpha: f64, min_support: f64, max_support: f64) -> (f64, f64) {
    let ratings = get_ratings(data_file);
    let n = ratings.len();

    if n < 1 {
        return (min_support, max_support);
    }

    let mut unique_ratings = ratings.clone();
    unique_ratings.dedup();

    let (min, max) = (ratings[0], ratings[n - 1]);

    if min < min_support {
        panic!("The minimum value of the dataset ({min}) is less than the given minimum support ({min_support})")
    }

    if max > max_support {
        panic!("The maximum value of the dataset ({max}) is greater than the given maximum support ({max_support})")
    }

    let n = n as f64;

    // Gets the 100(1-ALPHA)% confidence interval for a particular value of the CDF.
    let get_cdf_ci = |x: f64| -> (f64, f64) {
        let ratings_below_x = ratings.iter().filter(|&&rating| rating <= x).count() as f64;
        let (mut cdf_lower_ci, mut cdf_upper_ci) = (0., 1.);

        if x >= min {
            cdf_lower_ci = incbi(ratings_below_x, n - ratings_below_x + 1., alpha / 2.);
        }

        if x < max {
            cdf_upper_ci = incbi(1. + ratings_below_x, n - ratings_below_x, 1. - alpha / 2.);
        }

        (cdf_lower_ci, cdf_upper_ci)
    };

    let mut cdf_lower_ci_sum = 0.;
    let mut cdf_upper_ci_sum = 0.;

    for i in 0..unique_ratings.len() - 1 {
        let (lower_cdf_ci, upper_cdf_ci) = get_cdf_ci(unique_ratings[i]);
        cdf_lower_ci_sum += (unique_ratings[i + 1] - unique_ratings[i]) * upper_cdf_ci;
        cdf_upper_ci_sum += (unique_ratings[i + 1] - unique_ratings[i]) * lower_cdf_ci;
    }

    let mean_lower_ci = max - (min - min_support) * get_cdf_ci(min_support).1 - cdf_lower_ci_sum;
    let mean_upper_ci = max_support - (max_support - max) * get_cdf_ci(max).0 - cdf_upper_ci_sum;

    (mean_lower_ci, mean_upper_ci)
}

pub fn output(args: Vec<String>) {
    if args.len() != 5 {
        let n = args.len() - 1;
        panic!("Incorrect number of arguments ({n}), expected 4");
    }

    let data_file = &args[1];
    let alpha = args[2]
        .trim()
        .parse::<f64>()
        .expect("Could not convert 'alpha' into a number");
    let min_support = args[3]
        .trim()
        .parse::<f64>()
        .expect("Could not convert 'min_support' into a number");
    let max_support = args[4]
        .trim()
        .parse::<f64>()
        .expect("Could not convert 'max_support' into a number");

    if alpha <= 0. || alpha >= 1. {
        panic!("The argument 'alpha' ({alpha}) must be greater than 0 and less than 1");
    }

    let confidence_level = 100. * (1. - alpha);
    let n = get_ratings(data_file).len() as f64;
    let mean = get_ratings(data_file).iter().sum::<f64>() / n;
    let mean_ci = get_mean_ci(data_file, alpha, min_support, max_support);

    println!("Number of Ratings: {n}\nMean: {mean}\n{confidence_level}% Confidence Interval: {mean_ci:?}")
}
