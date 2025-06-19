use std::time::Duration;

use console::style;
use failure::{Fallible, bail};
use reqwest::header::{self, HeaderMap};
use url::{ParseError, Url};

pub fn parse_url(url: &str) -> Result<Url, ParseError> {
    Url::parse(url).or_else(|err| {
        if err == ParseError::RelativeUrlWithoutBase {
            let url_with_base = format!("{}{}", "https://", url);
            Url::parse(url_with_base.as_str())
        } else {
            Err(err)
        }
    })
}

pub fn retrieve_request_headers(
    url: Url,
    timeout: u64,
    ua: &str,
) -> Result<HeaderMap, reqwest::Error> {
    let client = reqwest::blocking::Client::new();

    let res = client
        .head(url)
        .timeout(Duration::from_secs(timeout))
        .header(header::USER_AGENT, ua)
        .send()?;

    Ok(res.headers().clone())
}
pub fn print_headers(hmap: HeaderMap) {
    for (key, value) in hmap.iter() {
        println!(
            "{}: {}",
            style(key).red(),
            style(value.to_str().unwrap_or("<..>")).green()
        );
    }
}

pub fn throw_err(msg: String) -> Fallible<()> {
    bail!("{}", msg)
}
