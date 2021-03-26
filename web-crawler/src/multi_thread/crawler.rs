use crate::client::get_blocking_client;
use crate::task::RequestTask;
use reqwest::blocking::Client;
use std::collections::HashSet;
use std::sync::mpsc::{Receiver, Sender};
use std::sync::{mpsc, Arc, Mutex};
use std::{cmp, thread};
use url::Url;

fn start(task: Sender<RequestTask>, done: Receiver<Option<(u16, Vec<Url>)>>, root: Url) {
    let mut task_pending_completion = 0;
    let mut history: HashSet<String> = HashSet::new();
    history.insert(root.as_str().into());
    task.send(RequestTask::new(root, 0)).unwrap();
    task_pending_completion += 1;
    while task_pending_completion > 0 {
        let result = match done.recv() {
            Err(err) => {
                eprintln!("{}", err);
                return;
            }
            Ok(r) => r,
        };
        task_pending_completion -= 1;
        if let Some((depth, urls)) = result {
            for url in urls.into_iter() {
                if history.insert(url.as_str().into()) {
                    task.send(RequestTask::new(url, depth)).unwrap();
                    task_pending_completion += 1;
                }
            }
        }
    }
    println!("Completed all task. Closing task channel...");
}

fn worker(
    id: usize,
    task: Arc<Mutex<Receiver<RequestTask>>>,
    done: Sender<Option<(u16, Vec<Url>)>>,
    client: Client,
    max_depth: u16,
) {
    loop {
        let mut request_task = match task.lock().unwrap().recv() {
            Err(_) => {
                println!("Worker #{}: Task channel closed. Returning...", id);
                break;
            }
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
        println!("Worker #{}:\n{}\n", id, request_task);
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

pub fn crawl(root_url: Url, mut max_concurrent_request: usize, max_depth: u16) {
    max_concurrent_request = cmp::max(max_concurrent_request, 1);
    let blocking_client = get_blocking_client();
    let (task_sender, task_receiver) = mpsc::channel();
    let (done_sender, done_receiver) = mpsc::channel();
    let atomic_task_receiver = Arc::new(Mutex::new(task_receiver));

    let mut handles = vec![];
    for id in 0..max_concurrent_request {
        let task = Arc::clone(&atomic_task_receiver);
        let done = done_sender.clone();
        let client = blocking_client.clone();
        let handle = thread::spawn(move || worker(id + 1, task, done, client, max_depth));
        handles.push(handle);
    }
    drop(blocking_client);
    drop(atomic_task_receiver);
    drop(done_sender);

    start(task_sender, done_receiver, root_url);

    for handle in handles {
        handle.join().unwrap();
    }
}
