use clap::Parser;

#[derive(Parser)]
#[command(name = env!("CARGO_PKG_NAME"))]
#[command(version = env!("CARGO_PKG_VERSION"))]
#[command(about = "CLI flashcard application", long_about = None)]
pub struct Args {
    #[arg(short = 'f', long, help = "Load cards from a CSV file")]
    pub csv: Option<String>,
    // TODO: Add delimiter option for custom CSVs
    // #[arg(
    //     short = 'd',
    //     long = "Delimiter",
    //     help = "Choose delimiter, default ','"
    //
    // )]
    // pub delim: String,
}
