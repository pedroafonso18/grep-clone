use std::env::current_dir;
use std::fs;
use std::path::Path;
mod clicommands;

fn main() {
    let args = clicommands::Cli::parse();

    if args.list_files {
        match current_dir() {
            Ok(dir) => match fs::read_dir(dir) {
                Ok(entries) => {
                    for entry in entries {
                        if let Ok(entry) = entry {
                            println!("{}", entry.file_name().to_string_lossy());
                        }
                    }
                }
                Err(e) => {
                    eprintln!("Error reading directory: {}", e);
                }
            },
            Err(e) => {
                eprintln!("Error getting current directory: {}", e);
            }
        }
        return;
    }

    for file in &args.file {
        let path = Path::new(file);
        if !path.is_file() {
            continue;
        }

        let contents = fs::read_to_string(file).unwrap_or_else(|e| {
            eprintln!("Error reading file '{}': {}", file, e);
            std::process::exit(1);
        });

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

        if !found {
            println!(
                "The text '{}' was not found in the file: {}.",
                args.find,
                path.file_name()
                    .unwrap_or_else(|| std::ffi::OsStr::new("unknown"))
                    .to_string_lossy()
            );
        }
    }
}
