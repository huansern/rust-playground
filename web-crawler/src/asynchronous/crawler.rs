use crate::client::get_async_client;
use crate::task::RequestTask;
use reqwest::Client;
use std::cmp;
use tokio::sync::mpsc;
use tokio::sync::mpsc::{Receiver, Sender, UnboundedSender};
use url::Url;

#[derive(Debug)]
struct Token {}

async fn worker(
    client: Client,
    mut task: RequestTask,
    done: Sender<(u16, Option<Vec<Url>>)>,
    token_return_channel: Sender<Token>,
    token: Token,
) {
    let response = match client.get(task.url.as_str()).send().await {
        Err(err) => {
            eprintln!("{}", err);
            return;
        }
        Ok(response) => response,
    };
    let urls = task.async_parse_response(response).await;
    println!("{}\n", task);
    done.send((task.depth, urls)).await.unwrap();
    token_return_channel.send(token).await.unwrap();
}

async fn start(
    mut done: Receiver<(u16, Option<Vec<Url>>)>,
    task: UnboundedSender<RequestTask>,
    root: Url,
    max_depth: u16,
) {
    let mut pending_job: u16 = 0;
    task.send(RequestTask::new(root, 0)).unwrap();
    pending_job += 1;
    while pending_job > 0 {
        let (depth, urls) = match done.recv().await {
            None => break,
            Some(result) => result,
        };
        pending_job -= 1;
        if depth < max_depth {
            if let Some(urls) = urls {
                for url in urls {
                    task.send(RequestTask::new(url, depth + 1)).unwrap();
                    pending_job += 1;
                }
            }
        }
    }
}

pub async fn crawl(root_url: Url, mut max_concurrent_request: usize, max_depth: u16) {
    max_concurrent_request = cmp::max(max_concurrent_request, 1);
    let client = get_async_client();
    let (token_sender, mut token_receiver) = mpsc::channel(max_concurrent_request);
    let (task_sender, mut task_receiver) = mpsc::unbounded_channel();
    let (done_sender, done_receiver) = mpsc::channel(max_concurrent_request);

    for _ in 0..max_concurrent_request {
        token_sender.send(Token {}).await.unwrap();
    }
    tokio::spawn(async move {
        start(done_receiver, task_sender, root_url, max_depth).await;
    });
    while let Some(task) = task_receiver.recv().await {
        let token = match token_receiver.recv().await {
            None => break,
            Some(token) => token,
        };
        let client_clone = client.clone();
        let done = done_sender.clone();
        let token_return_channel = token_sender.clone();
        tokio::spawn(async move {
            worker(client_clone, task, done, token_return_channel, token).await;
        });
    }
}
