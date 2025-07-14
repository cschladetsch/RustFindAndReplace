# RustFindAndReplace

[![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![Build Status](https://img.shields.io/github/actions/workflow/status/yourusername/RustFindAndReplace/ci.yml?branch=master&style=for-the-badge)](https://github.com/yourusername/RustFindAndReplace/actions)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg?style=for-the-badge)](https://opensource.org/licenses/MIT)
[![Crates.io](https://img.shields.io/crates/v/regex-replace.svg?style=for-the-badge)](https://crates.io/crates/regex-replace)
[![Downloads](https://img.shields.io/crates/d/regex-replace.svg?style=for-the-badge)](https://crates.io/crates/regex-replace)
[![Rust Version](https://img.shields.io/badge/rust-1.70%2B-blue.svg?style=for-the-badge)](https://www.rust-lang.org/)
[![Platform](https://img.shields.io/badge/platform-linux%20%7C%20macos%20%7C%20windows-lightgrey?style=for-the-badge)](https://github.com/yourusername/RustFindAndReplace)

[![codecov](https://img.shields.io/codecov/c/github/yourusername/RustFindAndReplace?style=for-the-badge)](https://codecov.io/gh/yourusername/RustFindAndReplace)
[![Documentation](https://img.shields.io/docsrs/regex-replace?style=for-the-badge)](https://docs.rs/regex-replace)
[![Dependencies](https://img.shields.io/librariesio/github/yourusername/RustFindAndReplace?style=for-the-badge)](https://libraries.io/github/yourusername/RustFindAndReplace)
[![GitHub Issues](https://img.shields.io/github/issues/yourusername/RustFindAndReplace.svg?style=for-the-badge)](https://github.com/yourusername/RustFindAndReplace/issues)
[![PRs Welcome](https://img.shields.io/badge/PRs-welcome-brightgreen.svg?style=for-the-badge)](https://github.com/yourusername/RustFindAndReplace/pulls)
[![Maintenance](https://img.shields.io/badge/Maintained%3F-yes-green.svg?style=for-the-badge)](https://github.com/yourusername/RustFindAndReplace/graphs/commit-activity)

A blazing-fast command-line tool for performing regex-based search and replace operations across multiple files recursively, built with Rust for maximum performance and safety.

## 📋 Table of Contents

- [Quick Start](#-quick-start)
- [Features](#-features)
- [Installation](#-installation)
- [Usage](#-usage)
- [Ignore Patterns](#-ignore-patterns)
- [Regular Expression Syntax](#-regular-expression-syntax)
- [Testing](#-testing)
- [Safety Features](#️-safety-features)
- [Performance](#-performance)
- [Contributing](#-contributing)

## 🚀 Quick Start

```bash
# Install from crates.io
cargo install regex-replace

# Basic usage
regex-replace -p 'old_text' -r 'new_text'

# Preview changes without modifying files
regex-replace -p 'pattern' -r 'replacement' --dry-run
```

## ✨ Features

- **Recursive file processing** - Search through directories and subdirectories
- **Regex pattern matching** - Full regex support for complex pattern matching
- **File extension filtering** - Process only specific file types
- **Ignore patterns** - Skip files/directories using `.rr_ignore` files (gitignore syntax)
- **Dry run mode** - Preview changes without modifying files
- **Verbose output** - See detailed information about matches and replacements
- **Fast performance** - Built with Rust for speed and efficiency
- **Safe operation** - Graceful error handling for unreadable files
- **Hidden files support** - Option to include hidden files and directories

## 📦 Installation

### From Source

```bash
# Clone the repository
git clone https://github.com/yourusername/RustFindAndReplace.git
cd RustFindAndReplace

# Build the project
cargo build --release

# The binary will be available at ./target/release/regex-replace
```

### Using Cargo

```bash
cargo install --path .
```

## 🔧 Usage

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
- `--include-hidden` - Include hidden files and directories in search
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

#### Include hidden files
Search and replace in all files including hidden ones:
```bash
regex-replace -p 'DEBUG' -r 'RELEASE' --include-hidden
```

## 🚫 Ignore Patterns

The tool supports `.rr_ignore` files to exclude files and directories from processing. These files use gitignore-style syntax.

### Ignore File Locations

1. **Project-specific**: `.rr_ignore` in the current working directory
2. **Target directory**: `.rr_ignore` in the directory being searched (if different from cwd)
3. **User-global**: `~/.rr_ignore` in your home directory

All ignore files are combined, with patterns from all files being applied.

### Ignore Pattern Syntax

- `*.log` - Ignore all .log files
- `target/**` - Ignore all files in the target directory
- `node_modules/**` - Ignore all files in node_modules
- `.git/**` - Ignore all files in .git directory
- `*.{tmp,temp,swp}` - Ignore files with these extensions
- `#` - Lines starting with # are comments
- Empty lines are ignored

### Example .rr_ignore file

```
# Build artifacts
target/**
*.o
*.so

# Version control
.git/**
.svn/**

# Dependencies
node_modules/**
vendor/**

# IDE files
.vscode/**
.idea/**
*.iml

# Temporary files
*.tmp
*.swp
*~
```

## 🔍 Regular Expression Syntax

This tool uses Rust's regex crate, which supports:
- Character classes: `[a-z]`, `\d`, `\w`, `\s`
- Quantifiers: `*`, `+`, `?`, `{n,m}`
- Anchors: `^`, `$`, `\b`
- Groups: `(...)`, `(?:...)`, `(?P<name>...)`
- Capture groups in replacements: `$1`, `$2`, etc.

For full regex syntax documentation, see: https://docs.rs/regex/latest/regex/#syntax

## 🧪 Testing

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

## 🛡️ Safety Features

- **Non-destructive by default**: Use `--dry-run` to preview changes
- **File validation**: Skips binary files and handles encoding errors gracefully
- **Atomic writes**: Files are written completely or not at all
- **Clear error messages**: Detailed error reporting for debugging

## ⚡ Performance

RustFindAndReplace is optimized for maximum performance:
- Efficient directory traversal with WalkDir
- Regex compilation once per run (not per file)
- Memory-efficient streaming file processing
- Minimal allocations during replacement operations
- Skip binary files automatically for faster processing

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

### Development Setup

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Run tests (`cargo test`)
4. Commit your changes (`git commit -m 'Add some amazing feature'`)
5. Push to the branch (`git push origin feature/amazing-feature`)
6. Open a Pull Request

## 🏗️ Build Status

Built with Rust 2021 edition for stability and modern language features.

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- Built with [Rust](https://www.rust-lang.org/)
- Uses [regex](https://crates.io/crates/regex) for pattern matching
- Command-line parsing by [clap](https://crates.io/crates/clap)
- Directory traversal with [walkdir](https://crates.io/crates/walkdir)
- Error handling via [anyhow](https://crates.io/crates/anyhow)
- Pattern matching with [globset](https://crates.io/crates/globset)

---

<div align="center">

## 📊 Project Stats

[![GitHub Stars](https://img.shields.io/github/stars/yourusername/RustFindAndReplace?style=social)](https://github.com/yourusername/RustFindAndReplace/stargazers)
[![GitHub Forks](https://img.shields.io/github/forks/yourusername/RustFindAndReplace?style=social)](https://github.com/yourusername/RustFindAndReplace/network/members)
[![GitHub Watchers](https://img.shields.io/github/watchers/yourusername/RustFindAndReplace?style=social)](https://github.com/yourusername/RustFindAndReplace/watchers)

[![Lines of Code](https://img.shields.io/tokei/lines/github/yourusername/RustFindAndReplace?style=flat-square)](https://github.com/yourusername/RustFindAndReplace)
[![GitHub Last Commit](https://img.shields.io/github/last-commit/yourusername/RustFindAndReplace?style=flat-square)](https://github.com/yourusername/RustFindAndReplace/commits/master)
[![GitHub Contributors](https://img.shields.io/github/contributors/yourusername/RustFindAndReplace?style=flat-square)](https://github.com/yourusername/RustFindAndReplace/graphs/contributors)

</div>

---

<div align="center">
  Made with ❤️ in Rust
</div>
