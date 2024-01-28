# mean_ci
Calculates a two-tailed confidence interval for the user score of an album on albumoftheyear.org. The confidence interval gives reasonable results for albums that have a low number of ratings, or for albums with weird rating distributions.

## Documentation
The PDF file containing all the math is on GitHub [here](https://github.com/Fr0stium/mean_ci/blob/master/documentation.pdf). It's a bit general, since the idea can be applied to sites other than AOTY.

## Requirements
You must be able to run the `.exe` file provided in the release. If you can't do that, you must download [Rust](https://www.rust-lang.org/tools/install).

## Usage
To calculate the confidence interval for an album, clone this repo, open Terminal/Command Prompt and run the following command in the program's directory:

```cargo run -- --album <album_id>```.

If you downloaded the release, you can instead use the following command:

```mean_ci.exe --album <album_id>```.

So basically, just replace "cargo run --" with the path to the .exe file.

By default, this will calculate a 90% confidence interval for the album's user score. I chose 90% because the lower bound provides a 95% one-tailed confidence interval for the user score. In other words, if the confidence interval is `(a, b)`, we can be 90% confident the user score is between `a` and `b`, and we can be 95% confident that the user score is at least `a` (the difference is that there's no upper bound).

Note that the actual user score is unknown, since it is a weighted average, and the weights for each user are not public. However, the unweighted user score is a very good approximation for the true score.

To calculate a confidence interval that is not 90%, run the following command:

```cargo run -- --album <album_id> --alpha <alpha>```,

where `<alpha>` is the significance level. For a 95% confidence interval, this would be `0.05`.

To calculate a chart for a year, run the following command:

```cargo run -- --chart <year>```,

where `<year>` is the year you'd like to calculate. It should be a number between 1950 and 2024. This will output a chart where the placement of each album is determined by the lower confidence bound of its user score. The chart is sorted in descending order; the album with the highest lower bound is first, and the album with the lowest lower bound is last.

There is a bug that if the number of albums in the chart is a multiple of 25, the program will output albums from another year. For now you have to manually edit the code so that it stops at a specific page number.

It could take a very long time to run a chart. Currently, the delay is set to 200ms. You can edit this to 0ms if you're impatient, but don't lag the site out please.

To calculate a chart for a year with a confidence interval that is not 90%, run the following command:

```cargo run -- --chart <year> --alpha <alpha>```,

where `<alpha>` is the signifiance level.

### Example
To calculate a 95% confidence interval for `JPEGMAFIA - LP! (Offline Version)`, run

```cargo run -- --album 428719 --alpha 0.05```,

which will output the following:

```
428719, "JPEGMAFIA - LP! (Offline Version)"
Number of Ratings: 5318
Mean: 90.42403159082362
95% Confidence Interval: [89.95456439157066, 90.84698643130072]
```

To calculate the 1950 chart with the default 90% confidence interval, run

```cargo run -- --chart 1950```,

```
1, +3, 129813, "Charlie Parker - Charlie Parker With Strings", 70.6867469879518, 65.53829170283431, 74.39317830366917
2, +3, 103576, "Yma Sumac - Voice of the Xtabay", 69.25, 63.47187018537203, 73.67282962136674
3, 0, 89623, "Jo Stafford - Autumn in New York", 70.65789473684211, 62.41901338576798, 75.91069686335243
4, +2, 61600, "Trío Aguilillas - Sones of Mexico", 68.14925373134328, 61.73917125799088, 73.05941004548929
5, -4, 223668, "Benny Goodman - The Famous 1938 Carnegie Hall Jazz Concert", 79.71428571428571, 60.6535599103917, 87.52152624389461
6, +1, 89247, "Ella Fitzgerald - Ella Sings Gershwin", 67.4375, 57.77987877762587, 73.89604761940858
7, +1, 135350, "Aracy De Almeida - Noel Rosa", 67.04166666666667, 55.02639085365262, 75.12826389074911
8, -6, 207844, "Juilliard Percussion Orchestra / New York Wind Ensemble / Frederic Waldman / René Le Roy - Complete Works of Edgard Varese Volume 1", 76.3, 51.627479623108215, 87.47895988462093
9, 0, 207856, "Les Paul - The New Sound!", 66.33333333333333, 50.872596897662525, 76.99343469320247
10, +2, 207861, "Frank Sinatra - Sing and Dance With Frank Sinatra", 61.2, 47.06454564286884, 71.34650282044508
11, 0, 207832, "Dr. Samuel J. Hoffman - Music for Peace of Mind", 62.4, 46.63769893177886, 73.74202710722699
12, -2, 207849, "Stan Kenton - Volume One - "Innovations in Modern Music"", 66.5, 45.227219667260734, 79.29489308896738
```

The output has the following format:

```rank, rank_difference, album_id, album_name, user_score, lower_bound, upper_bound```.

These parameters should be self explanatory, but `rank` is the rank of the album on this new chart, and `rank_difference` is the difference in rank from the chart on the site to this chart.