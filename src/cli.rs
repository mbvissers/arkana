use clap::Parser;

#[derive(Parser)]
#[command(name = env!("CARGO_PKG_NAME"))]
#[command(version = env!("CARGO_PKG_VERSION"))]
#[command(about = "CLI flashcard application", long_about = None)]
pub struct Args {
    #[arg(short = 'f', long, help = "Load cards from a CSV file")]
    pub csv: Option<String>,

    #[arg(
        short = 'd',
        long = "delimiter",
        help = "Choose delimiter, default ','"
    )]
    pub delim: Option<String>,

    #[arg(
        long = "no-header",
        help = "Set flag if CSV has no header row",
        action = clap::ArgAction::SetTrue,
    )]
    pub no_header: Option<bool>,
}
