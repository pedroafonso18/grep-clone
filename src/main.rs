use std::fs;
use clap::Parser;

#[derive(Parser)]
#[command(name = "grepbutbetter", version="1.0", about="grep is good, this is better")]
struct Cli {
    find: String,
}
fn main() {
    let contents = fs::read_to_string("poem.txt").expect("Error");
    let args = Cli::parse();
    if contents.contains(&args.find) {
        println!("The text '{}' was found in the file.", args.find);
    } else {
        println!("The text '{}' was not found in the file.", args.find);
    }
}
