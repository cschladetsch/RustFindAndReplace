use std::fs;
use std::process::Command;
use tempfile::TempDir;

#[test]
fn test_single_character_replacement() {
    let temp_dir = TempDir::new().unwrap();
    
    // Single character files
    fs::write(temp_dir.path().join("one.txt"), "1").unwrap();
    fs::write(temp_dir.path().join("a.txt"), "a").unwrap();
    fs::write(temp_dir.path().join("x.txt"), "X").unwrap();
    
    // Replace digit
    let output = Command::new("cargo")
        .args(&["run", "--", "-p", "1", "-r", "9", "-d"])
        .arg(temp_dir.path())
        .output()
        .expect("Failed to execute command");
    
    assert!(output.status.success());
    assert_eq!(fs::read_to_string(temp_dir.path().join("one.txt")).unwrap(), "9");
    assert_eq!(fs::read_to_string(temp_dir.path().join("a.txt")).unwrap(), "a");
    assert_eq!(fs::read_to_string(temp_dir.path().join("x.txt")).unwrap(), "X");
}

#[test]
fn test_two_character_file() {
    let temp_dir = TempDir::new().unwrap();
    
    fs::write(temp_dir.path().join("ab.txt"), "ab").unwrap();
    
    let output = Command::new("cargo")
        .args(&["run", "--", "-p", "ab", "-r", "XY", "-d"])
        .arg(temp_dir.path())
        .output()
        .expect("Failed to execute command");
    
    assert!(output.status.success());
    assert_eq!(fs::read_to_string(temp_dir.path().join("ab.txt")).unwrap(), "XY");
}

#[test]
fn test_three_character_file() {
    let temp_dir = TempDir::new().unwrap();
    
    fs::write(temp_dir.path().join("num.txt"), "111").unwrap();
    
    let output = Command::new("cargo")
        .args(&["run", "--", "-p", "111", "-r", "222", "-d"])
        .arg(temp_dir.path())
        .output()
        .expect("Failed to execute command");
    
    assert!(output.status.success());
    assert_eq!(fs::read_to_string(temp_dir.path().join("num.txt")).unwrap(), "222");
}

#[test]
fn test_newline_only_file() {
    let temp_dir = TempDir::new().unwrap();
    
    fs::write(temp_dir.path().join("newline.txt"), "\n").unwrap();
    
    let output = Command::new("cargo")
        .args(&["run", "--", "-p", r"\n", "-r", "NEWLINE", "-d"])
        .arg(temp_dir.path())
        .output()
        .expect("Failed to execute command");
    
    assert!(output.status.success());
    assert_eq!(fs::read_to_string(temp_dir.path().join("newline.txt")).unwrap(), "NEWLINE");
}

#[test]
fn test_space_only_file() {
    let temp_dir = TempDir::new().unwrap();
    
    fs::write(temp_dir.path().join("space.txt"), " ").unwrap();
    
    let output = Command::new("cargo")
        .args(&["run", "--", "-p", " ", "-r", "_", "-d"])
        .arg(temp_dir.path())
        .output()
        .expect("Failed to execute command");
    
    assert!(output.status.success());
    assert_eq!(fs::read_to_string(temp_dir.path().join("space.txt")).unwrap(), "_");
}

#[test]
fn test_very_short_content_with_regex() {
    let temp_dir = TempDir::new().unwrap();
    
    // Test regex on very short content
    fs::write(temp_dir.path().join("dot.txt"), ".").unwrap();
    fs::write(temp_dir.path().join("digit.txt"), "7").unwrap();
    fs::write(temp_dir.path().join("letter.txt"), "z").unwrap();
    
    // Replace any character with X
    let output = Command::new("cargo")
        .args(&["run", "--", "-p", ".", "-r", "X", "-d"])
        .arg(temp_dir.path())
        .output()
        .expect("Failed to execute command");
    
    assert!(output.status.success());
    assert_eq!(fs::read_to_string(temp_dir.path().join("dot.txt")).unwrap(), "X");
    assert_eq!(fs::read_to_string(temp_dir.path().join("digit.txt")).unwrap(), "X");
    assert_eq!(fs::read_to_string(temp_dir.path().join("letter.txt")).unwrap(), "X");
}

#[test]
fn test_multiple_short_files_batch() {
    let temp_dir = TempDir::new().unwrap();
    
    // Create 10 files with just "1"
    for i in 0..10 {
        fs::write(temp_dir.path().join(format!("f{}.txt", i)), "1").unwrap();
    }
    
    let output = Command::new("cargo")
        .args(&["run", "--", "-p", "1", "-r", "X", "-d"])
        .arg(temp_dir.path())
        .output()
        .expect("Failed to execute command");
    
    assert!(output.status.success());
    
    // Verify all files were changed
    for i in 0..10 {
        assert_eq!(
            fs::read_to_string(temp_dir.path().join(format!("f{}.txt", i))).unwrap(),
            "X"
        );
    }
    
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Total files processed: 10"));
    assert!(stdout.contains("Files modified: 10"));
}

#[test]
fn test_empty_file_handling() {
    let temp_dir = TempDir::new().unwrap();
    
    // Create empty file
    fs::write(temp_dir.path().join("empty.txt"), "").unwrap();
    
    let output = Command::new("cargo")
        .args(&["run", "--", "-p", "anything", "-r", "something", "-d"])
        .arg(temp_dir.path())
        .output()
        .expect("Failed to execute command");
    
    assert!(output.status.success());
    assert_eq!(fs::read_to_string(temp_dir.path().join("empty.txt")).unwrap(), "");
    
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Files modified: 0"));
}

#[test]
fn test_single_line_no_newline() {
    let temp_dir = TempDir::new().unwrap();
    
    // File with no trailing newline
    fs::write(temp_dir.path().join("no_newline.txt"), "hello").unwrap();
    
    let output = Command::new("cargo")
        .args(&["run", "--", "-p", "hello", "-r", "goodbye", "-d"])
        .arg(temp_dir.path())
        .output()
        .expect("Failed to execute command");
    
    assert!(output.status.success());
    assert_eq!(fs::read_to_string(temp_dir.path().join("no_newline.txt")).unwrap(), "goodbye");
}