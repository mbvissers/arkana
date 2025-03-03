mod cards;
mod cli;
mod tui;
mod widgets;

use std::io::{self};

use clap::Parser;
use cli::Args;
use tui::ArkanaApp;

fn main() -> io::Result<()> {
    let args = Args::parse();
    let mut terminal = tui::init()?;

    let app_result = match args.csv {
        Some(path) => ArkanaApp::default().run(&mut terminal, path.as_str()),
        _ => ArkanaApp::default().run(&mut terminal, ""),
    };

    tui::restore()?;
    app_result
}
