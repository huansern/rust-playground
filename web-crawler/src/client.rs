use reqwest::blocking::Client as BlockingClient;
use reqwest::Client as AsyncClient;
use std::time::Duration;

const CONNECT_TIMEOUT: u64 = 10;
const USER_AGENT: &str = "Mozilla/5.0 (X11; Linux x86_64; rv:87.0) Gecko/20100101 Firefox/87.0";

pub fn get_blocking_client() -> BlockingClient {
    BlockingClient::builder()
        .user_agent(USER_AGENT)
        .connect_timeout(Duration::new(CONNECT_TIMEOUT, 0))
        .gzip(true)
        .build()
        .unwrap()
}

pub fn get_async_client() -> AsyncClient {
    AsyncClient::builder()
        .user_agent(USER_AGENT)
        .connect_timeout(Duration::new(CONNECT_TIMEOUT, 0))
        .gzip(true)
        .build()
        .unwrap()
}
