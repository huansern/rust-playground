use crate::html;
use reqwest::blocking;
use select::document::Document;
use std::fmt;
use url::Url;

const CONTENT_TYPE: &str = "content-type";
const TEXT_HTML: &str = "text/html";

#[derive(Debug)]
pub struct RequestTask {
    pub url: Url,
    pub depth: u16,
    result: TaskResult,
}

#[derive(Debug, Default)]
struct TaskResult {
    status: u16,
    content_type: String,
    links: u16,
}

impl RequestTask {
    pub fn new(url: Url, depth: u16) -> Self {
        RequestTask {
            url,
            depth,
            result: Default::default(),
        }
    }

    pub fn parse_response(&mut self, response: blocking::Response) -> Option<Vec<Url>> {
        self.result.status = response.status().as_u16();
        match response.headers().get(CONTENT_TYPE) {
            None => {}
            Some(header) => {
                self.result.content_type = header.to_str().unwrap_or_default().into();
            }
        }
        if !self.result.content_type.starts_with(TEXT_HTML) {
            return None;
        }
        if let Ok(html_body) = response.text() {
            let urls = html::extract_url(&self.url, Document::from(html_body.as_str()));
            self.result.links = urls.len() as u16;
            Some(urls)
        } else {
            None
        }
    }

    pub async fn async_parse_response(&mut self, response: reqwest::Response) -> Option<Vec<Url>> {
        self.result.status = response.status().as_u16();
        match response.headers().get(CONTENT_TYPE) {
            None => {}
            Some(header) => {
                self.result.content_type = header.to_str().unwrap_or_default().into();
            }
        }
        if !self.result.content_type.starts_with(TEXT_HTML) {
            return None;
        }
        if let Ok(html_body) = response.text().await {
            let urls = html::extract_url(&self.url, Document::from(html_body.as_str()));
            self.result.links = urls.len() as u16;
            Some(urls)
        } else {
            None
        }
    }
}

impl fmt::Display for RequestTask {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "URL: {}\nDepth:{:>3}, Links:{:>4}, Status:{:>4}, Content type: {}",
            self.url, self.depth, self.result.links, self.result.status, self.result.content_type
        )
    }
}
