use colored::*;
use console::Term;
use dialoguer::{theme::ColorfulTheme, Confirm, Input, MultiSelect, Select};
use std::process::Command;

// Global variable to store current session exclude list
static mut CURRENT_EXCLUDES: Option<Vec<String>> = None;

fn main() {
    let term = Term::stdout();
    term.clear_screen().unwrap();

    show_banner();
    show_main_menu();
}

fn show_banner() {
    println!("{}", r#"
    ███████╗██╗██╗     ███████╗    ███████╗██╗███╗   ██╗██████╗ ███████╗██████╗ 
    ██╔════╝██║██║     ██╔════╝    ██╔════╝██║████╗  ██║██╔══██╗██╔════╝██╔══██╗
    █████╗  ██║██║     █████╗      █████╗  ██║██╔██╗ ██║██║  ██║█████╗  ██████╔╝
    ██╔══╝  ██║██║     ██╔══╝      ██╔══╝  ██║██║╚██╗██║██║  ██║██╔══╝  ██╔══██╗
    ██║     ██║███████╗███████╗    ██║     ██║██║ ╚████║██████╔╝███████╗██║  ██║
    ╚═╝     ╚═╝╚══════╝╚══════╝    ╚═╝     ╚═╝╚═╝  ╚═══╝╚═════╝ ╚══════╝╚═╝  ╚═╝
    "#.bright_cyan().bold());
    
    println!("{}", "    ═══════════════════════════════════════════════════════════════════════".bright_blue());
    println!("{}", "                BLAZING FAST • BEAUTIFUL • POWERFUL".bright_green().bold());
    println!("{}", "    ═══════════════════════════════════════════════════════════════════════".bright_blue());
    println!();
    
    // Add some sparkle effects
    println!("{}", "        Find files instantly with regex and pattern matching".bright_magenta());
    println!("{}", "        Grep through millions of lines at lightning speed".bright_yellow());
    println!("{}", "        Beautiful colored output with file type icons".bright_cyan());
    println!("{}", "        Fully customizable with exclude patterns".bright_white());
    println!();
    
    // Add animated-style border
    println!("{}", "    ▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓".bright_green());
    println!();
}

fn show_main_menu() {
    let options = vec![
        "Grep Search - Find text patterns in files",
        "File Search - Find files by name",
        "Configure Default Settings",
        "Help & Examples",
        "Exit",
    ];

    println!("{}", "    ┌─────────────────────────────────────────────────────────┐".bright_blue());
    println!("{}", "    │                 CHOOSE YOUR MISSION                     │".bright_yellow().bold());
    println!("{}", "    └─────────────────────────────────────────────────────────┘".bright_blue());
    println!();

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("What would you like to do?")
        .default(0)
        .items(&options[..])
        .interact();

    match selection {
        Ok(0) => grep_search_wizard(),
        Ok(1) => file_search_wizard(),
        Ok(2) => configure_settings(),
        Ok(3) => show_help(),
        Ok(4) => {
            println!();
            println!("{}", "    ╔══════════════════════════════════════════════════════════╗".bright_blue());
            println!("{}", "    ║     Thanks for using FFinder! Stay blazing fast!       ║".bright_yellow().bold());
            println!("{}", "    ║              Happy coding!                              ║".bright_green());
            println!("{}", "    ╚══════════════════════════════════════════════════════════╝".bright_blue());
            println!();
            std::process::exit(0);
        }
        Err(_) => {
            println!();
            println!("{}", "    ╔════════════════════════════════════════════════════════════════╗".bright_red());
            println!("{}", "    ║ Interactive mode not supported in this terminal            ║".bright_red());
            println!("{}", "    ║ Use direct commands instead:                               ║".bright_yellow());
            println!("{}", "    ║    ff grep 'pattern' -d /path                              ║".bright_white());
            println!("{}", "    ║    ff find 'filename' -d /path                             ║".bright_white());
            println!("{}", "    ╚════════════════════════════════════════════════════════════════╝".bright_red());
            std::process::exit(1);
        }
        _ => unreachable!(),
    }
}

fn grep_search_wizard() {
    let term = Term::stdout();
    term.clear_screen().unwrap();
    
    println!("{}", "\nGREP SEARCH WIZARD".bright_cyan().bold());
    println!("{}", "═".repeat(50).bright_blue());

    let pattern: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter search pattern")
        .interact_text()
        .unwrap();

    let directory: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter directory to search")
        .default(".".to_string())
        .interact_text()
        .unwrap();

    let case_insensitive = Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Case insensitive search?")
        .default(false)
        .interact()
        .unwrap();

    let use_regex = Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Use regex pattern?")
        .default(false)
        .interact()
        .unwrap();

    let use_progress = Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Show detailed progress with file names?")
        .default(true)
        .interact()
        .unwrap();

    let extensions: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("File extensions (e.g., rs,py,js) or press Enter for all")
        .allow_empty(true)
        .interact_text()
        .unwrap();

    let thread_count: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Number of threads (1-16, or press Enter for auto)")
        .allow_empty(true)
        .interact_text()
        .unwrap();

    let exclude_common = Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Exclude common directories (node_modules, target, build, etc.)?")
        .default(false)
        .interact()
        .unwrap();

    let additional_excludes: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Additional directories to exclude (comma-separated)")
        .allow_empty(true)
        .interact_text()
        .unwrap();

    execute_grep_search(
        &pattern,
        &directory,
        case_insensitive,
        use_regex,
        use_progress,
        if extensions.is_empty() {
            None
        } else {
            Some(&extensions)
        },
        if thread_count.is_empty() {
            None
        } else {
            Some(&thread_count)
        },
        exclude_common,
        if additional_excludes.is_empty() {
            None
        } else {
            Some(&additional_excludes)
        },
    );
}

fn file_search_wizard() {
    let term = Term::stdout();
    term.clear_screen().unwrap();
    
    println!("{}", "\nFILE SEARCH WIZARD".bright_cyan().bold());
    println!("{}", "═".repeat(50).bright_blue());

    let filename: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter filename pattern")
        .interact_text()
        .unwrap();

    let directory: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter directory to search")
        .default(".".to_string())
        .interact_text()
        .unwrap();

    let case_insensitive = Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Case insensitive search?")
        .default(false)
        .interact()
        .unwrap();

    let use_regex = Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Use regex pattern?")
        .default(false)
        .interact()
        .unwrap();

    execute_file_search(&filename, &directory, case_insensitive, use_regex);
}

fn configure_settings() {
    let term = Term::stdout();
    term.clear_screen().unwrap();
    
    println!("{}", "\nCONFIGURATION SETTINGS".bright_magenta().bold());
    println!("{}", "═".repeat(50).bright_blue());

    let settings = vec![
        "Output Color Theme",
        "Default Thread Count",
        "Default Exclude Directories",
        "Progress Display Options",
        "Back to Main Menu",
    ];

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("What would you like to configure?")
        .default(0)
        .items(&settings[..])
        .interact()
        .unwrap();

    match selection {
        0 => configure_colors(),
        1 => configure_threads(),
        2 => configure_excludes(),
        3 => configure_progress(),
        4 => show_main_menu(),
        _ => unreachable!(),
    }
}

fn configure_colors() {
    println!("{}", "\nColor themes coming soon!".bright_yellow());
    println!("Press Enter to continue...");
    std::io::stdin().read_line(&mut String::new()).unwrap();
    show_main_menu();
}

fn configure_threads() {
    println!(
        "{}",
        "\nThread configuration coming soon!".bright_yellow()
    );
    println!("Press Enter to continue...");
    std::io::stdin().read_line(&mut String::new()).unwrap();
    show_main_menu();
}

fn configure_excludes() {
    let term = Term::stdout();
    term.clear_screen().unwrap();
    
    println!("{}", "\nEXCLUDE DIRECTORIES MANAGER".bright_red().bold());
    println!("{}", "═".repeat(60).bright_blue());
    
    // Show ASCII art for exclude feature
    println!("{}", r#"
    ╔════════════════════════════════════════════════════════════╗
    ║  Choose which directories to EXCLUDE during searches    ║
    ║  All are selected by default • Deselect to search in   ║
    ║  Fewer excludes = More thorough search                 ║
    ╚════════════════════════════════════════════════════════════╝
    "#.bright_cyan());

    // Get all common directories with descriptions
    let all_directories = get_all_common_directories();
    
    println!("{}", "\nDirectories to EXCLUDE from searches:".bright_yellow());
    println!("{}", "   (All selected by default - DESELECT ones you want to search in)".bright_cyan());
    println!("{}", "   (Use SPACE to select/deselect, ENTER to confirm)".bright_cyan());

    // All directories are selected by default
    let defaults: Vec<bool> = vec![true; all_directories.len()];

    let selected = MultiSelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Choose directories to exclude (deselect to search in them)")
        .items(&all_directories[..])
        .defaults(&defaults[..])  // All selected by default
        .interact()
        .unwrap();

    // Store the current exclude list for this session
    let mut current_excludes = Vec::new();
    for &index in &selected {
        let dir_name = all_directories[index].split(" - ").next().unwrap();
        current_excludes.push(dir_name.to_string());
    }
    
    unsafe {
        CURRENT_EXCLUDES = Some(current_excludes.clone());
    }

    if selected.len() == all_directories.len() {
        println!("{}", "\nAll directories will be excluded (default behavior)".bright_red());
    } else if selected.is_empty() {
        println!("{}", "\nNo directories will be excluded (searching everywhere including node_modules, target, etc.)".bright_green());
    } else {
        let excluded_count = selected.len();
        let included_count = all_directories.len() - excluded_count;
        
        println!("{}", format!("\n{} directories will be excluded", excluded_count).bright_red());
        println!("{}", format!("{} directories will be SEARCHED (including):", included_count).bright_green());
        
        // Show which directories will be searched in
        for (index, _dir) in all_directories.iter().enumerate() {
            if !selected.contains(&index) {
                let dir_name = all_directories[index].split(" - ").next().unwrap();
                println!("   • {}", dir_name.bright_green());
            }
        }
        println!("{}", "\nThis configuration is active for current session only!".bright_cyan());
    }
    
    println!("\nPress Enter to continue...");
    std::io::stdin().read_line(&mut String::new()).unwrap();
    show_main_menu();
}

fn get_all_common_directories() -> Vec<String> {
    vec![
        "node_modules - Node.js dependencies (can be huge!)".to_string(),
        "target - Rust build directory".to_string(),
        "build - General build output directories".to_string(),
        "_build - Alternative build directory".to_string(),
        "dist - Distribution/bundled files".to_string(),
        ".git - Git version control metadata".to_string(),
        ".svn - Subversion version control".to_string(),
        "__pycache__ - Python bytecode cache".to_string(),
        ".pytest_cache - Pytest cache files".to_string(),
        "venv - Python virtual environment".to_string(),
        ".venv - Python virtual environment (hidden)".to_string(),
        ".env - Environment virtual directory".to_string(),
        ".idea - IntelliJ IDEA settings".to_string(),
        ".vscode - VS Code settings".to_string(),
        "deps - Dependencies directory".to_string(),
        "_deps - Alternative dependencies directory".to_string(),
        ".cache - General cache directory".to_string(),
        "vendor - Third-party code/libraries".to_string(),
        "coverage - Code coverage reports".to_string(),
        "tmp - Temporary files".to_string(),
        ".tmp - Temporary files (hidden)".to_string(),
        ".next - Next.js build cache".to_string(),
        ".nuxt - Nuxt.js build cache".to_string(),
        ".output - Build output directory".to_string(),
        "public - Public static files (usually skip)".to_string(),
        "static - Static files directory".to_string(),
        "assets - Asset files directory".to_string(),
        ".DS_Store - macOS system files".to_string(),
        "Thumbs.db - Windows thumbnail cache".to_string(),
        ".sass-cache - Sass compilation cache".to_string(),
        ".webpack - Webpack cache".to_string(),
        ".parcel-cache - Parcel bundler cache".to_string(),
        "bower_components - Bower dependencies".to_string(),
        "jspm_packages - JSPM packages".to_string(),
        ".nyc_output - NYC coverage output".to_string(),
        "logs - Log files directory".to_string(),
        "*.log - Log files".to_string(),
        ".log - Log directory".to_string(),
        "temp - Temporary directory".to_string(),
        ".temp - Temporary directory (hidden)".to_string(),
        ".yarn - Yarn cache directory".to_string(),
        "node - Node directory".to_string(),
        ".npm - NPM cache".to_string(),
        ".cargo - Cargo registry cache".to_string(),
        ".rustup - Rustup toolchain".to_string(),
    ]
}

fn get_default_exclude_directories() -> Vec<String> {
    vec![
        "node_modules".to_string(),
        "target".to_string(),
        "build".to_string(),
        "_build".to_string(),
        "dist".to_string(),
        ".git".to_string(),
        ".svn".to_string(),
        "__pycache__".to_string(),
        ".pytest_cache".to_string(),
        "venv".to_string(),
        ".venv".to_string(),
        ".env".to_string(),
        ".idea".to_string(),
        ".vscode".to_string(),
        "deps".to_string(),
        "_deps".to_string(),
        ".cache".to_string(),
        "vendor".to_string(),
        "coverage".to_string(),
        "tmp".to_string(),
        ".tmp".to_string(),
        ".next".to_string(),
        ".nuxt".to_string(),
        ".output".to_string(),
        "public".to_string(),
        "static".to_string(),
        "assets".to_string(),
        ".DS_Store".to_string(),
        "Thumbs.db".to_string(),
        ".sass-cache".to_string(),
        ".webpack".to_string(),
        ".parcel-cache".to_string(),
        "bower_components".to_string(),
        "jspm_packages".to_string(),
        ".nyc_output".to_string(),
        "logs".to_string(),
        "*.log".to_string(),
        ".log".to_string(),
        "temp".to_string(),
        ".temp".to_string(),
        ".yarn".to_string(),
        "node".to_string(),
        ".npm".to_string(),
        ".cargo".to_string(),
        ".rustup".to_string(),
    ]
}

fn configure_progress() {
    println!("{}", "\nPROGRESS DISPLAY OPTIONS".bright_blue().bold());

    let options = vec![
        "Minimal - Just progress bar",
        "Standard - Progress + file count",
        "Detailed - Progress + current file names",
        "Verbose - All details + performance metrics",
    ];

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Choose progress display level")
        .default(1)
        .items(&options[..])
        .interact()
        .unwrap();

    println!(
        "{}",
        format!("Progress display set to: {}", options[selection]).bright_green()
    );
    println!("Press Enter to continue...");
    std::io::stdin().read_line(&mut String::new()).unwrap();
    show_main_menu();
}

fn show_help() {
    println!("{}", "\nHELP & EXAMPLES".bright_blue().bold());
    println!("{}", "═".repeat(60).bright_blue());

    println!("{}", "\nCOMMAND LINE USAGE:".bright_yellow());
    println!("  ffinder                    # Launch interactive mode");
    println!("  ff grep 'pattern'          # Direct grep search");
    println!("  ff find 'name'             # Direct file search");

    println!("{}", "\nGREP EXAMPLES:".bright_yellow());
    println!("  ff grep 'main()' -d /projects");
    println!("  ff grep 'TODO|FIXME' -r -i");
    println!("  ff grep 'error' -e 'log,txt' -t 8");
    println!("  ff grep 'function' -x 'node_modules,build'");

    println!("{}", "\nFILE SEARCH EXAMPLES:".bright_yellow());
    println!("  ff find '*.rs' -r");
    println!("  ff find 'config' -i");
    println!("  ff find 'main.cpp' -d /projects");

    println!("{}", "\nOPTIONS:".bright_yellow());
    println!("  -d, --dir <PATH>        Directory to search");
    println!("  -i, --ignore-case       Case insensitive");
    println!("  -r, --regex             Use regex patterns");
    println!("  -e, --ext <EXTS>        File extensions");
    println!("  -t, --threads <NUM>     Thread count");
    println!("  -x, --exclude <DIRS>    Exclude directories");
    println!("  --progress              Show detailed progress");

    println!("{}", "\nPERFORMANCE TIPS:".bright_yellow());
    println!("  • Use exclude directories for faster searches");
    println!("  • Specify file extensions to reduce scope");
    println!("  • Adjust thread count based on your CPU");
    println!("  • Use regex for complex pattern matching");

    println!("\nPress Enter to continue...");
    std::io::stdin().read_line(&mut String::new()).unwrap();
    show_main_menu();
}

fn execute_grep_search(
    pattern: &str,
    directory: &str,
    case_insensitive: bool,
    use_regex: bool,
    use_progress: bool,
    extensions: Option<&String>,
    thread_count: Option<&String>,
    exclude_common: bool,
    additional_excludes: Option<&String>,
) {
    println!("{}", "\nEXECUTING SEARCH...".bright_green().bold());
    println!("{}", "═".repeat(50).bright_blue());

    let mut args = vec!["grep", pattern, "-d", directory];

    if case_insensitive {
        args.push("-i");
    }

    if use_regex {
        args.push("-r");
    }

    if use_progress {
        args.push("--progress");
    }

    if let Some(ext) = extensions {
        if !ext.is_empty() {
            args.extend_from_slice(&["-e", ext]);
        }
    }

    if let Some(threads) = thread_count {
        if !threads.is_empty() {
            args.extend_from_slice(&["-t", threads]);
        }
    }

    // Handle exclusions
    let exclude_str;
    if exclude_common || (additional_excludes.is_some() && !additional_excludes.unwrap().is_empty())
    {
        let mut exclude_list = Vec::new();
        if exclude_common {
            // Use session-specific exclude list if configured, otherwise use default
            unsafe {
                if let Some(ref current_excludes) = CURRENT_EXCLUDES {
                    exclude_list.extend_from_slice(current_excludes);
                } else {
                    // Default exclude list if not configured
                    exclude_list.extend(get_default_exclude_directories());
                }
            }
        }

        if let Some(additional) = additional_excludes {
            if !additional.is_empty() {
                for dir in additional.split(',') {
                    exclude_list.push(dir.trim().to_string());
                }
            }
        }

        exclude_str = exclude_list.join(",");
        args.push("-x");
        args.push(&exclude_str);
    }

    let result = Command::new("file_finder")
        .args(&args)
        .status();

    match result {
        Ok(status) => {
            if status.success() {
                println!("{}", "\nSearch completed successfully!".bright_green());
            } else {
                println!("{}", "\nSearch failed!".bright_red());
            }
        }
        Err(e) => {
            println!("{}", format!("\nFailed to execute: {}", e).bright_red());
            println!(
                "{}",
                "Make sure FFinder is properly installed".bright_yellow()
            );
        }
    }

    println!("\nPress Enter to return to main menu...");
    std::io::stdin().read_line(&mut String::new()).unwrap();
    show_main_menu();
}

fn execute_file_search(filename: &str, directory: &str, case_insensitive: bool, use_regex: bool) {
    println!("{}", "\nEXECUTING FILE SEARCH...".bright_cyan().bold());
    println!("{}", "═".repeat(50).bright_blue());

    let mut args = vec!["find", filename, "-d", directory];

    if case_insensitive {
        args.push("-i");
    }

    if use_regex {
        args.push("-r");
    }

    let result = Command::new("file_finder")
        .args(&args)
        .status();

    match result {
        Ok(status) => {
            if status.success() {
                println!(
                    "{}",
                    "\nFile search completed successfully!".bright_green()
                );
            } else {
                println!("{}", "\nFile search failed!".bright_red());
            }
        }
        Err(e) => {
            println!("{}", format!("\nFailed to execute: {}", e).bright_red());
            println!(
                "{}",
                "Make sure FFinder is properly installed".bright_yellow()
            );
        }
    }

    println!("\nPress Enter to return to main menu...");
    std::io::stdin().read_line(&mut String::new()).unwrap();
    show_main_menu();
}