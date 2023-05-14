use special_fun::cephes_double::incbi;

/// Gets the 100(1-ALPHA)% confidence interval for the mean of the ratings.
pub fn get_mean_ci(ratings: &Vec<f64>, alpha: f64, min_support: f64, max_support: f64) -> [f64; 2] {
    let n = ratings.len();
    if n < 1 {
        return [min_support, max_support];
    }
    let [min, max] = [ratings[0], ratings[n - 1]];
    if min < min_support {
        panic!("The minimum value of the dataset ({min}) is less than the given minimum support ({min_support})")
    }
    if max > max_support {
        panic!("The maximum value of the dataset ({max}) is greater than the given maximum support ({max_support})")
    }
    let n = n as f64;
    // Gets the 100(1-ALPHA)% confidence interval for a particular value of the CDF.
    let get_cdf_ci = |x: f64| -> [f64; 2] {
        let ratings_below_x = ratings.iter().filter(|&&rating| rating <= x).count() as f64;
        let (mut cdf_lower_ci, mut cdf_upper_ci) = (0., 1.);
        if x >= min {
            cdf_lower_ci = incbi(ratings_below_x, n - ratings_below_x + 1., alpha / 2.);
        }
        if x < max {
            cdf_upper_ci = incbi(1. + ratings_below_x, n - ratings_below_x, 1. - alpha / 2.);
        }
        [cdf_lower_ci, cdf_upper_ci]
    };
    let mut cdf_lower_ci_sum = 0.;
    let mut cdf_upper_ci_sum = 0.;
    let mut unique_ratings = ratings.clone();
    unique_ratings.dedup();
    for w in unique_ratings.windows(2) {
        let [curr, next] = [w[0], w[1]];
        let [lower_cdf_ci, upper_cdf_ci] = get_cdf_ci(curr);
        cdf_lower_ci_sum += (next - curr) * upper_cdf_ci;
        cdf_upper_ci_sum += (next - curr) * lower_cdf_ci;
    }
    let mean_lower_ci = max - (min - min_support) * get_cdf_ci(min_support)[1] - cdf_lower_ci_sum;
    let mean_upper_ci = max_support - (max_support - max) * get_cdf_ci(max)[0] - cdf_upper_ci_sum;
    [mean_lower_ci, mean_upper_ci]
}
