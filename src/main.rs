use clap::{Arg, ArgMatches, Command};
use colored::*;
use console::Term;
use dialoguer::{theme::ColorfulTheme, Select};

mod file_search;
mod grep_search;
mod utils;

use file_search::FileSearcher;
use grep_search::GrepSearcher;

fn main() {
    let app = Command::new("File Finder & Grep Tool")
        .version("1.0.0")
        .author("Rust CLI Developer")
        .about("A powerful file search and grep tool with beautiful output")
        .subcommand(
            Command::new("find")
                .about("Search for files recursively")
                .arg(
                    Arg::new("filename")
                        .help("The filename to search for")
                        .required(true)
                        .index(1),
                )
                .arg(
                    Arg::new("directory")
                        .help("Directory to search in (default: current directory)")
                        .short('d')
                        .long("dir")
                        .value_name("PATH"),
                )
                .arg(
                    Arg::new("case-insensitive")
                        .help("Case insensitive search")
                        .short('i')
                        .long("ignore-case")
                        .action(clap::ArgAction::SetTrue),
                )
                .arg(
                    Arg::new("regex")
                        .help("Use regex pattern")
                        .short('r')
                        .long("regex")
                        .action(clap::ArgAction::SetTrue),
                ),
        )
        .subcommand(
            Command::new("grep")
                .about("Search for text patterns in files")
                .arg(
                    Arg::new("pattern")
                        .help("The pattern to search for")
                        .required(true)
                        .index(1),
                )
                .arg(
                    Arg::new("directory")
                        .help("Directory to search in (default: current directory)")
                        .short('d')
                        .long("dir")
                        .value_name("PATH"),
                )
                .arg(
                    Arg::new("case-insensitive")
                        .help("Case insensitive search")
                        .short('i')
                        .long("ignore-case")
                        .action(clap::ArgAction::SetTrue),
                )
                .arg(
                    Arg::new("regex")
                        .help("Use regex pattern")
                        .short('r')
                        .long("regex")
                        .action(clap::ArgAction::SetTrue),
                )
                .arg(
                    Arg::new("extensions")
                        .help("File extensions to search (e.g., rs,py,js)")
                        .short('e')
                        .long("ext")
                        .value_name("EXTENSIONS"),
                ),
        );

    let matches = app.get_matches();

    match matches.subcommand() {
        Some(("find", sub_matches)) => handle_file_search(sub_matches),
        Some(("grep", sub_matches)) => handle_grep_search(sub_matches),
        _ => show_interactive_menu(),
    }
}

fn handle_file_search(matches: &ArgMatches) {
    let filename = matches.get_one::<String>("filename").unwrap();
    let directory = matches
        .get_one::<String>("directory")
        .map(|s| s.as_str())
        .unwrap_or(".");
    let case_insensitive = matches.get_flag("case-insensitive");
    let use_regex = matches.get_flag("regex");

    println!("{}", "🔍 File Search Mode".bright_cyan().bold());
    println!("{}", "═".repeat(50).bright_blue());

    let searcher = FileSearcher::new(directory, case_insensitive, use_regex);
    searcher.search(filename);
}

fn handle_grep_search(matches: &ArgMatches) {
    let pattern = matches.get_one::<String>("pattern").unwrap();
    let directory = matches
        .get_one::<String>("directory")
        .map(|s| s.as_str())
        .unwrap_or(".");
    let case_insensitive = matches.get_flag("case-insensitive");
    let use_regex = matches.get_flag("regex");
    let extensions = matches.get_one::<String>("extensions");

    println!("{}", "🔎 Grep Search Mode".bright_green().bold());
    println!("{}", "═".repeat(50).bright_blue());

    let searcher = GrepSearcher::new(directory, case_insensitive, use_regex, extensions);
    searcher.search(pattern);
}

fn show_interactive_menu() {
    let term = Term::stdout();
    term.clear_screen().unwrap();

    println!("{}", "🚀 File Finder & Grep Tool".bright_magenta().bold());
    println!("{}", "═".repeat(60).bright_blue());
    println!();

    let options = vec![
        "🔍 Search for files",
        "🔎 Search for text patterns (grep)",
        "❌ Exit",
    ];

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Choose an option:")
        .default(0)
        .items(&options[..])
        .interact()
        .unwrap();

    match selection {
        0 => interactive_file_search(),
        1 => interactive_grep_search(),
        2 => {
            println!("{}", "👋 Goodbye!".bright_yellow());
            std::process::exit(0);
        }
        _ => unreachable!(),
    }
}

fn interactive_file_search() {
    println!("{}", "\n🔍 Interactive File Search".bright_cyan().bold());
    
    let filename: String = dialoguer::Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter filename to search")
        .interact_text()
        .unwrap();

    let directory: String = dialoguer::Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter directory (or press Enter for current)")
        .default(".".to_string())
        .interact_text()
        .unwrap();

    let case_insensitive = dialoguer::Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Case insensitive search?")
        .default(false)
        .interact()
        .unwrap();

    let use_regex = dialoguer::Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Use regex pattern?")
        .default(false)
        .interact()
        .unwrap();

    println!();
    let searcher = FileSearcher::new(&directory, case_insensitive, use_regex);
    searcher.search(&filename);
}

fn interactive_grep_search() {
    println!("{}", "\n🔎 Interactive Grep Search".bright_green().bold());
    
    let pattern: String = dialoguer::Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter search pattern")
        .interact_text()
        .unwrap();

    let directory: String = dialoguer::Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter directory (or press Enter for current)")
        .default(".".to_string())
        .interact_text()
        .unwrap();

    let case_insensitive = dialoguer::Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Case insensitive search?")
        .default(false)
        .interact()
        .unwrap();

    let use_regex = dialoguer::Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Use regex pattern?")
        .default(false)
        .interact()
        .unwrap();

    let extensions: Option<String> = dialoguer::Input::with_theme(&ColorfulTheme::default())
        .with_prompt("File extensions (e.g., rs,py,js) or press Enter for all")
        .allow_empty(true)
        .interact_text()
        .ok();

    println!();
    let searcher = GrepSearcher::new(&directory, case_insensitive, use_regex, extensions.as_ref());
    searcher.search(&pattern);
}
