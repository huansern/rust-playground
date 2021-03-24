use crate::client::get_blocking_client;
use crate::task::RequestTask;
use reqwest::blocking::Client;
use url::Url;

struct Tasks {
    list: Vec<RequestTask>,
    completed: usize,
}

impl Tasks {
    fn new(root: RequestTask) -> Self {
        Tasks {
            list: vec![root],
            completed: 0,
        }
    }

    fn next(&mut self) -> Option<&mut RequestTask> {
        let mut next = None;
        if self.completed < self.list.len() {
            next = self.list.get_mut(self.completed);
            self.completed += 1;
        }
        next
    }

    fn add(&mut self, urls: Vec<Url>, depth: u16) {
        let mut tasks = urls
            .into_iter()
            .map(|url| RequestTask::new(url, depth))
            .collect::<Vec<RequestTask>>();
        self.list.append(&mut tasks);
    }
}

fn get(client: &Client, task: &mut RequestTask) -> Result<Option<Vec<Url>>, reqwest::Error> {
    let response = client.get(task.url.as_str()).send()?;
    Ok(task.parse_response(response))
}

pub fn crawl(root_url: Url, max_depth: u16) {
    let client = get_blocking_client();
    let mut tasks = Tasks::new(RequestTask::new(root_url, 0));
    loop {
        let task = match tasks.next() {
            None => break,
            Some(task) => task,
        };
        let urls = match get(&client, task) {
            Err(err) => {
                eprintln!("{}", err);
                continue;
            }
            Ok(urls) => urls,
        };
        println!("{}\n", task);
        if task.depth < max_depth {
            if let Some(urls) = urls {
                let depth = task.depth + 1;
                tasks.add(urls, depth);
            }
        }
    }
}
