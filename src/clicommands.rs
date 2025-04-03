use clap::Parser;

#[derive(Parser)]
#[command(name = "grepbutbetter", about = "Search for patterns in files")]
pub struct Cli {
    // search string in files
    #[clap(value_name = "PATTERN")]
    pub find: String,
    
    // search in files, may be multiple
    #[clap(value_name = "FILE", num_args = 1..)]
    pub file: Vec<String>,

    // flag for case insensitive search
    #[clap(short, long, help = "Perform a case-insensitive search")]
    pub ignore_case: bool,

    #[clap(short, long, help = "Colorize the string search")]
    pub color: bool,

    
}

impl Cli {
    pub fn parse() -> Self {
        Cli::parse_from(std::env::args())
    }
}