use std::env;
use url::Url;
use web_crawler::{asynchronous, url_scheme};

#[tokio::main]
pub async fn main() {
    let input = env::args().nth(1).expect("missing input url");
    let input_url = match Url::parse(&input) {
        Err(err) => {
            eprintln!("{}", err);
            std::process::exit(1);
        }
        Ok(url) => url,
    };
    url_scheme::expect_http(&input_url);
    asynchronous::crawler::crawl(input_url, 4, 1).await;
}
