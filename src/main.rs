use std::fs;
mod clicommands;

fn main() {
    let args = clicommands::Cli::parse();
    let contents = fs::read_to_string(&args.file).expect("Error");
    if contents.contains(&args.find) {
        println!("The text '{}' was found in the file.", args.find);
    } else {
        println!("The text '{}' was not found in the file.", args.find);
    }
}
