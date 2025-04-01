use clap::Parser;

#[derive(Parser)]
pub struct Cli {
    pub find: String,
    pub file: String,
    #[clap(short, long, help = "Perform a case-insensitive search")]
    pub ignore_case: bool,
}

impl Cli {
    pub fn parse() -> Self {
        Cli::parse_from(std::env::args())
    }
}