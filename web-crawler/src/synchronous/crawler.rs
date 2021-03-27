use crate::client::get_blocking_client;
use crate::task::RequestTask;
use reqwest::blocking::Client;
use std::collections::{HashSet, VecDeque};
use url::Url;

struct Crawler {
    http_client: Client,
    max_depth: u16,
    queue: VecDeque<RequestTask>,
    history: HashSet<String>,
}

impl Crawler {
    fn new(http_client: Client, max_depth: u16) -> Self {
        Crawler {
            http_client,
            max_depth,
            queue: VecDeque::new(),
            history: HashSet::new(),
        }
    }

    fn add(&mut self, url: Url, depth: u16) {
        if self.history.insert(url.as_str().into()) {
            self.queue.push_back(RequestTask::new(url, depth));
        }
    }

    fn send_request(&self, task: &mut RequestTask) -> Option<Vec<Url>> {
        match self.http_client.get(task.url.as_str()).send() {
            Err(err) => {
                eprintln!("{}", err);
                None
            }
            Ok(response) => {
                let urls = task.parse_response(response);
                println!("{}\n", task);
                urls
            }
        }
    }

    fn start(mut self, root_url: Url) {
        println!("Crawling begin from {}", root_url.as_str());
        self.add(root_url, 0);
        while let Some(mut task) = self.queue.pop_front() {
            let urls = match self.send_request(&mut task) {
                None => continue,
                Some(urls) => urls,
            };
            if task.depth < self.max_depth {
                for url in urls {
                    self.add(url, task.depth + 1);
                }
            }
        }
        println!("Done.");
    }
}

pub fn crawl(root_url: Url, max_depth: u16) {
    let crawler = Crawler::new(get_blocking_client(), max_depth);
    crawler.start(root_url);
}
