use select::document::Document;
use select::predicate::Name;
use url::{ParseError, Url};

pub fn extract_url(from: &Url, document: Document) -> Vec<Url> {
    document
        .find(Name("a"))
        .into_iter()
        .filter_map(|node| node.attr("href"))
        .filter_map(|href| parse(from, href))
        .collect::<Vec<Url>>()
}

fn parse(base: &Url, href: &str) -> Option<Url> {
    match Url::parse(href) {
        Ok(url) => Some(url),
        Err(ParseError::RelativeUrlWithoutBase) => try_join(base.clone(), href),
        _ => None,
    }
}

fn try_join(base: Url, href: &str) -> Option<Url> {
    base.join(href).ok()
}
