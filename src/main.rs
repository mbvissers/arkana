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
    let header = true;

    // Set Path
    let path = match args.csv {
        Some(path) => path,
        _ => String::from(""),
    };

    let config = AppConfig {
        file_path: Some(String::from(path)),
        delimiter: Some(String::from(delim)),
        header: Some(header),
    };

    let app_result = ArkanaApp::default().run(&mut terminal, config);
    tui::restore()?;
    app_result
}
