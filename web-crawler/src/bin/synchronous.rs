use clap::clap_app;
use url::Url;
use web_crawler::{synchronous, url_scheme};

fn main() {
    let args = clap_app!(app =>
        (name: "Web crawler (single-threaded)")
        (version: "0.1.0")
        (about: "Visit input HTTP URL and its hyperlinks.")
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

    let max_depth = args
        .value_of("DEPTH")
        .unwrap_or("1")
        .parse::<u16>()
        .unwrap_or(1);

    synchronous::crawler::crawl(input_url, max_depth);
}
