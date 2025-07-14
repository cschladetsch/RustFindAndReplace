use std::fs;
use std::process::Command;
use tempfile::TempDir;
use std::path::Path;

fn run_replacement(dir: &Path, pattern: &str, replacement: &str, extra_args: &[&str]) -> std::process::Output {
    let mut cmd = Command::new("cargo");
    cmd.args(&["run", "--", "-p", pattern, "-r", replacement, "-d"])
       .arg(dir);
    
    for arg in extra_args {
        cmd.arg(arg);
    }
    
    cmd.output().expect("Failed to execute command")
}

#[test]
fn test_simple_numeric_replacement() {
    let temp_dir = TempDir::new().unwrap();
    
    // Test 111 -> 222
    fs::write(temp_dir.path().join("test1.txt"), "111").unwrap();
    let output = run_replacement(temp_dir.path(), "111", "222", &[]);
    assert!(output.status.success());
    assert_eq!(fs::read_to_string(temp_dir.path().join("test1.txt")).unwrap(), "222");
    
    // Test 222 -> 333
    let output = run_replacement(temp_dir.path(), "222", "333", &[]);
    assert!(output.status.success());
    assert_eq!(fs::read_to_string(temp_dir.path().join("test1.txt")).unwrap(), "333");
    
    // Test multiple occurrences
    fs::write(temp_dir.path().join("test2.txt"), "111 111 111").unwrap();
    let output = run_replacement(temp_dir.path(), "111", "222", &[]);
    assert!(output.status.success());
    assert_eq!(fs::read_to_string(temp_dir.path().join("test2.txt")).unwrap(), "222 222 222");
}

#[test]
fn test_complex_content_replacement() {
    let temp_dir = TempDir::new().unwrap();
    
    // Test with mixed content
    fs::write(temp_dir.path().join("mixed.txt"), "The number 111 appears here and 111 here too.").unwrap();
    let output = run_replacement(temp_dir.path(), "111", "999", &[]);
    assert!(output.status.success());
    assert_eq!(
        fs::read_to_string(temp_dir.path().join("mixed.txt")).unwrap(),
        "The number 999 appears here and 999 here too."
    );
    
    // Test with newlines
    fs::write(temp_dir.path().join("multiline.txt"), "111\n222\n111\n333").unwrap();
    let output = run_replacement(temp_dir.path(), "111", "AAA", &[]);
    assert!(output.status.success());
    assert_eq!(
        fs::read_to_string(temp_dir.path().join("multiline.txt")).unwrap(),
        "AAA\n222\nAAA\n333"
    );
}

#[test]
fn test_regex_pattern_replacements() {
    let temp_dir = TempDir::new().unwrap();
    
    // Test digit replacement
    fs::write(temp_dir.path().join("digits.txt"), "abc123def456ghi789").unwrap();
    let output = run_replacement(temp_dir.path(), r"\d+", "NUM", &[]);
    assert!(output.status.success());
    assert_eq!(
        fs::read_to_string(temp_dir.path().join("digits.txt")).unwrap(),
        "abcNUMdefNUMghiNUM"
    );
    
    // Test word boundary replacement
    fs::write(temp_dir.path().join("words.txt"), "foo foobar barfoo foo").unwrap();
    let output = run_replacement(temp_dir.path(), r"\bfoo\b", "baz", &[]);
    assert!(output.status.success());
    assert_eq!(
        fs::read_to_string(temp_dir.path().join("words.txt")).unwrap(),
        "baz foobar barfoo baz"
    );
    
    // Test character class replacement
    fs::write(temp_dir.path().join("chars.txt"), "a1b2c3d4e5").unwrap();
    let output = run_replacement(temp_dir.path(), r"[a-z]", "X", &[]);
    assert!(output.status.success());
    assert_eq!(
        fs::read_to_string(temp_dir.path().join("chars.txt")).unwrap(),
        "X1X2X3X4X5"
    );
}

#[test]
fn test_special_characters_in_content() {
    let temp_dir = TempDir::new().unwrap();
    
    // Test with special regex characters in content
    fs::write(temp_dir.path().join("special.txt"), "Price: $111.99 (was $222.99)").unwrap();
    let output = run_replacement(temp_dir.path(), r"\$111\.99", "USD 333.33", &[]);
    assert!(output.status.success());
    assert_eq!(
        fs::read_to_string(temp_dir.path().join("special.txt")).unwrap(),
        "Price: USD 333.33 (was $222.99)"
    );
    
    // Test with backslashes
    fs::write(temp_dir.path().join("paths.txt"), r"C:\Users\111\Documents").unwrap();
    let output = run_replacement(temp_dir.path(), "111", "JohnDoe", &[]);
    assert!(output.status.success());
    assert_eq!(
        fs::read_to_string(temp_dir.path().join("paths.txt")).unwrap(),
        r"C:\Users\JohnDoe\Documents"
    );
}

#[test]
fn test_empty_and_whitespace_handling() {
    let temp_dir = TempDir::new().unwrap();
    
    // Test empty file
    fs::write(temp_dir.path().join("empty.txt"), "").unwrap();
    let output = run_replacement(temp_dir.path(), "111", "222", &[]);
    assert!(output.status.success());
    assert_eq!(fs::read_to_string(temp_dir.path().join("empty.txt")).unwrap(), "");
    
    // Test whitespace replacement
    fs::write(temp_dir.path().join("spaces.txt"), "111   111\t111").unwrap();
    let output = run_replacement(temp_dir.path(), "111", "X", &[]);
    assert!(output.status.success());
    assert_eq!(
        fs::read_to_string(temp_dir.path().join("spaces.txt")).unwrap(),
        "X   X\tX"
    );
    
    // Test replacement with spaces
    fs::write(temp_dir.path().join("space_replace.txt"), "foo bar baz").unwrap();
    let output = run_replacement(temp_dir.path(), " ", "_", &[]);
    assert!(output.status.success());
    assert_eq!(
        fs::read_to_string(temp_dir.path().join("space_replace.txt")).unwrap(),
        "foo_bar_baz"
    );
}

#[test]
fn test_large_file_replacement() {
    let temp_dir = TempDir::new().unwrap();
    
    // Create a file with many occurrences
    let mut content = String::new();
    for i in 0..1000 {
        content.push_str(&format!("Line {}: The pattern AAA appears here.\n", i));
    }
    fs::write(temp_dir.path().join("large.txt"), &content).unwrap();
    
    let output = run_replacement(temp_dir.path(), "AAA", "BBB", &[]);
    assert!(output.status.success());
    
    let result = fs::read_to_string(temp_dir.path().join("large.txt")).unwrap();
    assert!(!result.contains("AAA"));
    assert!(result.contains("BBB"));
    assert_eq!(result.matches("BBB").count(), 1000);
}

#[test]
fn test_unicode_content_replacement() {
    let temp_dir = TempDir::new().unwrap();
    
    // Test with unicode content
    fs::write(temp_dir.path().join("unicode.txt"), "Hello 世界 111 こんにちは 111").unwrap();
    let output = run_replacement(temp_dir.path(), "111", "二二二", &[]);
    assert!(output.status.success());
    assert_eq!(
        fs::read_to_string(temp_dir.path().join("unicode.txt")).unwrap(),
        "Hello 世界 二二二 こんにちは 二二二"
    );
    
    // Test emoji replacement
    fs::write(temp_dir.path().join("emoji.txt"), "Happy 😊 Sad 😢 Happy 😊").unwrap();
    let output = run_replacement(temp_dir.path(), "😊", "😎", &[]);
    assert!(output.status.success());
    assert_eq!(
        fs::read_to_string(temp_dir.path().join("emoji.txt")).unwrap(),
        "Happy 😎 Sad 😢 Happy 😎"
    );
}

#[test]
fn test_binary_file_handling() {
    let temp_dir = TempDir::new().unwrap();
    
    // Create a binary-like file
    let binary_content = vec![0u8, 1, 2, 3, 255, 254, 253];
    fs::write(temp_dir.path().join("binary.bin"), &binary_content).unwrap();
    
    // Should handle binary files gracefully
    let output = run_replacement(temp_dir.path(), "111", "222", &[]);
    assert!(output.status.success());
    
    // Binary content should remain unchanged (no text match)
    let result = fs::read(temp_dir.path().join("binary.bin")).unwrap();
    assert_eq!(result, binary_content);
}

#[test]
fn test_case_sensitive_replacement() {
    let temp_dir = TempDir::new().unwrap();
    
    // Test case sensitivity
    fs::write(temp_dir.path().join("case.txt"), "AAA aaa Aaa AaA").unwrap();
    let output = run_replacement(temp_dir.path(), "aaa", "bbb", &[]);
    assert!(output.status.success());
    assert_eq!(
        fs::read_to_string(temp_dir.path().join("case.txt")).unwrap(),
        "AAA bbb Aaa AaA"
    );
}

#[test]
fn test_overlapping_patterns() {
    let temp_dir = TempDir::new().unwrap();
    
    // Test overlapping pattern replacement
    fs::write(temp_dir.path().join("overlap.txt"), "111111").unwrap();
    let output = run_replacement(temp_dir.path(), "111", "X", &[]);
    assert!(output.status.success());
    assert_eq!(
        fs::read_to_string(temp_dir.path().join("overlap.txt")).unwrap(),
        "XX"
    );
    
    // Test with different overlap
    fs::write(temp_dir.path().join("overlap2.txt"), "abcabcabc").unwrap();
    let output = run_replacement(temp_dir.path(), "abc", "XYZ", &[]);
    assert!(output.status.success());
    assert_eq!(
        fs::read_to_string(temp_dir.path().join("overlap2.txt")).unwrap(),
        "XYZXYZXYZ"
    );
}

#[test]
fn test_replacement_with_groups() {
    let temp_dir = TempDir::new().unwrap();
    
    // Note: This tests literal replacement, not capture group substitution
    // since the tool doesn't support $1 style replacements
    fs::write(temp_dir.path().join("groups.txt"), "foo123bar456baz789").unwrap();
    let output = run_replacement(temp_dir.path(), r"[a-z]+(\d+)", "WORD_NUM", &[]);
    assert!(output.status.success());
    assert_eq!(
        fs::read_to_string(temp_dir.path().join("groups.txt")).unwrap(),
        "WORD_NUMWORD_NUMWORD_NUM"
    );
}

#[test]
fn test_dry_run_preserves_content() {
    let temp_dir = TempDir::new().unwrap();
    
    let original = "111 222 333 111";
    fs::write(temp_dir.path().join("preserve.txt"), original).unwrap();
    
    let output = run_replacement(temp_dir.path(), "111", "999", &["--dry-run"]);
    assert!(output.status.success());
    
    // Content should be unchanged
    assert_eq!(
        fs::read_to_string(temp_dir.path().join("preserve.txt")).unwrap(),
        original
    );
    
    // But output should indicate what would be changed
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Files modified: 1"));
    assert!(stdout.contains("Dry run"));
}

#[test]
fn test_hidden_files_handling() {
    let temp_dir = TempDir::new().unwrap();
    
    // Create hidden file
    fs::write(temp_dir.path().join(".hidden.txt"), "111").unwrap();
    fs::write(temp_dir.path().join("visible.txt"), "111").unwrap();
    
    // Without --include-hidden
    let output = run_replacement(temp_dir.path(), "111", "222", &[]);
    assert!(output.status.success());
    assert_eq!(fs::read_to_string(temp_dir.path().join(".hidden.txt")).unwrap(), "111");
    assert_eq!(fs::read_to_string(temp_dir.path().join("visible.txt")).unwrap(), "222");
    
    // With --include-hidden
    let output = run_replacement(temp_dir.path(), "222", "333", &["--include-hidden"]);
    assert!(output.status.success());
    assert_eq!(fs::read_to_string(temp_dir.path().join(".hidden.txt")).unwrap(), "111");
    assert_eq!(fs::read_to_string(temp_dir.path().join("visible.txt")).unwrap(), "333");
}

#[test]
fn test_nested_directory_replacement() {
    let temp_dir = TempDir::new().unwrap();
    
    // Create nested structure
    let deep_path = temp_dir.path().join("a").join("b").join("c");
    fs::create_dir_all(&deep_path).unwrap();
    
    fs::write(temp_dir.path().join("root.txt"), "111").unwrap();
    fs::write(temp_dir.path().join("a").join("a.txt"), "111").unwrap();
    fs::write(temp_dir.path().join("a").join("b").join("b.txt"), "111").unwrap();
    fs::write(deep_path.join("c.txt"), "111").unwrap();
    
    let output = run_replacement(temp_dir.path(), "111", "222", &[]);
    assert!(output.status.success());
    
    assert_eq!(fs::read_to_string(temp_dir.path().join("root.txt")).unwrap(), "222");
    assert_eq!(fs::read_to_string(temp_dir.path().join("a").join("a.txt")).unwrap(), "222");
    assert_eq!(fs::read_to_string(temp_dir.path().join("a").join("b").join("b.txt")).unwrap(), "222");
    assert_eq!(fs::read_to_string(deep_path.join("c.txt")).unwrap(), "222");
    
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Total files processed: 4"));
    assert!(stdout.contains("Files modified: 4"));
}

#[test]
fn test_ignore_patterns() {
    let temp_dir = TempDir::new().unwrap();
    
    // Create .rr_ignore file
    fs::write(temp_dir.path().join(".rr_ignore"), "ignored/**\n*.log").unwrap();
    
    // Create files
    fs::create_dir(temp_dir.path().join("ignored")).unwrap();
    fs::write(temp_dir.path().join("ignored").join("file.txt"), "111").unwrap();
    fs::write(temp_dir.path().join("test.txt"), "111").unwrap();
    fs::write(temp_dir.path().join("test.log"), "111").unwrap();
    
    let output = run_replacement(temp_dir.path(), "111", "222", &[]);
    assert!(output.status.success());
    
    // Ignored files should be unchanged
    assert_eq!(fs::read_to_string(temp_dir.path().join("ignored").join("file.txt")).unwrap(), "111");
    assert_eq!(fs::read_to_string(temp_dir.path().join("test.log")).unwrap(), "111");
    
    // Non-ignored file should be changed
    assert_eq!(fs::read_to_string(temp_dir.path().join("test.txt")).unwrap(), "222");
}

#[test]
fn test_extension_filtering() {
    let temp_dir = TempDir::new().unwrap();
    
    fs::write(temp_dir.path().join("test.txt"), "111").unwrap();
    fs::write(temp_dir.path().join("test.rs"), "111").unwrap();
    fs::write(temp_dir.path().join("test.md"), "111").unwrap();
    fs::write(temp_dir.path().join("test.py"), "111").unwrap();
    
    let output = run_replacement(temp_dir.path(), "111", "222", &["-e", "txt,rs"]);
    assert!(output.status.success());
    
    assert_eq!(fs::read_to_string(temp_dir.path().join("test.txt")).unwrap(), "222");
    assert_eq!(fs::read_to_string(temp_dir.path().join("test.rs")).unwrap(), "222");
    assert_eq!(fs::read_to_string(temp_dir.path().join("test.md")).unwrap(), "111");
    assert_eq!(fs::read_to_string(temp_dir.path().join("test.py")).unwrap(), "111");
}

#[test]
fn test_no_extension_files() {
    let temp_dir = TempDir::new().unwrap();
    
    fs::write(temp_dir.path().join("Makefile"), "111").unwrap();
    fs::write(temp_dir.path().join("LICENSE"), "111").unwrap();
    fs::write(temp_dir.path().join("test.txt"), "111").unwrap();
    
    // Without extension filter, all should be processed
    let output = run_replacement(temp_dir.path(), "111", "222", &[]);
    assert!(output.status.success());
    
    assert_eq!(fs::read_to_string(temp_dir.path().join("Makefile")).unwrap(), "222");
    assert_eq!(fs::read_to_string(temp_dir.path().join("LICENSE")).unwrap(), "222");
    assert_eq!(fs::read_to_string(temp_dir.path().join("test.txt")).unwrap(), "222");
}

#[test]
fn test_symbolic_content() {
    let temp_dir = TempDir::new().unwrap();
    
    // Test various symbols and operators
    fs::write(temp_dir.path().join("symbols.txt"), "a+b=111, c*d=111, e/f=111").unwrap();
    let output = run_replacement(temp_dir.path(), "111", "X", &[]);
    assert!(output.status.success());
    assert_eq!(
        fs::read_to_string(temp_dir.path().join("symbols.txt")).unwrap(),
        "a+b=X, c*d=X, e/f=X"
    );
    
    // Test code-like content
    fs::write(temp_dir.path().join("code.txt"), "if (x == 111) { return 111; }").unwrap();
    let output = run_replacement(temp_dir.path(), "111", "999", &[]);
    assert!(output.status.success());
    assert_eq!(
        fs::read_to_string(temp_dir.path().join("code.txt")).unwrap(),
        "if (x == 999) { return 999; }"
    );
}

#[test]
fn test_incremental_replacements() {
    let temp_dir = TempDir::new().unwrap();
    
    fs::write(temp_dir.path().join("increment.txt"), "111").unwrap();
    
    // Chain of replacements
    for (from, to) in vec![("111", "222"), ("222", "333"), ("333", "444"), ("444", "555")] {
        let output = run_replacement(temp_dir.path(), from, to, &[]);
        assert!(output.status.success());
    }
    
    assert_eq!(
        fs::read_to_string(temp_dir.path().join("increment.txt")).unwrap(),
        "555"
    );
}

#[test]
fn test_line_ending_preservation() {
    let temp_dir = TempDir::new().unwrap();
    
    // Test LF endings
    fs::write(temp_dir.path().join("lf.txt"), "111\n222\n111\n").unwrap();
    let output = run_replacement(temp_dir.path(), "111", "AAA", &[]);
    assert!(output.status.success());
    assert_eq!(
        fs::read_to_string(temp_dir.path().join("lf.txt")).unwrap(),
        "AAA\n222\nAAA\n"
    );
    
    // Test CRLF endings
    fs::write(temp_dir.path().join("crlf.txt"), "111\r\n222\r\n111\r\n").unwrap();
    let output = run_replacement(temp_dir.path(), "111", "BBB", &[]);
    assert!(output.status.success());
    assert_eq!(
        fs::read_to_string(temp_dir.path().join("crlf.txt")).unwrap(),
        "BBB\r\n222\r\nBBB\r\n"
    );
}