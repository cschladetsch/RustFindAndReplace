#!/bin/bash

# Create a test directory
TEST_DIR="test_short_files_demo"
rm -rf $TEST_DIR
mkdir -p $TEST_DIR

echo "=== Testing RustFindAndReplace on Short Files ==="
echo

# Test 1: Single character file
echo "Test 1: Single character file"
echo "1" > $TEST_DIR/single_char.txt
echo "Before: $(cat $TEST_DIR/single_char.txt)"
rr -p "1" -r "X" -d $TEST_DIR
echo "After:  $(cat $TEST_DIR/single_char.txt)"
echo

# Test 2: Three character file (111 -> 222)
echo "Test 2: Three character file (111 -> 222)"
echo "111" > $TEST_DIR/three_chars.txt
echo "Before: $(cat $TEST_DIR/three_chars.txt)"
rr -p "111" -r "222" -d $TEST_DIR
echo "After:  $(cat $TEST_DIR/three_chars.txt)"
echo

# Test 3: Short word replacement
echo "Test 3: Short word replacement"
echo "hi" > $TEST_DIR/short_word.txt
echo "Before: $(cat $TEST_DIR/short_word.txt)"
rr -p "hi" -r "hello" -d $TEST_DIR
echo "After:  $(cat $TEST_DIR/short_word.txt)"
echo

# Test 4: Empty file (should handle gracefully)
echo "Test 4: Empty file"
touch $TEST_DIR/empty.txt
echo "Before: [empty file]"
rr -p "anything" -r "something" -d $TEST_DIR
echo "After:  [still empty]"
echo

# Test 5: Multiple short files
echo "Test 5: Multiple short files with same content"
echo "111" > $TEST_DIR/file1.txt
echo "111" > $TEST_DIR/file2.txt
echo "111" > $TEST_DIR/file3.txt
echo "Before: file1=$(cat $TEST_DIR/file1.txt), file2=$(cat $TEST_DIR/file2.txt), file3=$(cat $TEST_DIR/file3.txt)"
rr -p "111" -r "999" -d $TEST_DIR --verbose
echo "After:  file1=$(cat $TEST_DIR/file1.txt), file2=$(cat $TEST_DIR/file2.txt), file3=$(cat $TEST_DIR/file3.txt)"
echo

# Test 6: Very short file with regex
echo "Test 6: Single digit regex replacement"
echo "5" > $TEST_DIR/digit.txt
echo "Before: $(cat $TEST_DIR/digit.txt)"
rr -p "\d" -r "N" -d $TEST_DIR
echo "After:  $(cat $TEST_DIR/digit.txt)"
echo

# Cleanup
echo "=== Cleaning up test directory ==="
rm -rf $TEST_DIR