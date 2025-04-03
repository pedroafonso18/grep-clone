use std::fs;
use std::path::Path;
use colored::Colorize;
mod clicommands;

fn main() {
    let args = clicommands::Cli::parse();

    let mut any_found = false;
    for file in &args.file {
        let path = Path::new(file);
        if !path.is_file() {
            continue;
        }

        let contents = fs::read_to_string(file).unwrap_or_else(|e| {
            eprintln!("{}", format!("Error reading file '{}': {}", file, e).red());
            std::process::exit(1);
        });

        let search_term = if args.ignore_case {
            args.find.to_lowercase()
        } else {
            args.find.clone()
        };

        for (line_number, line) in contents.lines().enumerate() {
            let line_to_search = if args.ignore_case {
                line.to_lowercase()
            } else {
                line.to_string()
            };

            if line_to_search.contains(&search_term) {
                any_found = true;
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
                    println!("{}", format!("Code inside brackets:\n{}", bracket_content).blue());
                }

                if args.color {
                    println!(
                        "The text '{}' was found in file: {}, line {}: {}",
                        args.find.cyan(),
                        path.file_name()
                            .unwrap_or_else(|| std::ffi::OsStr::new("unknown"))
                            .to_string_lossy()
                            .yellow(),
                        (line_number + 1).to_string().red(),
                        line.green()
                    );
                } else {
                    println!(
                        "The text '{}' was found in file: {}, line {}: {}",
                        args.find,
                        path.file_name()
                            .unwrap_or_else(|| std::ffi::OsStr::new("unknown"))
                            .to_string_lossy(),
                        line_number + 1,
                        line
                    );
                }
            }
        }

    }

    if !any_found  {
        println!("{}", format!("The text '{}' was not found in any file.", args.find).red());
    }
}
