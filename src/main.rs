use std::fs;
mod clicommands;

fn main() {
    let args = clicommands::Cli::parse();
    for file in args.file{
    let contents = fs::read_to_string(&file).expect("Error");

    let search_term = if args.ignore_case {
        args.find.to_lowercase()
    } else {
        args.find.clone()
    };

    let mut found = false;
    for (line_number, line) in contents.lines().enumerate() {
        let line_to_search = if args.ignore_case {
            line.to_lowercase()
        } else {
            line.to_string()
        };

        if line_to_search.contains(&search_term) {
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
            println!(
                "The text '{}' was found on file: {}, line {}: {}",
                args.find,
                file.rsplit('/').next().unwrap_or(&file).to_string(),
                line_number + 1,
                line
            );
        }
    }

    if !found {
        println!("The text '{}' was not found in the file: {}.", args.find, file.rsplit('/').next().unwrap_or(&file).to_string());
    }
    }
}