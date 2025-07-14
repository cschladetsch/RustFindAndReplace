# regex-replace

A fast and efficient command-line tool for performing regex-based search and replace operations across multiple files recursively.

## Features

- 🔍 **Recursive file processing** - Search through directories and subdirectories
- 🎯 **Regex pattern matching** - Full regex support for complex pattern matching
- 📁 **File extension filtering** - Process only specific file types
- 👀 **Dry run mode** - Preview changes without modifying files
- 📊 **Verbose output** - See detailed information about matches and replacements
- ⚡ **Fast performance** - Built with Rust for speed and efficiency
- 🛡️ **Safe operation** - Graceful error handling for unreadable files

## Installation

### From Source

```bash
# Clone the repository
git clone https://github.com/yourusername/regex-replace.git
cd regex-replace

# Build the project
cargo build --release

# The binary will be available at ./target/release/regex-replace
```

### Using Cargo

```bash
cargo install --path .
```

## Usage

```bash
regex-replace [OPTIONS]
```

### Options

- `-p, --pattern <PATTERN>` - Regex pattern to search for (required)
- `-r, --replace <REPLACE>` - Replacement text (required)
- `-d, --directory <DIRECTORY>` - Directory to search in (default: current directory)
- `-e, --extensions <EXTENSIONS>` - File extensions to include (comma-separated, e.g., "txt,rs,js")
- `-n, --dry-run` - Show what would be changed without modifying files
- `-v, --verbose` - Display detailed output including match information
- `-h, --help` - Print help information
- `-V, --version` - Print version information

### Examples

#### Basic replacement
Replace all numbers with "XXX" in the current directory:
```bash
regex-replace -p '\d+' -r 'XXX'
```

#### Process specific file types
Replace "TODO" with "DONE" in only .rs and .txt files:
```bash
regex-replace -p 'TODO' -r 'DONE' -e 'rs,txt'
```

#### Dry run with verbose output
Preview changes without modifying files:
```bash
regex-replace -p 'old_function' -r 'new_function' --dry-run --verbose
```

#### Search in specific directory
Replace email addresses with "[REDACTED]" in the docs folder:
```bash
regex-replace -p '\b[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\.[A-Z|a-z]{2,}\b' -r '[REDACTED]' -d ./docs
```

#### Using capture groups
Replace function declarations using capture groups:
```bash
regex-replace -p 'fn (\w+)\(' -r 'function $1(' -e 'rs'
```

## Regular Expression Syntax

This tool uses Rust's regex crate, which supports:
- Character classes: `[a-z]`, `\d`, `\w`, `\s`
- Quantifiers: `*`, `+`, `?`, `{n,m}`
- Anchors: `^`, `$`, `\b`
- Groups: `(...)`, `(?:...)`, `(?P<name>...)`
- Capture groups in replacements: `$1`, `$2`, etc.

For full regex syntax documentation, see: https://docs.rs/regex/latest/regex/#syntax

## Testing

The project includes comprehensive unit and integration tests:

```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run only unit tests
cargo test --lib

# Run only integration tests
cargo test --test integration_test
```

## Safety Features

- **Non-destructive by default**: Use `--dry-run` to preview changes
- **File validation**: Skips binary files and handles encoding errors gracefully
- **Atomic writes**: Files are written completely or not at all
- **Clear error messages**: Detailed error reporting for debugging

## Performance

The tool is optimized for performance:
- Parallel directory traversal
- Efficient regex compilation (compiled once, used many times)
- Memory-efficient file processing
- Zero-copy replacements where possible

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

### Development Setup

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Run tests (`cargo test`)
4. Commit your changes (`git commit -m 'Add some amazing feature'`)
5. Push to the branch (`git push origin feature/amazing-feature`)
6. Open a Pull Request

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Acknowledgments

- Built with [Rust](https://www.rust-lang.org/)
- Uses [regex](https://crates.io/crates/regex) for pattern matching
- Command-line parsing by [clap](https://crates.io/crates/clap)
- Directory traversal with [walkdir](https://crates.io/crates/walkdir)
- Error handling via [anyhow](https://crates.io/crates/anyhow)