use clap::Parser;

#[derive(Parser)]
#[command(name = env!("CARGO_PKG_NAME"))]
#[command(version = env!("CARGO_PKG_VERSION"))]
#[command(about = "CLI flashcard application", long_about = None)]
pub struct Args {
    #[arg(short = 'f', long, help = "Load cards from a CSV file")]
    pub csv: Option<String>,

    #[arg(
        short = 's',
        long = "semi-colon",
        help = "Set flag if delimiter is semi-colon",
        action = clap::ArgAction::SetTrue,
    )]
    pub semi_colon: Option<bool>,

    #[arg(
        long = "no-header",
        help = "Set flag if CSV has no header row",
        action = clap::ArgAction::SetTrue,
    )]
    pub no_header: Option<bool>,
}
