mod bar;
mod download;
mod utils;

use crate::download::http_download;
use clap::Parser;
use clap::{ArgAction, arg};
use failure::{Fallible, bail};

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

    /// Output file path (default: current directory)
    #[arg(short_alias = 'O', long, alias = "file")]
    output: String,
}

fn main() -> Fallible<()> {
    let args = Args::parse();

    let url = utils::parse_url(args.url.as_str())?;

    match url.scheme() {
        "http" | "https" => http_download(url, &args),
        "ftp" | "ftps" => unimplemented!(),
        _ => bail!(format!("unsupported scheme {}", url.scheme())),
    }
}
