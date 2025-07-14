# How to Test RustFindAndReplace

## 1. Run All Tests
```bash
# Run all tests in release mode
cargo test --release

# Run with verbose output
cargo test --release -- --nocapture

# Run tests in parallel (default)
cargo test --release

# Run tests sequentially
cargo test --release -- --test-threads=1
```

## 2. Run Specific Test Suites
```bash
# Run only unit tests
cargo test --release --lib

# Run only integration tests
cargo test --release --test integration_test

# Run exhaustive replacement tests
cargo test --release --test exhaustive_replacement_tests

# Run short file tests
cargo test --release --test short_file_tests

# Run specific test by name
cargo test --release test_simple_numeric_replacement
```

## 3. Manual Testing

### Basic replacement test
```bash
# Create a test file
echo "Hello 111 World 111" > test.txt

# Replace 111 with 222
rr -p "111" -r "222"

# Check the result
cat test.txt  # Should show: Hello 222 World 222
```

### Test with dry run
```bash
# Create test file
echo "Replace this 123" > test.txt

# Dry run (no actual changes)
rr -p "123" -r "999" --dry-run

# File should be unchanged
cat test.txt  # Still shows: Replace this 123
```

### Test with regex patterns
```bash
# Create test file with numbers
echo "abc123def456ghi789" > test.txt

# Replace all digits with NUM
rr -p '\d+' -r 'NUM'

# Check result
cat test.txt  # Should show: abcNUMdefNUMghiNUM
```

### Test directory recursion
```bash
# Create nested directories
mkdir -p test_dir/sub1/sub2
echo "111" > test_dir/file1.txt
echo "111" > test_dir/sub1/file2.txt
echo "111" > test_dir/sub1/sub2/file3.txt

# Replace in all files
rr -p "111" -r "999" -d test_dir

# All files should be changed
find test_dir -name "*.txt" -exec cat {} \;
```

### Test file extension filtering
```bash
# Create files with different extensions
echo "111" > test.txt
echo "111" > test.rs
echo "111" > test.md

# Only replace in .txt and .rs files
rr -p "111" -r "222" -e "txt,rs"

# Check results
cat test.txt  # Should show: 222
cat test.rs   # Should show: 222
cat test.md   # Should show: 111 (unchanged)
```

### Test ignore patterns
```bash
# Create .rr_ignore file
echo "*.log" > .rr_ignore
echo "temp/**" >> .rr_ignore

# Create files
echo "111" > test.txt
echo "111" > test.log
mkdir temp
echo "111" > temp/ignored.txt

# Run replacement
rr -p "111" -r "222"

# Check what was changed
cat test.txt         # Should show: 222
cat test.log         # Should show: 111 (ignored)
cat temp/ignored.txt # Should show: 111 (ignored)
```

### Test verbose mode
```bash
# Create test file
echo "Find this pattern" > test.txt

# Run with verbose output
rr -p "pattern" -r "REPLACED" --verbose

# Should show detailed information about the replacement
```

## 4. Performance Testing

### Test with many files
```bash
# Create 100 files
for i in {1..100}; do echo "Test 111 content" > "file$i.txt"; done

# Time the replacement
time rr -p "111" -r "999"

# Clean up
rm file*.txt
```

### Test with large file
```bash
# Create a large file (1MB)
yes "This is line 111 with some text" | head -n 30000 > large.txt

# Replace in large file
time rr -p "111" -r "999"

# Clean up
rm large.txt
```

## 5. Edge Case Testing

### Empty files
```bash
touch empty.txt
rr -p "anything" -r "something"
# Should handle gracefully
```

### Binary files
```bash
# Create a binary file
echo -e "\x00\x01\x02\x03" > binary.bin
rr -p "111" -r "222"
# Should handle gracefully without corrupting
```

### Special characters
```bash
echo 'Price: $111.99' > price.txt
rr -p '\$111\.99' -r 'USD 333.33'
cat price.txt  # Should show: Price: USD 333.33
```

### Hidden files
```bash
echo "111" > .hidden.txt
echo "111" > visible.txt

# Without --include-hidden
rr -p "111" -r "222"
cat .hidden.txt  # Should show: 111 (unchanged)
cat visible.txt  # Should show: 222

# With --include-hidden
rr -p "111" -r "333" --include-hidden
cat .hidden.txt  # Should show: 333
```

## 6. Integration Testing Script

Create a comprehensive test script:

```bash
#!/bin/bash
# save as test_all.sh

echo "Running comprehensive tests..."

# Test 1: Basic replacement
echo "Test 1: Basic replacement"
echo "111" > test1.txt
rr -p "111" -r "222"
[[ $(cat test1.txt) == "222" ]] && echo "PASS" || echo "FAIL"

# Test 2: Multiple files
echo "Test 2: Multiple files"
echo "AAA" > test2a.txt
echo "AAA" > test2b.txt
rr -p "AAA" -r "BBB"
[[ $(cat test2a.txt) == "BBB" && $(cat test2b.txt) == "BBB" ]] && echo "PASS" || echo "FAIL"

# Add more tests...

# Cleanup
rm -f test*.txt
echo "Tests complete!"
```

## 7. Debugging Tests

If tests fail, debug with:

```bash
# Run with Rust backtrace
RUST_BACKTRACE=1 cargo test --release

# Run single test with output
cargo test --release test_name -- --nocapture --exact

# Check test binary directly
./target/release/regex-replace -p "test" -r "TEST" --verbose
```

## Tips

1. Always test both with and without `--dry-run` first
2. Test on copies of important files
3. Use `--verbose` to understand what's happening
4. Check the summary output for confirmation
5. Use version control to track changes and rollback if needed