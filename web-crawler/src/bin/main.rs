use std::env;
use std::error::Error;
use url::Url;
use web_crawler::task::RequestTask;

fn get(url: Url) -> Result<(RequestTask, Vec<Url>), reqwest::Error> {
    let response = reqwest::blocking::get(url.as_str())?;
    let mut task = RequestTask::new(url, 0);
    let urls = task.parse_response(response).unwrap_or_default();
    Ok((task, urls))
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = env::args().nth(1).expect("missing input url");
    let input_url = match Url::parse(&input) {
        Ok(url) => url,
        Err(err) => {
            eprintln!("{}", err.to_string());
            std::process::exit(1);
        }
    };

    let (task, urls) = get(input_url)?;
    println!("{}\n", task);
    for (i, url) in urls.iter().enumerate() {
        println!("Url #{}: {}", i + 1, url.as_str());
    }
    Ok(())
}
