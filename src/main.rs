mod cards;
mod cli;
mod tui;
mod widgets;

use std::io::{self};

use clap::Parser;
use cli::Args;
use tui::{AppConfig, ArkanaApp};

fn main() -> io::Result<()> {
    let args = Args::parse();
    let mut terminal = tui::init()?;

    let delim = ",";

    // Set Path
    let path = match args.csv {
        Some(path) => path,
        _ => String::from(""),
    };

    // Set has_headers
    let has_headers = match args.no_header.unwrap() {
        true => false,
        false => true,
    };

    let config = AppConfig {
        file_path: String::from(path),
        delimiter: String::from(delim),
        has_headers,
    };

    let app_result = ArkanaApp::default().run(&mut terminal, config);
    tui::restore()?;
    app_result
}
