use clap::clap_app;
use url::Url;
use web_crawler::{asynchronous, url_scheme};

#[tokio::main]
pub async fn main() {
    let args = clap_app!(app =>
        (name: "Web crawler (asynchronous)")
        (version: "0.1.0")
        (about: "Asynchronously visit input HTTP URL and its hyperlinks.")
        (@arg WORKER: -w --worker [WORKER] +takes_value "Number of workers")
        (@arg DEPTH: -d --depth [DEPTH] +takes_value "Maximum depth of the hyperlink relative to the input URL")
        (@arg URL: +required "First URL to visit")
    ).get_matches();

    let input_url = match Url::parse(args.value_of("URL").unwrap()) {
        Err(err) => {
            eprintln!("Invalid URL: {}", err);
            std::process::exit(1);
        }
        Ok(url) => url,
    };
    url_scheme::expect_http(&input_url);

    let max_concurrent_request = args
        .value_of("WORKER")
        .unwrap_or("4")
        .parse::<usize>()
        .unwrap_or(4);

    let max_depth = args
        .value_of("DEPTH")
        .unwrap_or("1")
        .parse::<u16>()
        .unwrap_or(1);

    asynchronous::crawler::crawl(input_url, max_concurrent_request, max_depth).await;
}
