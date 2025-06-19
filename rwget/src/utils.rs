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
