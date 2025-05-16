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

    let path = args.csv.unwrap_or_default();
    let no_headers = args.no_header.unwrap_or(false);

    let config = AppConfig {
        file_path: path,
        has_headers: !no_headers,
    };

    let app_result = ArkanaApp::default().run(&mut terminal, config);

    match &app_result {
        Ok(_) => {
            println!("Exited safely");
            app_result
        }
        Err(e) => {
            eprintln!("Error: {}", e);
            app_result
        }
    }
}
