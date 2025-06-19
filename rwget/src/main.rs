mod bar;
mod download;
mod utils;

use crate::download::{ftp_download, http_download};
use crate::utils::throw_err;
use clap::Parser;
use clap::{ArgAction, arg};
use failure::Fallible;

#[derive(Parser, Debug)]
#[command(
    name = "rwget",
    author = "Robert Kuzhin",
    version = "0.0.1",
    about = "wget clone written on rust"
)]
struct Args {
    /// URL to download (e.g., 'https://example.com')
    #[arg(short_alias = 'U', long)]
    url: String,

    /// Enable quiet mode (suppresses progress output)
    #[arg(short_alias = 'q', long, alias = "quiet", action = ArgAction::SetTrue)]
    quiet_mode: bool,

    /// Output file path
    #[arg(short_alias = 'O', long, alias = "file")]
    output: String,

    /// View file headers
    #[arg(short, long, action = ArgAction::SetTrue)]
    headers: bool,

    /// Timeout for the request in seconds
    #[arg(short, long, default_value_t = 30)]
    timeout: u64,
}

fn main() -> Fallible<()> {
    let args = Args::parse();

    let url = utils::parse_url(args.url.as_str())?;

    match url.scheme() {
        "http" | "https" => http_download(url, &args),
        "ftp" | "ftps" => ftp_download(url, args.quiet_mode, &args.output),
        _ => throw_err(format!("unsupported scheme: {}", url.scheme())),
    }
}
