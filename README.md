# grepbutbetter

`grepbutbetter` is a command-line tool written in Rust that allows you to search for text within files, with additional features like case-insensitive search and extracting code blocks enclosed in curly brackets.

## Features

- Search for a specific text in a file.
- Case-insensitive search using the `--ignore-case` or `-i` flag.
- Extract and display code blocks enclosed in `{}` when a match is found.
- Colorize the output using the `--color` or `-c` flag.

## Installation

1. Clone the repository:
   ```bash
   git clone <repository-url>
   cd grepbutbetter
   ```

2. Build the project in release mode:
   ```bash
   cargo build --release
   ```

3. Add the binary to your PATH:
   ```bash
   export PATH="/grepbutbetter/target/release:$PATH"
   ```

4. Verify the installation:
   ```bash
   grepbutbetter --help
   ```

## Usage

Run the tool with the following syntax:
```bash
grepbutbetter <text-to-find> <file-path> [OPTIONS]
```

### Example

Search for the word `example` in `file.txt`:
```bash
grepbutbetter example file.txt
```

Search for the word in all the files:

```bash
grepbutbetter example *
```

Perform a case-insensitive search:
```bash
grepbutbetter example file.txt --ignore-case
```

Search for the word `example` in `file.txt` with colored output:
```bash
grepbutbetter example file.txt --color
```

### Output

- If the text is found, it displays the line number and the matching line.
- If the text is inside a code block (enclosed in `{}`), it extracts and displays the entire block.

## Dependencies

This project uses the following dependencies:
- [`clap`](https://crates.io/crates/clap): For parsing command-line arguments.

## License

This project is licensed under the MIT License. See the `LICENSE` file for details.

## Contributing

Contributions are welcome! Feel free to open issues or submit pull requests.

## Acknowledgments

This project was built using Rust and inspired by the functionality of the `grep` command-line tool.
