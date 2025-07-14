use anyhow::{Context, Result};
use clap::Parser;
use globset::{Glob, GlobSet, GlobSetBuilder};
use regex::Regex;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use walkdir::{DirEntry, WalkDir};

mod file_processor;
use file_processor::process_file;

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

    #[arg(long, help = "Include hidden files and directories")]
    include_hidden: bool,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let regex = Regex::new(&args.pattern)
        .with_context(|| format!("Invalid regex pattern: {}", args.pattern))?;

    let extensions: Option<Vec<&str>> = args.extensions.as_ref().map(|ext| {
        ext.split(',').collect()
    });

    let ignore_set = Arc::new(build_ignore_set(&args.directory)?);
    let base_dir = std::fs::canonicalize(&args.directory)?;

    let mut total_files = 0;
    let mut modified_files = 0;

    let walker = WalkDir::new(&args.directory)
        .into_iter()
        .filter_entry(|e| args.include_hidden || !is_hidden(e))
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file());

    for entry in walker {
        let path = entry.path();

        if should_ignore_fast(path, &base_dir, &ignore_set) {
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


fn build_ignore_set(working_dir: &str) -> Result<GlobSet> {
    let mut builder = GlobSetBuilder::new();
    
    // Add some common patterns by default for better performance
    let default_patterns = [".git/**", ".svn/**", "target/**", "node_modules/**"];
    for pattern in &default_patterns {
        if let Ok(glob) = Glob::new(pattern) {
            builder.add(glob);
        }
    }
    
    // Load .rr_ignore from current working directory
    let cwd_ignore = Path::new(".").join(".rr_ignore");
    if cwd_ignore.exists() {
        load_ignore_file(&cwd_ignore, &mut builder)?;
    }
    
    // Load .rr_ignore from target directory
    if working_dir != "." {
        let local_ignore = Path::new(working_dir).join(".rr_ignore");
        if local_ignore.exists() && local_ignore != cwd_ignore {
            load_ignore_file(&local_ignore, &mut builder)?;
        }
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

fn should_ignore_fast(path: &Path, base_dir: &Path, ignore_set: &Arc<GlobSet>) -> bool {
    // Get relative path from base directory
    let relative_path = match path.strip_prefix(base_dir) {
        Ok(rel) => rel,
        Err(_) => return false,
    };
    
    // Check if the path matches any ignore pattern
    ignore_set.is_match(relative_path)
}

fn is_hidden(entry: &DirEntry) -> bool {
    entry.file_name()
         .to_str()
         .map(|s| s.starts_with('.'))
         .unwrap_or(false)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_regex_creation() {
        assert!(Regex::new(r"\d+").is_ok());
        assert!(Regex::new(r"[").is_err());
    }
}
