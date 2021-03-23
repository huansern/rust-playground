use crate::html;
use reqwest::blocking::Response;
use select::document::Document;
use std::fmt;
use url::Url;

pub const RESULT_HEADER: &str = " Status | Content Type             | Links | URL";

#[derive(Debug)]
pub struct URLRequestResult {
    pub url: Url,
    pub distance: u16,
    pub status_code: u16,
    pub content_type: String,
    pub url_count: usize,
}

impl URLRequestResult {
    pub fn new(distance: u16, url: Url) -> Self {
        URLRequestResult {
            url,
            distance,
            status_code: 0,
            content_type: "".into(),
            url_count: 0,
        }
    }

    pub fn parse(&mut self, response: Response) -> Option<Vec<Url>> {
        self.status_code = response.status().as_u16();
        match response.headers().get("content-type") {
            None => return None,
            Some(header) => {
                self.content_type = header.to_str().unwrap_or_default().into();
                if !self.content_type.starts_with("text/html") {
                    return None;
                }
            }
        }
        if let Ok(body) = response.text() {
            let urls = html::extract_url(&self.url, Document::from(body.as_str()));
            self.url_count = urls.len();
            Some(urls)
        } else {
            None
        }
    }
}

impl fmt::Display for URLRequestResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:>7} | {:>24} | {:>5} | {}",
            self.status_code, self.content_type, self.url_count, self.url
        )
    }
}
