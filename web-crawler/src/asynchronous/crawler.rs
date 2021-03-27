use crate::client::get_async_client;
use crate::task::RequestTask;
use reqwest::Client;
use std::cmp;
use std::collections::HashSet;
use tokio::sync::mpsc;
use tokio::sync::mpsc::{Receiver, Sender, UnboundedReceiver, UnboundedSender};
use url::Url;

struct Crawler {
    max_depth: u16,
    pending_tasks: usize,
    history: HashSet<String>,
    task_channel: UnboundedSender<RequestTask>,
    result_channel: Receiver<Option<(Vec<Url>, u16)>>,
}

impl Crawler {
    fn new(
        max_depth: u16,
        task_channel: UnboundedSender<RequestTask>,
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

    async fn start(mut self, root_url: Url) {
        println!("Crawling begin from {}", root_url.as_str());
        self.add(root_url, 0);
        while !self.done() {
            let result = match self.result_channel.recv().await {
                None => panic!("Result channel closed before all task result is received."),
                Some(result) => result,
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
        println!("Done.");
    }
}

#[derive(Clone, Debug)]
struct Worker {
    http_client: Client,
    result_channel: Sender<Option<(Vec<Url>, u16)>>,
}

impl Worker {
    fn new(http_client: Client, result_channel: Sender<Option<(Vec<Url>, u16)>>) -> Self {
        Worker {
            http_client,
            result_channel,
        }
    }

    async fn send_request(&self, task: &mut RequestTask) -> Option<Vec<Url>> {
        match self.http_client.get(task.url.as_str()).send().await {
            Err(err) => {
                eprintln!("{}", err);
                None
            }
            Ok(response) => {
                let urls = task.async_parse_response(response).await;
                println!("{}\n", task);
                urls
            }
        }
    }

    async fn work(self, mut task: RequestTask, return_channel: Sender<Worker>) {
        match self.send_request(&mut task).await {
            None => self.result_channel.send(None).await.unwrap(),
            Some(urls) => self
                .result_channel
                .send(Some((urls, task.depth)))
                .await
                .unwrap(),
        }
        return_channel.send(self).await.unwrap();
    }
}

struct WorkerDispatcher {
    max_concurrent_worker: usize,
    sender: Sender<Worker>,
    receiver: Receiver<Worker>,
    task_channel: UnboundedReceiver<RequestTask>,
}

impl WorkerDispatcher {
    fn new(max_concurrent_worker: usize, task_channel: UnboundedReceiver<RequestTask>) -> Self {
        let (sender, receiver) = mpsc::channel(max_concurrent_worker);
        WorkerDispatcher {
            max_concurrent_worker,
            sender,
            receiver,
            task_channel,
        }
    }

    async fn spawn_worker(&self, worker: Worker) {
        for _ in 0..self.max_concurrent_worker {
            self.sender.send(worker.clone()).await.unwrap();
        }
    }

    fn dispatch(&self, worker: Worker, task: RequestTask) {
        let return_channel = self.sender.clone();
        tokio::spawn(async move {
            worker.work(task, return_channel).await;
        });
    }

    async fn run(&mut self) {
        while let Some(task) = self.task_channel.recv().await {
            let worker = match self.receiver.recv().await {
                None => break,
                Some(worker) => worker,
            };
            self.dispatch(worker, task);
        }
    }
}

pub async fn crawl(root_url: Url, mut max_concurrent_request: usize, max_depth: u16) {
    max_concurrent_request = cmp::max(max_concurrent_request, 1);
    let (task_sender, task_receiver) = mpsc::unbounded_channel();
    let (result_sender, result_receiver) = mpsc::channel(max_concurrent_request);

    let mut dispatcher = WorkerDispatcher::new(max_concurrent_request, task_receiver);
    dispatcher
        .spawn_worker(Worker::new(get_async_client(), result_sender))
        .await;
    tokio::spawn(async move {
        dispatcher.run().await;
    });

    let crawler = Crawler::new(max_depth, task_sender, result_receiver);
    crawler.start(root_url).await;
}
