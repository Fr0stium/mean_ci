use special_fun::cephes_double::incbi;
use std::fmt::Display;

pub struct ConfidenceInterval {
    lower_bound: f64,
    upper_bound: f64,
}

impl Display for ConfidenceInterval {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}, {}]", self.lower_bound, self.upper_bound)
    }
}

/// Gets the 100(1-`alpha`)% confidence interval for the mean of the ratings.
pub fn get_mean_ci(ratings: &Vec<f64>, alpha: f64, min_support: f64, max_support: f64) -> ConfidenceInterval {
    let n = ratings.len();
    if n < 1 {
        return ConfidenceInterval {
            lower_bound: min_support,
            upper_bound: max_support,
        };
    }
    let [min, max] = [ratings[0], ratings[n - 1]];
    if min < min_support {
        panic!("The minimum value of the dataset ({min}) is less than the given minimum support ({min_support})")
    }
    if max > max_support {
        panic!("The maximum value of the dataset ({max}) is greater than the given maximum support ({max_support})")
    }
    let n = n as f64;
    // Gets the 100(1-`alpha`)% confidence interval for a particular value of the CDF.
    let get_cdf_ci = |x: f64| -> ConfidenceInterval {
        let ratings_below_x = ratings.iter().filter(|&&rating| rating <= x).count() as f64;
        let mut cdf_ci = ConfidenceInterval {
            lower_bound: 0.,
            upper_bound: 1.,
        };
        if x >= min {
            cdf_ci.lower_bound = incbi(ratings_below_x, n - ratings_below_x + 1., alpha / 2.);
        }
        if x < max {
            cdf_ci.upper_bound = incbi(1. + ratings_below_x, n - ratings_below_x, 1. - alpha / 2.);
        }
        cdf_ci
    };
    let mut mean_ci = ConfidenceInterval {
        lower_bound: max - (min - min_support) * get_cdf_ci(min_support).upper_bound,
        upper_bound: max_support - (max_support - max) * get_cdf_ci(max).lower_bound,
    };
    let mut unique_ratings = ratings.clone();
    unique_ratings.dedup();
    for w in unique_ratings.windows(2) {
        let [curr, next] = [w[0], w[1]];
        let cdf_ci = get_cdf_ci(curr);
        mean_ci.lower_bound -= (next - curr) * cdf_ci.upper_bound;
        mean_ci.upper_bound -= (next - curr) * cdf_ci.lower_bound;
    }
    mean_ci
}
