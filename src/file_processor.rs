use anyhow::{Context, Result};
use regex::Regex;
use std::fs;
use std::path::Path;

pub fn process_file(
    path: &Path,
    regex: &Regex,
    replacement: &str,
    dry_run: bool,
    verbose: bool,
) -> Result<bool> {
    let content = fs::read_to_string(path)
        .with_context(|| format!("Failed to read file: {}", path.display()))?;

    if !regex.is_match(&content) {
        return Ok(false);
    }

    let new_content = regex.replace_all(&content, replacement);

    if verbose || dry_run {
        println!("\nFile: {}", path.display());

        if verbose {
            let matches: Vec<_> = regex.find_iter(&content).collect();
            println!("Found {} matches", matches.len());

            if dry_run {
                for (i, mat) in matches.iter().enumerate() {
                    println!(
                        "  Match {}: \"{}\" -> \"{}\"",
                        i + 1,
                        &content[mat.start()..mat.end()],
                        replacement
                    );
                }
            }
        }
    }

    if !dry_run {
        // Only write if content actually changed (saves disk I/O)
        if new_content != content {
            fs::write(path, new_content.as_ref())
                .with_context(|| format!("Failed to write file: {}", path.display()))?;
        }
    }

    Ok(true)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn test_process_file_with_match() -> Result<()> {
        let temp_dir = TempDir::new()?;
        let file_path = temp_dir.path().join("test.txt");
        fs::write(&file_path, "Hello 123 World 456")?;

        let regex = Regex::new(r"\d+")?;
        let modified = process_file(&file_path, &regex, "XXX", false, false)?;

        assert!(modified);
        let content = fs::read_to_string(&file_path)?;
        assert_eq!(content, "Hello XXX World XXX");

        Ok(())
    }

    #[test]
    fn test_process_file_no_match() -> Result<()> {
        let temp_dir = TempDir::new()?;
        let file_path = temp_dir.path().join("test.txt");
        fs::write(&file_path, "Hello World")?;

        let regex = Regex::new(r"\d+")?;
        let modified = process_file(&file_path, &regex, "XXX", false, false)?;

        assert!(!modified);
        let content = fs::read_to_string(&file_path)?;
        assert_eq!(content, "Hello World");

        Ok(())
    }

    #[test]
    fn test_process_file_dry_run() -> Result<()> {
        let temp_dir = TempDir::new()?;
        let file_path = temp_dir.path().join("test.txt");
        let original_content = "Hello 123 World";
        fs::write(&file_path, original_content)?;

        let regex = Regex::new(r"\d+")?;
        let modified = process_file(&file_path, &regex, "XXX", true, false)?;

        assert!(modified);
        let content = fs::read_to_string(&file_path)?;
        assert_eq!(content, original_content);

        Ok(())
    }

    #[test]
    fn test_process_file_multiple_replacements() -> Result<()> {
        let temp_dir = TempDir::new()?;
        let file_path = temp_dir.path().join("test.txt");
        fs::write(&file_path, "foo bar foo baz foo")?;

        let regex = Regex::new(r"foo")?;
        let modified = process_file(&file_path, &regex, "replaced", false, false)?;

        assert!(modified);
        let content = fs::read_to_string(&file_path)?;
        assert_eq!(content, "replaced bar replaced baz replaced");

        Ok(())
    }

    #[test]
    fn test_process_file_with_special_chars() -> Result<()> {
        let temp_dir = TempDir::new()?;
        let file_path = temp_dir.path().join("test.txt");
        fs::write(&file_path, "Hello $world$ and $universe$")?;

        let regex = Regex::new(r"\$(\w+)\$")?;
        let modified = process_file(&file_path, &regex, "[$1]", false, false)?;

        assert!(modified);
        let content = fs::read_to_string(&file_path)?;
        assert_eq!(content, "Hello [world] and [universe]");

        Ok(())
    }

    #[test]
    fn test_process_file_nonexistent() {
        let path = Path::new("/nonexistent/file.txt");
        let regex = Regex::new(r"test").unwrap();
        let result = process_file(path, &regex, "replacement", false, false);

        assert!(result.is_err());
    }
}