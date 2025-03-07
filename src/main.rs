mod cards;
mod cli;
mod tui;
mod widgets;

use std::error::Error;

use clap::Parser;
use cli::Args;
use scopeguard::defer;
use tui::{AppConfig, ArkanaApp};

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    let mut terminal = tui::init()?;

    // Gracefully shut down, even on panic
    defer! {
        tui::restore().unwrap();
    }

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
        has_headers,
    };

    let app_result = ArkanaApp::default().run(&mut terminal, config);

    match &app_result {
        Ok(_) => app_result,
        Err(e) => {
            eprintln!("Error: {}", e);
            app_result
        }
    }
}
