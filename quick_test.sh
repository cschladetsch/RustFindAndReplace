#!/bin/bash

# Quick test script for RustFindAndReplace
set -e

echo "=== Quick Test for RustFindAndReplace ==="
echo

# Create test directory
TEST_DIR="quick_test_temp"
rm -rf $TEST_DIR
mkdir -p $TEST_DIR
cd $TEST_DIR

# Test 1: Simple replacement
echo "📝 Test 1: Simple replacement (111 → 222)"
echo "111 hello 111" > test1.txt
echo "Before: $(cat test1.txt)"
rr -p "111" -r "222"
echo "After:  $(cat test1.txt)"
echo "✅ Test 1 complete"
echo

# Test 2: Regex replacement
echo "📝 Test 2: Regex replacement (digits → NUM)"
echo "abc123def456" > test2.txt
echo "Before: $(cat test2.txt)"
rr -p '\d+' -r 'NUM'
echo "After:  $(cat test2.txt)"
echo "✅ Test 2 complete"
echo

# Test 3: Multiple files
echo "📝 Test 3: Multiple files"
echo "foo" > file1.txt
echo "foo" > file2.txt
echo "foo" > file3.txt
echo "Before: All files contain 'foo'"
rr -p "foo" -r "bar"
echo "After:  file1=$(cat file1.txt), file2=$(cat file2.txt), file3=$(cat file3.txt)"
echo "✅ Test 3 complete"
echo

# Test 4: Dry run
echo "📝 Test 4: Dry run (should not change)"
echo "change me" > dryrun.txt
echo "Before: $(cat dryrun.txt)"
rr -p "change me" -r "changed!" --dry-run
echo "After:  $(cat dryrun.txt) (should be unchanged)"
echo "✅ Test 4 complete"
echo

# Test 5: Extension filter
echo "📝 Test 5: Extension filtering"
echo "python" > test.py
echo "rust" > test.rs
echo "text" > test.txt
echo "Before: py='python', rs='rust', txt='text'"
rr -p '\w+' -r 'REPLACED' -e "py,rs"
echo "After:  py='$(cat test.py)', rs='$(cat test.rs)', txt='$(cat test.txt)'"
echo "✅ Test 5 complete"
echo

# Cleanup
cd ..
rm -rf $TEST_DIR

echo "🎉 All tests completed successfully!"