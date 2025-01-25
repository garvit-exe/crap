# Crap - A Simple Rust-based Grep Tool

**Crap** is a lightweight, fast, and customizable command-line search tool inspired by `grep`. Built with Rust, it leverages the performance and reliability of modern Rust tooling to provide a powerful alternative for text searching in files.

---

## Features

- **Fast and Efficient**: Optimized for performance using Rust's concurrency and memory safety features.
- **Regular Expression Support**: Use advanced regex patterns to search through files.
- **Syntax Highlighting**: Colorized output for better readability (with **termcolor**).
- **Cross-platform**: Designed to run seamlessly on Windows, macOS, and Linux.
- **Easy to Integrate**: Works with file streams or piped input/output.

---

## Installation

### From Source
To build `crap` from source, you must have the **Rust toolchain** installed. Follow these steps:

1. Clone the repository:
   ```bash
   git clone https://github.com/your-repo/crap.git
   cd crap
   ```

2. Build the project:
   ```bash
   cargo build --release
   ```

3. Install the binary to your path:
   ```bash
   cargo install --path .
   ```

---

## Usage

The `crap` tool allows you to search for text patterns in files with ease.

### Basic Syntax
```bash
crap [OPTIONS] <PATTERN> [FILE...]
```

### Examples

1. **Search for a word in a file**:
   ```bash
   crap "error" logs.txt
   ```

2. **Recursive search in a directory**:
   ```bash
   crap -r "TODO" ./src
   ```

3. **Case-insensitive search**:
   ```bash
   crap -i "rust" README.md
   ```

4. **Display line numbers with matches**:
   ```bash
   crap -n "main" src/main.rs
   ```

5. **Pipe input support**:
   ```bash
   cat file.txt | crap "pattern"
   ```

---

## Options

- `-r`, `--recursive`: Search recursively through directories.
- `-i`, `--ignore-case`: Perform a case-insensitive search.
- `-n`, `--line-numbers`: Display line numbers for each match.
- `--color`: Enable syntax highlighting (auto, always, never).
- `--help`: Display help information.
- `--version`: Show the version of `crap`.

---

## Examples in Rust Code

Here's an example of how to use the `regex` and `termcolor` crates to implement basic functionality:

```rust
use regex::Regex;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};
use std::io::Write;

fn search_in_file(file_content: &str, pattern: &str) {
    let re = Regex::new(pattern).unwrap();
    for (line_number, line) in file_content.lines().enumerate() {
        if re.is_match(line) {
            let mut stdout = StandardStream::stdout(ColorChoice::Auto);
            stdout.set_color(ColorSpec::new().set_fg(Some(Color::Green))).unwrap();
            write!(&mut stdout, "{}: ", line_number + 1).unwrap();
            stdout.set_color(ColorSpec::new().set_reset(true)).unwrap();
            writeln!(&mut stdout, "{}", line).unwrap();
        }
    }
}
```

---

## Contribution

Contributions to improve **Crap** are always welcome! Feel free to open issues or submit pull requests.

1. Fork the repository.
2. Create a new branch for your feature/bug fix.
3. Submit a PR describing your changes.

---

## License

This tool is licensed under the **MIT License**. See `LICENSE` for more details.

---

## Acknowledgments

- **[Regex crate](https://docs.rs/regex/)**: For providing regex support.
- **[Termcolor crate](https://docs.rs/termcolor/)**: For colored terminal output.

Enjoy using **crap** for all your text-search needs!