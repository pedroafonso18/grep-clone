use clap::Parser;

#[derive(Parser)]
pub struct Cli {
    pub find: String,
    pub file: String,
}

impl Cli {
    pub fn parse() -> Self {
        Cli::parse_from(std::env::args())
    }
}