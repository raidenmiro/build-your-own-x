use crate::Args;
use failure::Fallible;
use url::Url;

pub fn http_download(url: Url, args: &Args) -> Fallible<()> {
    let version = env!("CARGO_PKG_VERSION");

    unreachable!()
}

pub fn ftp_download(url: Url, quiet_mode: bool, filename: &str) -> Fallible<()> {
    unreachable!()
}
