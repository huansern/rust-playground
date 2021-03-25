use std::env;
use url::Url;
use web_crawler::asynchronous;

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
    asynchronous::crawler::crawl(input_url, 4, 1).await;
}
