use colored::*;
use indicatif::{ProgressBar, ProgressStyle};
use regex::Regex;
use std::collections::HashSet;
use std::fs;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

use crate::utils::{get_file_icon, is_binary_file, should_search_file};

pub struct GrepSearcher {
    directory: PathBuf,
    case_insensitive: bool,
    use_regex: bool,
    extensions: Option<HashSet<String>>,
}

#[derive(Debug)]
pub struct Match {
    file_path: PathBuf,
    line_number: usize,
    line_content: String,
    match_start: usize,
    match_end: usize,
}

impl GrepSearcher {
    pub fn new(
        directory: &str,
        case_insensitive: bool,
        use_regex: bool,
        extensions: Option<&String>,
    ) -> Self {
        let extensions = extensions.map(|ext_str| {
            ext_str
                .split(',')
                .map(|e| e.trim().to_lowercase())
                .collect::<HashSet<String>>()
        });

        Self {
            directory: PathBuf::from(directory),
            case_insensitive,
            use_regex,
            extensions,
        }
    }

    pub fn search(&self, pattern: &str) {
        println!(
            "{} Searching for pattern: {}",
            "🔎".bright_yellow(),
            pattern.bright_white().bold()
        );
        println!(
            "{} Directory: {}",
            "📁".bright_blue(),
            self.directory.display().to_string().bright_cyan()
        );
        
        let ext_info = if let Some(ref exts) = self.extensions {
            format!("Extensions: {}", exts.iter().cloned().collect::<Vec<_>>().join(", ")).bright_magenta()
        } else {
            "All files".bright_magenta()
        };
        
        println!(
            "{} Options: {} | {} | {}",
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
            },
            ext_info
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
        pb.set_message("Scanning files...");

        let mut all_matches = Vec::new();
        let mut files_searched = 0;

        // Prepare search pattern
        let search_regex = if self.use_regex {
            match Regex::new(pattern) {
                Ok(regex) => regex,
                Err(e) => {
                    println!("{} Invalid regex pattern: {}", "❌".red(), e);
                    return;
                }
            }
        } else {
            let escaped_pattern = regex::escape(pattern);
            let final_pattern = if self.case_insensitive {
                format!("(?i){}", escaped_pattern)
            } else {
                escaped_pattern
            };
            match Regex::new(&final_pattern) {
                Ok(regex) => regex,
                Err(e) => {
                    println!("{} Error creating search pattern: {}", "❌".red(), e);
                    return;
                }
            }
        };

        // Walk through directory
        for entry in WalkDir::new(&self.directory).into_iter().filter_map(|e| e.ok()) {
            if entry.file_type().is_file() {
                let path = entry.path();
                
                if !should_search_file(path, &self.extensions) || is_binary_file(path) {
                    continue;
                }

                files_searched += 1;
                if files_searched % 50 == 0 {
                    pb.set_message(format!("Searched {} files...", files_searched));
                }

                if let Ok(file_matches) = self.search_in_file(path, &search_regex) {
                    all_matches.extend(file_matches);
                }
            }
        }

        pb.finish_and_clear();

        // Display results
        self.display_results(&all_matches, pattern, files_searched);
    }

    fn search_in_file(&self, path: &Path, regex: &Regex) -> Result<Vec<Match>, std::io::Error> {
        let file = fs::File::open(path)?;
        let reader = BufReader::new(file);
        let mut matches = Vec::new();

        for (line_number, line_result) in reader.lines().enumerate() {
            let line = line_result?;
            
            for mat in regex.find_iter(&line) {
                matches.push(Match {
                    file_path: path.to_path_buf(),
                    line_number: line_number + 1,
                    line_content: line.clone(),
                    match_start: mat.start(),
                    match_end: mat.end(),
                });
            }
        }

        Ok(matches)
    }

    fn display_results(&self, matches: &[Match], pattern: &str, files_searched: usize) {
        if matches.is_empty() {
            println!(
                "{} No matches found for pattern: {} (searched {} files)",
                "😔".bright_red(),
                pattern.bright_white().bold(),
                files_searched.to_string().bright_yellow()
            );
            return;
        }

        // Group matches by file
        let mut file_matches: std::collections::HashMap<&Path, Vec<&Match>> =
            std::collections::HashMap::new();
        for m in matches {
            file_matches.entry(&m.file_path).or_default().push(m);
        }

        println!(
            "{} Found {} match{} in {} file{} (searched {} files):",
            "🎉".bright_green(),
            matches.len().to_string().bright_yellow().bold(),
            if matches.len() == 1 { "" } else { "es" },
            file_matches.len().to_string().bright_cyan().bold(),
            if file_matches.len() == 1 { "" } else { "s" },
            files_searched.to_string().bright_white()
        );
        println!("{}", "═".repeat(80).bright_blue());

        let mut file_index = 1;
        for (file_path, file_match_list) in &file_matches {
            self.display_file_matches(file_path, file_match_list, file_index);
            file_index += 1;
            
            if file_index <= file_matches.len() {
                println!("{}", "─".repeat(80).bright_black());
            }
        }

        println!("{}", "═".repeat(80).bright_blue());
        println!(
            "{} Search completed. Found {} match{} in {} file{}.",
            "✅".bright_green(),
            matches.len().to_string().bright_yellow().bold(),
            if matches.len() == 1 { "" } else { "es" },
            file_matches.len().to_string().bright_cyan().bold(),
            if file_matches.len() == 1 { "" } else { "s" }
        );
    }

    fn display_file_matches(&self, file_path: &Path, matches: &[&Match], file_index: usize) {
        let filename = file_path.file_name().unwrap().to_string_lossy();
        let directory = file_path.parent().unwrap_or(Path::new("")).display();

        println!(
            "{} {}. {} {} ({} match{})",
            "📄".bright_blue(),
            file_index.to_string().bright_white().bold(),
            get_file_icon(file_path),
            filename.bright_white().bold(),
            matches.len().to_string().bright_yellow(),
            if matches.len() == 1 { "" } else { "es" }
        );
        
        println!(
            "   {} {}",
            "📍".bright_yellow(),
            directory.to_string().bright_cyan()
        );

        // Show matches with context
        for (i, m) in matches.iter().enumerate() {
            self.display_match(m, i + 1);
        }
    }

    fn display_match(&self, m: &Match, match_index: usize) {
        let line_num_str = format!("{:4}", m.line_number);
        
        // Highlight the matched text
        let before_match = &m.line_content[..m.match_start];
        let matched_text = &m.line_content[m.match_start..m.match_end];
        let after_match = &m.line_content[m.match_end..];

        println!(
            "     {} {} │ {}{}{}",
            match_index.to_string().bright_magenta(),
            line_num_str.bright_blue(),
            before_match.trim_start(),
            matched_text.on_bright_yellow().black().bold(),
            after_match
        );
    }
}
