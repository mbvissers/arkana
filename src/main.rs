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

    match args.csv {
        Some(path) => {
            println!("Loading cards from {}", path);
        }
        _ => {
            println!("Loading cards from default path");
        }
    }

    let mut terminal = tui::init()?;
    let app_result = ArkanaApp::default().run(&mut terminal);

    tui::restore()?;
    app_result
}
