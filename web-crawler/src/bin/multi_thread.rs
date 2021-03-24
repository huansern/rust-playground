use std::env;
use url::Url;
use web_crawler::multi_thread;

fn main() {
    let input = env::args().nth(1).expect("missing input url");
    let input_url = match Url::parse(&input) {
        Err(err) => {
            eprintln!("{}", err);
            std::process::exit(1);
        }
        Ok(url) => url,
    };
    multi_thread::crawler::crawl(input_url, 4, 1);
}
