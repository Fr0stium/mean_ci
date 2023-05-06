# mean_ci
Calculates a two-tailed confidence interval for the mean of any dataset whose bounds are **known** and **finite**, but whose probability distribution is unknown. The data could be of any sample size and could come from any bounded probability distribution. The confidence interval is always in between the dataset's support and is more accurate than the normal approximations, especially for data with a small sample size or small sample variance.

An important assumption is that each data point comes from the same distribution; that is, every data point is independent and identically distributed.

If the distribution is known, then this program will likely produce less accurate outputs. However, the output may still be useful since many distributions are not closed under convolution. For example, the sum of i.i.d. Beta random variables does not have a closed form, so an exact confidence interval for the mean of a sample coming from a Beta distribution would be difficult to compute.

## Documentation
The PDF file containing all the math is on GitHub [here](https://github.com/Fr0stium/mean_ci/blob/aa1d3b1964399f5f3f857edf1f12e8c5c4b93815/documentation.pdf).

## Requirements
You must be able to run the `.exe` file provided in the release. If you can't do that, you must download [Rust](https://www.rust-lang.org/tools/install).

## Usage
If you just downloaded the executable, run the program in the command prompt by dragging the executable in the command prompt, and following it by ` path alpha min_support max_support`.

If you don't have Windows, you can download Rust, `cd` into the program's directory, and run the program with `cargo run -- path alpha min_support max_support`.

* `path` is the path of a `.txt` file that only contains numbers separated by commas. The path should not contain spaces!
* `alpha` is the level of significance. It must be greater than `0` and less than `1`.
* `min_support` is the minimum possible value the dataset could take.
* `max_support` is the maximum possible value the dataset could take.

### Example
Suppose a product has ratings that can range from 0 stars to 5 stars. We collect the product's ratings and place them into a text file like so:
```
4.5,2.5,4,3.5,4.5,5,4.5,3.5,4.5,4,3,5,4.5,4,5,3,4,4,3,4,3,3,3.5,4,4,4,4,4.5,5,3.5,4,4.5,4,3.5,3,1,4,4.5,4,3,3,5,3,4.5,4,5,4.5,4,3,3,2.5,3.5,4.5,5,5,4,4,4,4,4,4,4,5,3.5,3,1.5,4,4,4.5,4,4
```
Let's say the path of my text file is `C:\Users\iifro\OneDrive\Documents\data.txt`, and let's say I downloaded the executable into my downloads folder. Suppose we wanted a 95% confidence interval for the mean of the product ratings. Then I would run the program like so:
```
C:\Users\iifro\Downloads\mean_ci.exe C:\Users\iifro\OneDrive\Documents\data.txt 0.05 0 5
```
The program will then output
```
Number of Ratings: 71
Mean: 3.8732394366197185
95% Confidence Interval: (3.478021162894903, 4.133960590388318)
```
