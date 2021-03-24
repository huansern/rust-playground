use crate::task::RequestTask;
use reqwest::blocking::Client;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::thread;
use url::Url;

fn start(task: Sender<RequestTask>, done: Receiver<Option<(u16, Vec<Url>)>>, root: Url) {
    let mut total_task = 0;
    let mut completed_task = 0;
    task.send(RequestTask::new(root, 0)).unwrap();
    total_task += 1;
    loop {
        let result = match done.recv() {
            Err(_) => break,
            Ok(r) => r,
        };
        completed_task += 1;
        if let Some((depth, urls)) = result {
            for url in urls.into_iter() {
                task.send(RequestTask::new(url, depth)).unwrap();
                total_task += 1;
            }
        } else if completed_task == total_task {
            break;
        }
    }
}

fn worker(
    task: Arc<Mutex<Receiver<RequestTask>>>,
    done: Sender<Option<(u16, Vec<Url>)>>,
    client: Client,
    max_depth: u16,
) {
    loop {
        let mut request_task = match task.lock().unwrap().recv() {
            Err(_) => break,
            Ok(task) => task,
        };
        let response = match client.get(request_task.url.as_str()).send() {
            Err(_) => {
                done.send(None).unwrap();
                continue;
            }
            Ok(r) => r,
        };
        let urls = request_task.parse_response(response);
        println!("{}", request_task);
        if request_task.depth < max_depth {
            match urls {
                Some(urls) if urls.len() > 0 => {
                    done.send(Some((request_task.depth + 1, urls))).unwrap();
                    continue;
                }
                _ => {}
            }
        }
        done.send(None).unwrap();
    }
}

pub fn crawl(root_url: Url, mut threads: usize, max_depth: u16) {
    if threads < 1 {
        threads = 1;
    }
    let (task_sender, task_receiver) = channel();
    let (done_sender, done_receiver) = channel();
    let atomic_task_receiver = Arc::new(Mutex::new(task_receiver));

    let mut handles = vec![];
    for _ in 0..threads {
        let task = Arc::clone(&atomic_task_receiver);
        let done = done_sender.clone();
        let client = Client::builder().build().unwrap();
        let handle = thread::spawn(move || worker(task, done, client, max_depth));
        handles.push(handle);
    }
    drop(atomic_task_receiver);
    drop(done_sender);

    start(task_sender, done_receiver, root_url);

    for handle in handles {
        handle.join().unwrap();
    }
}
