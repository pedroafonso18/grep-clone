use std::fs;
mod clicommands;

fn main() {
    let args = clicommands::Cli::parse();
    let contents = fs::read_to_string(&args.file).expect("Error");

    let mut found = false;
    for (line_number, line) in contents.lines().enumerate() {
        if line.contains(&args.find) {
            println!(
                "The text '{}' was found on line {}: {}",
                args.find,
                line_number + 1,
                line
            );
            found = true;
            if line.contains("{") && !line.contains("}") {
                let mut bracket_content = String::new();
                bracket_content.push_str(line);
                for next_line in contents.lines().skip(line_number + 1) {
                    bracket_content.push('\n');
                    bracket_content.push_str(next_line);
                    if next_line.contains("}") {
                        break;
                    }
                }
                println!("Code inside brackets:\n{}", bracket_content);
            }
        }
    }

    if !found {
        println!("The text '{}' was not found in the file.", args.find);
    }
}