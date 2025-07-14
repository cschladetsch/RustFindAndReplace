# Testing Guide for regex-replace

## Prerequisites

Ensure you have Rust installed. If not, install it from https://rustup.rs/

## Running Tests

### Run all tests (unit + integration)
```bash
cargo test
```

### Run tests with output displayed
```bash
cargo test -- --nocapture
```

### Run tests in parallel (default) or sequentially
```bash
# Parallel (faster, default)
cargo test

# Sequential (useful for debugging)
cargo test -- --test-threads=1
```

### Run only unit tests
```bash
cargo test --lib
```

### Run only integration tests
```bash
cargo test --test integration_test
```

### Run a specific test by name
```bash
# Run a specific unit test
cargo test test_process_file_with_match

# Run all tests matching a pattern
cargo test test_process_file
```

### Run tests with verbose output
```bash
cargo test -- --nocapture --test-threads=1
```

### Run tests in release mode (optimized)
```bash
cargo test --release
```

## Test Coverage

The test suite includes:

### Unit Tests (in `src/main.rs`)
- `test_regex_creation` - Validates regex pattern compilation
- `test_process_file_with_match` - Tests basic file replacement
- `test_process_file_no_match` - Tests behavior when no matches found
- `test_process_file_dry_run` - Ensures dry-run doesn't modify files
- `test_process_file_multiple_replacements` - Tests multiple replacements
- `test_process_file_with_special_chars` - Tests regex capture groups
- `test_process_file_nonexistent` - Tests error handling

### Integration Tests (in `tests/integration_test.rs`)
- `test_basic_replacement` - End-to-end basic replacement
- `test_dry_run` - CLI dry-run mode verification
- `test_file_extension_filter` - Extension filtering functionality
- `test_recursive_directory_traversal` - Recursive directory processing
- `test_verbose_output` - Verbose output validation
- `test_invalid_regex` - Invalid regex error handling
- `test_no_matches` - No matches found scenario
- `test_multiple_files` - Multiple file processing

## Quick Test Commands

```bash
# Most common - run all tests with output
cargo test -- --nocapture

# Quick check - run all tests
cargo test

# Debug a specific test
cargo test test_name -- --nocapture --test-threads=1
```

## Continuous Integration

For CI/CD pipelines, use:
```bash
cargo test --all-features --no-fail-fast
```