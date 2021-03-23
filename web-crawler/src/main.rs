use crate::result::{URLRequestResult, RESULT_HEADER};
use std::env;
use std::error::Error;
use url::Url;

mod html;
mod result;

fn get(url: Url) -> Result<(URLRequestResult, Vec<Url>), reqwest::Error> {
    let response = reqwest::blocking::get(url.as_str())?;
    let mut request_result = URLRequestResult::new(0, url);
    let urls = request_result.parse(response).unwrap_or_default();
    Ok((request_result, urls))
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = env::args().nth(1).expect("missing input url");
    println!("Input url: {}", input);

    let input_url = match Url::parse(&input) {
        Ok(url) => url,
        Err(err) => {
            eprintln!("{}", err.to_string());
            std::process::exit(1);
        }
    };

    let (request_result, urls) = get(input_url)?;
    println!("{}", RESULT_HEADER);
    println!("{}", request_result);
    for (i, url) in urls.iter().enumerate() {
        println!("Url #{}: {}", i + 1, url.as_str());
    }
    Ok(())
}
