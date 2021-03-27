use crate::client::get_blocking_client;
use crate::task::RequestTask;
use reqwest::blocking::Client;
use std::collections::HashSet;
use std::sync::mpsc::{Receiver, Sender};
use std::sync::{mpsc, Arc, Mutex};
use std::thread::JoinHandle;
use std::{cmp, thread};
use url::Url;

struct Crawler {
    max_depth: u16,
    pending_tasks: usize,
    history: HashSet<String>,
    task_channel: Sender<RequestTask>,
    result_channel: Receiver<Option<(Vec<Url>, u16)>>,
}

impl Crawler {
    fn new(
        max_depth: u16,
        task_channel: Sender<RequestTask>,
        result_channel: Receiver<Option<(Vec<Url>, u16)>>,
    ) -> Self {
        Crawler {
            max_depth,
            pending_tasks: 0,
            history: HashSet::new(),
            task_channel,
            result_channel,
        }
    }

    fn add(&mut self, url: Url, depth: u16) {
        if self.history.insert(url.as_str().into()) {
            self.task_channel
                .send(RequestTask::new(url, depth))
                .unwrap();
            self.pending_tasks += 1;
        }
    }

    fn done(&self) -> bool {
        self.pending_tasks == 0
    }

    fn start(mut self, root_url: Url) {
        println!("Crawling begin from {}", root_url.as_str());
        self.add(root_url, 0);
        while !self.done() {
            let result = match self.result_channel.recv() {
                Err(_) => panic!("Result channel closed before all task result is received."),
                Ok(result) => result,
            };
            self.pending_tasks -= 1;
            if let Some((urls, mut depth)) = result {
                if depth < self.max_depth {
                    depth += 1;
                    for url in urls {
                        self.add(url, depth);
                    }
                }
            }
        }
        println!("Completed all task. Closing task channel...");
    }
}

struct Worker {
    id: usize,
    http_client: Client,
    task_channel: Arc<Mutex<Receiver<RequestTask>>>,
    result_channel: Sender<Option<(Vec<Url>, u16)>>,
}

impl Worker {
    fn new(
        id: usize,
        http_client: Client,
        task_channel: Arc<Mutex<Receiver<RequestTask>>>,
        result_channel: Sender<Option<(Vec<Url>, u16)>>,
    ) -> Self {
        Worker {
            id,
            http_client,
            task_channel,
            result_channel,
        }
    }

    fn start(self) -> JoinHandle<()> {
        thread::spawn(move || self.work())
    }

    fn work(self) {
        loop {
            let mut task = match self.task_channel.lock().unwrap().recv() {
                Err(_) => {
                    println!("Worker #{}: Task channel closed. Returning...", self.id);
                    break;
                }
                Ok(task) => task,
            };
            match self.send_request(&mut task) {
                None => self.result_channel.send(None).unwrap(),
                Some(urls) => self.result_channel.send(Some((urls, task.depth))).unwrap(),
            }
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
                println!("Worker #{}:\n{}\n", self.id, task);
                urls
            }
        }
    }
}

struct WorkerPool {
    max_concurrent_worker: usize,
    handles: Vec<JoinHandle<()>>,
}

impl WorkerPool {
    fn new(max_concurrent_worker: usize) -> Self {
        WorkerPool {
            max_concurrent_worker,
            handles: Vec::new(),
        }
    }

    fn spawn_worker(
        &mut self,
        http_client: Client,
        task_channel: Receiver<RequestTask>,
        result_channel: Sender<Option<(Vec<Url>, u16)>>,
    ) {
        let atomic_task_channel = Arc::new(Mutex::new(task_channel));
        for id in 0..self.max_concurrent_worker {
            let worker = Worker::new(
                id,
                http_client.clone(),
                Arc::clone(&atomic_task_channel),
                result_channel.clone(),
            );
            self.handles.push(worker.start());
        }
    }

    fn join(self) {
        for handle in self.handles {
            handle.join().unwrap();
        }
    }
}

pub fn crawl(root_url: Url, mut max_concurrent_request: usize, max_depth: u16) {
    max_concurrent_request = cmp::max(max_concurrent_request, 1);
    let (task_sender, task_receiver) = mpsc::channel();
    let (result_sender, result_receiver) = mpsc::channel();

    let mut workers = WorkerPool::new(max_concurrent_request);
    workers.spawn_worker(get_blocking_client(), task_receiver, result_sender);

    let crawler = Crawler::new(max_depth, task_sender, result_receiver);
    crawler.start(root_url);

    workers.join();
}
