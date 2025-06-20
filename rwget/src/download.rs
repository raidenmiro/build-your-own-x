use crate::{
    Args,
    utils::{print_headers, retrieve_request_headers},
};
use failure::Fallible;
use url::Url;

pub fn http_download(url: Url, args: &Args) -> Fallible<()> {
    let version = env!("CARGO_PKG_VERSION");
    let user_agent = format!("rwget/{}", version);

    let headers = retrieve_request_headers(url, args.timeout, &user_agent)?;

    if args.headers {
        print_headers(headers);
        return Ok(());
    }

    unreachable!()
}

pub fn ftp_download(url: Url, quiet_mode: bool, filename: &str) -> Fallible<()> {
    unreachable!()
}
