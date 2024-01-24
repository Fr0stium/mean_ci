mod evaluate;
mod websites;

use clap::Parser;

#[derive(Parser, Debug)]
#[clap(group(
    clap::ArgGroup::new("music_type")
        .required(true)
        .args(&["album", "chart"]),
))]
struct Args {
    #[clap(long)]
    album: Option<i32>,

    #[clap(long)]
    chart: Option<i32>,

    #[clap(long, default_value_t = 0.1)]
    alpha: f64,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    if let Some(album) = args.album {
        websites::aoty::output_album_ci(album, args.alpha).await;
    } else if let Some(chart) = args.chart {
        websites::aoty::output_chart_rankings(chart, args.alpha).await;
    }
}
