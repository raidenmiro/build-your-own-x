use indicatif::{ProgressBar, ProgressStyle};

static BAR_FMT: &'static str = "{msg} {spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} eta: {eta}";

pub fn create_progress_bar(quiet_mode: bool, msg: String, length: Option<u32>) -> ProgressBar {
    let bar = match quiet_mode {
        true => ProgressBar::hidden(),
        false => ProgressBar::new(100),
    };

    bar.set_message(msg);

    match length.is_some() {
        true => bar.set_style(
            ProgressStyle::default_bar()
                .template(BAR_FMT)
                .unwrap()
                .progress_chars("=> "),
        ),
        false => bar.set_style(ProgressStyle::default_spinner()),
    };

    bar
}
