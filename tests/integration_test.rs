use std::fs;
use std::process::Command;
use tempfile::TempDir;

#[test]
fn test_basic_replacement() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.txt");
    fs::write(&file_path, "Hello 123 World 456").unwrap();

    let output = Command::new("cargo")
        .args(&["run", "--", "-p", r"\d+", "-r", "XXX", "-d"])
        .arg(temp_dir.path())
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());
    let content = fs::read_to_string(&file_path).unwrap();
    assert_eq!(content, "Hello XXX World XXX");
}

#[test]
fn test_dry_run() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.txt");
    let original_content = "Replace this 123";
    fs::write(&file_path, original_content).unwrap();

    let output = Command::new("cargo")
        .args(&["run", "--", "-p", r"\d+", "-r", "XXX", "-d"])
        .arg(temp_dir.path())
        .arg("--dry-run")
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());
    let content = fs::read_to_string(&file_path).unwrap();
    assert_eq!(content, original_content);
}

#[test]
fn test_file_extension_filter() {
    let temp_dir = TempDir::new().unwrap();
    
    fs::write(temp_dir.path().join("test.txt"), "Hello 123").unwrap();
    fs::write(temp_dir.path().join("test.rs"), "let x = 123;").unwrap();
    fs::write(temp_dir.path().join("test.md"), "Number: 123").unwrap();

    let output = Command::new("cargo")
        .args(&["run", "--", "-p", r"\d+", "-r", "XXX", "-d"])
        .arg(temp_dir.path())
        .args(&["-e", "txt,rs"])
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());
    
    assert_eq!(fs::read_to_string(temp_dir.path().join("test.txt")).unwrap(), "Hello XXX");
    assert_eq!(fs::read_to_string(temp_dir.path().join("test.rs")).unwrap(), "let x = XXX;");
    assert_eq!(fs::read_to_string(temp_dir.path().join("test.md")).unwrap(), "Number: 123");
}

#[test]
fn test_recursive_directory_traversal() {
    let temp_dir = TempDir::new().unwrap();
    let sub_dir = temp_dir.path().join("subdir");
    fs::create_dir(&sub_dir).unwrap();
    
    fs::write(temp_dir.path().join("file1.txt"), "Test 123").unwrap();
    fs::write(sub_dir.join("file2.txt"), "Test 456").unwrap();

    let output = Command::new("cargo")
        .args(&["run", "--", "-p", r"\d+", "-r", "NUM", "-d"])
        .arg(temp_dir.path())
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());
    
    assert_eq!(fs::read_to_string(temp_dir.path().join("file1.txt")).unwrap(), "Test NUM");
    assert_eq!(fs::read_to_string(sub_dir.join("file2.txt")).unwrap(), "Test NUM");
}

#[test]
fn test_verbose_output() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.txt");
    fs::write(&file_path, "Hello 123 World").unwrap();

    let output = Command::new("cargo")
        .args(&["run", "--", "-p", r"\d+", "-r", "XXX", "-d"])
        .arg(temp_dir.path())
        .arg("--verbose")
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("test.txt"));
    assert!(stdout.contains("Found 1 matches"));
}

#[test]
fn test_invalid_regex() {
    let output = Command::new("cargo")
        .args(&["run", "--", "-p", "[", "-r", "XXX", "-d", "."])
        .output()
        .expect("Failed to execute command");

    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("Invalid regex pattern"));
}

#[test]
fn test_no_matches() {
    let temp_dir = TempDir::new().unwrap();
    fs::write(temp_dir.path().join("test.txt"), "Hello World").unwrap();

    let output = Command::new("cargo")
        .args(&["run", "--", "-p", r"\d+", "-r", "XXX", "-d"])
        .arg(temp_dir.path())
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Files modified: 0"));
}

#[test]
fn test_multiple_files() {
    let temp_dir = TempDir::new().unwrap();
    
    for i in 1..=5 {
        fs::write(
            temp_dir.path().join(format!("file{}.txt", i)),
            format!("Number: {}", i * 100)
        ).unwrap();
    }

    let output = Command::new("cargo")
        .args(&["run", "--", "-p", r"\d+", "-r", "XXX", "-d"])
        .arg(temp_dir.path())
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());
    
    for i in 1..=5 {
        let content = fs::read_to_string(temp_dir.path().join(format!("file{}.txt", i))).unwrap();
        assert_eq!(content, "Number: XXX");
    }
    
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Total files processed: 5"));
    assert!(stdout.contains("Files modified: 5"));
}