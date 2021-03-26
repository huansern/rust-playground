use url::Url;

pub fn is_http(url: &Url) -> bool {
    url.scheme().starts_with("http")
}

pub fn expect_http(url: &Url) {
    if !is_http(url) {
        eprintln!("only http(s) url is supported");
        std::process::exit(1);
    }
}
