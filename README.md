# mean_ci
Calculates a two-tailed confidence interval for the user score of an album on albumoftheyear.org. The confidence interval gives reasonable results for albums that have a low number of ratings, or for albums with weird rating distributions.

## Documentation
The PDF file containing all the math is on GitHub [here](https://github.com/Fr0stium/mean_ci/blob/master/documentation.pdf). It's a bit general, since the idea can be applied to sites other than AOTY.

## Requirements
You must be able to run the `.exe` file provided in the release. If you can't do that, you must download [Rust](https://www.rust-lang.org/tools/install).

## Usage
To calculate the confidence interval for an album, run the following command in the program's directory:

```cargo run -- --album <album_id>```.

By default, this will calculate a 90% confidence interval for the album's user score. I chose 90% because the lower bound provides a 95% one-tailed confidence interval for the user score. In other words, if the confidence interval is `(a, b)`, we can be 90% confident the user score is between `a` and `b`, and we can be 95% confident that the user score is at least `a` (the difference is that there's no upper bound).

Note that the actual user score is unknown, since it is a weighted average, and the weights for each user are not public. However, the unweighted user score is a very good approximation for the true score.

To calculate a confidence interval that is not 90%, run the following command:

```cargo run -- --album <album_id> --alpha <alpha>```,

where `<alpha>` is the significance level. For a 95% confidence interval, this would be `0.05`.

To calculate a chart for a year, run the following command:

```cargo run -- --chart <year>```,

where `<year>` is the year you'd like to calculate. It should be a number between 1950 and 2024. There is a bug that if the number of albums in the chart is a multiple of 25, the program will output albums from another year. For now you have to manually edit the code so that it stops at a specific page number.

It could take a very long time to run a chart. Currently, the delay is set to 200ms. You can edit this to 0ms if you're impatient, but don't lag the site out please.

To calculate a chart for a year with a confidence interval that is not 90%, run the following command:

```cargo run -- --chart <year> --alpha <alpha>```,

where `<alpha>` is the signifiance level.

### Example
To calculate a 95% confidence interval for `JPEGMAFIA - LP! (Offline Version)`, I'd run

```cargo run -- --album 428719 --alpha 0.05```,

which will output the following:

```
428719, "JPEGMAFIA - LP! (Offline Version)"
Number of Ratings: 5318
Mean: 90.42403159082362
95% Confidence Interval: [89.95456439157066, 90.84698643130072]
```