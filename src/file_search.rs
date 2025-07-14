use colored::*;
use indicatif::{ProgressBar, ProgressStyle};
use regex::Regex;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::SystemTime;
use walkdir::WalkDir;

use crate::utils::{format_file_size, format_modified_time, get_file_icon};

pub struct FileSearcher {
    directory: PathBuf,
    case_insensitive: bool,
    use_regex: bool,
}

impl FileSearcher {
    pub fn new(directory: &str, case_insensitive: bool, use_regex: bool) -> Self {
        Self {
            directory: PathBuf::from(directory),
            case_insensitive,
            use_regex,
        }
    }

    pub fn search(&self, pattern: &str) {
        println!(
            "{} Searching for: {}",
            "🔍".bright_yellow(),
            pattern.bright_white().bold()
        );
        println!(
            "{} Directory: {}",
            "📁".bright_blue(),
            self.directory.display().to_string().bright_cyan()
        );
        println!(
            "{} Options: {} | {}",
            "⚙️".bright_magenta(),
            if self.case_insensitive {
                "Case Insensitive".green()
            } else {
                "Case Sensitive".red()
            },
            if self.use_regex {
                "Regex".green()
            } else {
                "Literal".yellow()
            }
        );
        println!("{}", "─".repeat(80).bright_black());

        // Create progress bar
        let pb = ProgressBar::new_spinner();
        pb.set_style(
            ProgressStyle::default_spinner()
                .template("{spinner:.green} {msg}")
                .unwrap()
                .tick_chars("⠁⠂⠄⡀⢀⠠⠐⠈ "),
        );
        pb.set_message("Scanning directories...");

        let mut matches = Vec::new();
        let mut total_files = 0;

        // Prepare search pattern
        let search_pattern = if self.use_regex {
            match Regex::new(pattern) {
                Ok(regex) => Some(regex),
                Err(e) => {
                    println!("{} Invalid regex pattern: {}", "❌".red(), e);
                    return;
                }
            }
        } else {
            None
        };

        // Walk through directory
        for entry in WalkDir::new(&self.directory).into_iter().filter_map(|e| e.ok()) {
            total_files += 1;
            if total_files % 100 == 0 {
                pb.set_message(format!("Scanned {} files...", total_files));
            }

            if entry.file_type().is_file() {
                let filename = entry.file_name().to_string_lossy();
                let is_match = if let Some(ref regex) = search_pattern {
                    regex.is_match(&filename)
                } else if self.case_insensitive {
                    filename.to_lowercase().contains(&pattern.to_lowercase())
                } else {
                    filename.contains(pattern)
                };

                if is_match {
                    matches.push(entry.path().to_path_buf());
                }
            }
        }

        pb.finish_and_clear();

        // Display results
        self.display_results(&matches, pattern);
    }

    fn display_results(&self, matches: &[PathBuf], pattern: &str) {
        if matches.is_empty() {
            println!(
                "{} No files found matching: {}",
                "😔".bright_red(),
                pattern.bright_white().bold()
            );
            return;
        }

        println!(
            "{} Found {} match{}:",
            "🎉".bright_green(),
            matches.len().to_string().bright_yellow().bold(),
            if matches.len() == 1 { "" } else { "es" }
        );
        println!("{}", "═".repeat(80).bright_blue());

        for (index, path) in matches.iter().enumerate() {
            self.display_file_info(path, index + 1);
            if index < matches.len() - 1 {
                println!("{}", "─".repeat(80).bright_black());
            }
        }

        println!("{}", "═".repeat(80).bright_blue());
        println!(
            "{} Search completed. Found {} file{}.",
            "✅".bright_green(),
            matches.len().to_string().bright_yellow().bold(),
            if matches.len() == 1 { "" } else { "s" }
        );
    }

    fn display_file_info(&self, path: &Path, index: usize) {
        let metadata = match fs::metadata(path) {
            Ok(meta) => meta,
            Err(_) => {
                println!(
                    "{} {}. {} {}",
                    "❌".red(),
                    index.to_string().bright_white().bold(),
                    get_file_icon(path),
                    path.display().to_string().bright_red()
                );
                return;
            }
        };

        let filename = path.file_name().unwrap().to_string_lossy();
        let directory = path.parent().unwrap_or(Path::new("")).display();
        let size = format_file_size(metadata.len());
        let modified = format_modified_time(metadata.modified().unwrap_or(SystemTime::UNIX_EPOCH));

        println!(
            "{} {}. {} {}",
            "📄".bright_blue(),
            index.to_string().bright_white().bold(),
            get_file_icon(path),
            filename.bright_white().bold()
        );
        
        println!(
            "   {} {}",
            "📍".bright_yellow(),
            directory.to_string().bright_cyan()
        );
        
        println!(
            "   {} {}  {} {}  {} {}",
            "📏".bright_magenta(),
            size.bright_white(),
            "🕒".bright_green(),
            modified.bright_white(),
            "🔗".bright_blue(),
            path.display().to_string().dimmed()
        );
    }
}
