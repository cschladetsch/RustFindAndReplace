use anyhow::{Context, Result};
use clap::Parser;
use globset::{Glob, GlobSet, GlobSetBuilder};
use regex::Regex;
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, help = "Regex pattern to search for")]
    pattern: String,

    #[arg(short, long, help = "Replacement text")]
    replace: String,

    #[arg(short, long, default_value = ".", help = "Directory to search in")]
    directory: String,

    #[arg(short, long, help = "File extensions to include (e.g., txt,rs,js)")]
    extensions: Option<String>,

    #[arg(short = 'n', long, help = "Dry run - show what would be changed without modifying files")]
    dry_run: bool,

    #[arg(short, long, help = "Verbose output")]
    verbose: bool,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let regex = Regex::new(&args.pattern)
        .with_context(|| format!("Invalid regex pattern: {}", args.pattern))?;

    let extensions: Option<Vec<&str>> = args.extensions.as_ref().map(|ext| {
        ext.split(',').collect()
    });

    let ignore_set = build_ignore_set(&args.directory)?;

    let mut total_files = 0;
    let mut modified_files = 0;

    for entry in WalkDir::new(&args.directory)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let path = entry.path();
        
        if !path.is_file() {
            continue;
        }

        if should_ignore(path, &args.directory, &ignore_set) {
            if args.verbose {
                println!("Ignoring: {}", path.display());
            }
            continue;
        }

        if let Some(ref exts) = extensions {
            let has_valid_extension = path.extension()
                .and_then(|ext| ext.to_str())
                .map(|ext| exts.contains(&ext))
                .unwrap_or(false);
            
            if !has_valid_extension {
                continue;
            }
        }

        match process_file(path, &regex, &args.replace, args.dry_run, args.verbose) {
            Ok(modified) => {
                total_files += 1;
                if modified {
                    modified_files += 1;
                }
            }
            Err(e) => {
                eprintln!("Error processing {}: {}", path.display(), e);
            }
        }
    }

    println!("\nSummary:");
    println!("Total files processed: {}", total_files);
    println!("Files modified: {}", modified_files);
    if args.dry_run {
        println!("(Dry run - no files were actually modified)");
    }

    Ok(())
}

fn process_file(path: &Path, regex: &Regex, replacement: &str, dry_run: bool, verbose: bool) -> Result<bool> {
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
                    println!("  Match {}: \"{}\" -> \"{}\"", 
                        i + 1, 
                        &content[mat.start()..mat.end()],
                        replacement
                    );
                }
            }
        }
    }

    if !dry_run {
        fs::write(path, new_content.as_ref())
            .with_context(|| format!("Failed to write file: {}", path.display()))?;
    }

    Ok(true)
}

fn build_ignore_set(working_dir: &str) -> Result<GlobSet> {
    let mut builder = GlobSetBuilder::new();
    
    // Load .rr_ignore from current working directory
    let cwd_ignore = Path::new(".").join(".rr_ignore");
    if cwd_ignore.exists() {
        load_ignore_file(&cwd_ignore, &mut builder)?;
    }
    
    // Load .rr_ignore from target directory
    let local_ignore = Path::new(working_dir).join(".rr_ignore");
    if local_ignore.exists() {
        load_ignore_file(&local_ignore, &mut builder)?;
    }
    
    // Load ~/.rr_ignore from home directory
    if let Ok(home_dir) = std::env::var("HOME") {
        let home_ignore = PathBuf::from(home_dir).join(".rr_ignore");
        if home_ignore.exists() {
            load_ignore_file(&home_ignore, &mut builder)?;
        }
    }
    
    builder.build()
        .with_context(|| "Failed to build ignore pattern set")
}

fn load_ignore_file(path: &Path, builder: &mut GlobSetBuilder) -> Result<()> {
    let content = fs::read_to_string(path)
        .with_context(|| format!("Failed to read ignore file: {}", path.display()))?;
    
    for line in content.lines() {
        let line = line.trim();
        
        // Skip empty lines and comments
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        
        // Add the glob pattern
        let glob = Glob::new(line)
            .with_context(|| format!("Invalid glob pattern in {}: {}", path.display(), line))?;
        builder.add(glob);
    }
    
    Ok(())
}

fn should_ignore(path: &Path, base_dir: &str, ignore_set: &GlobSet) -> bool {
    // Get relative path from base directory
    let relative_path = match path.strip_prefix(base_dir) {
        Ok(rel) => rel,
        Err(_) => return false,
    };
    
    // Check if the path matches any ignore pattern
    ignore_set.is_match(relative_path)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn test_regex_creation() {
        assert!(Regex::new(r"\d+").is_ok());
        assert!(Regex::new(r"[").is_err());
    }

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
