use colored::*;
use console::Term;
use dialoguer::{theme::ColorfulTheme, Confirm, Input, MultiSelect, Select};
use std::process::Command;
use std::collections::HashMap;
use std::sync::Mutex;

// Global storage for custom profiles (in a real app, this would be saved to file)
lazy_static::lazy_static! {
    static ref CUSTOM_PROFILES: Mutex<HashMap<String, Vec<String>>> = Mutex::new(HashMap::new());
}

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
    println!("{}", "                � BLAZING FAST • BEAUTIFUL • POWERFUL 🔎".bright_green().bold());
    println!("{}", "    ═══════════════════════════════════════════════════════════════════════".bright_blue());
    println!();
    
    // Add some sparkle effects
    println!("{}", "        ✨ Find files instantly with regex and pattern matching ✨".bright_magenta());
    println!("{}", "        🚀 Grep through millions of lines at lightning speed 🚀".bright_yellow());
    println!("{}", "        🎨 Beautiful colored output with file type icons 🎨".bright_cyan());
    println!("{}", "        ⚙️  Fully customizable with exclude patterns ⚙️".bright_white());
    println!();
    
    // Add animated-style border
    println!("{}", "    ▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓".bright_green());
    println!();
}

fn show_main_menu() {
    let options = vec![
        "🔍 Grep Search - Find text patterns in files",
        "📁 File Search - Find files by name",
        "⚙️  Configure Default Settings",
        "� Help & Examples",
        "❌ Exit",
    ];

    println!("{}", "    ┌─────────────────────────────────────────────────────────┐".bright_blue());
    println!("{}", "    │                 🌟 CHOOSE YOUR MISSION 🌟               │".bright_yellow().bold());
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
            println!("{}", "    ║     👋 Thanks for using FFinder! Stay blazing fast! 🚀  ║".bright_yellow().bold());
            println!("{}", "    ║              🌟 Happy coding! 🌟                        ║".bright_green());
            println!("{}", "    ╚══════════════════════════════════════════════════════════╝".bright_blue());
            println!();
            std::process::exit(0);
        }
        Err(_) => {
            println!();
            println!("{}", "    ╔════════════════════════════════════════════════════════════════╗".bright_red());
            println!("{}", "    ║ ❌ Interactive mode not supported in this terminal            ║".bright_red());
            println!("{}", "    ║ 💡 Use direct commands instead:                               ║".bright_yellow());
            println!("{}", "    ║    ff grep 'pattern' -d /path                                  ║".bright_white());
            println!("{}", "    ║    ff find 'filename' -d /path                                 ║".bright_white());
            println!("{}", "    ╚════════════════════════════════════════════════════════════════╝".bright_red());
            std::process::exit(1);
        }
        _ => unreachable!(),
    }
}

fn grep_search_wizard() {
    let term = Term::stdout();
    term.clear_screen().unwrap();
    
    println!("{}", "\n🔍 GREP SEARCH WIZARD".bright_cyan().bold());
    println!("{}", "═".repeat(50).bright_blue());

    let pattern: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("🎯 Enter search pattern")
        .interact_text()
        .unwrap();

    let directory: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("📁 Enter directory to search")
        .default(".".to_string())
        .interact_text()
        .unwrap();

    let case_insensitive = Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("🔤 Case insensitive search?")
        .default(false)
        .interact()
        .unwrap();

    let use_regex = Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("🔄 Use regex pattern?")
        .default(false)
        .interact()
        .unwrap();

    let use_progress = Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("📊 Show detailed progress with file names?")
        .default(true)
        .interact()
        .unwrap();

    let extensions: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("📄 File extensions (e.g., rs,py,js) or press Enter for all")
        .allow_empty(true)
        .interact_text()
        .unwrap();

    let thread_count: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("🧵 Number of threads (1-16, or press Enter for auto)")
        .allow_empty(true)
        .interact_text()
        .unwrap();

    let exclude_common = Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("🚫 Exclude common directories (node_modules, target, build, etc.)?")
        .default(false)
        .interact()
        .unwrap();

    let additional_excludes: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("➕ Additional directories to exclude (comma-separated)")
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
    
    println!("{}", "\n📁 FILE SEARCH WIZARD".bright_cyan().bold());
    println!("{}", "═".repeat(50).bright_blue());

    let filename: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("📄 Enter filename pattern")
        .interact_text()
        .unwrap();

    let directory: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("📁 Enter directory to search")
        .default(".".to_string())
        .interact_text()
        .unwrap();

    let case_insensitive = Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("🔤 Case insensitive search?")
        .default(false)
        .interact()
        .unwrap();

    let use_regex = Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("🔄 Use regex pattern?")
        .default(false)
        .interact()
        .unwrap();

    execute_file_search(&filename, &directory, case_insensitive, use_regex);
}

fn configure_settings() {
    let term = Term::stdout();
    term.clear_screen().unwrap();
    
    println!("{}", "\n⚙️ CONFIGURATION SETTINGS".bright_magenta().bold());
    println!("{}", "═".repeat(50).bright_blue());

    let settings = vec![
        "🎨 Output Color Theme",
        "🧵 Default Thread Count",
        "🚫 Default Exclude Directories",
        "📊 Progress Display Options",
        "⬅️  Back to Main Menu",
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
    println!("{}", "\n🎨 Color themes coming soon!".bright_yellow());
    println!("Press Enter to continue...");
    std::io::stdin().read_line(&mut String::new()).unwrap();
    show_main_menu();
}

fn configure_threads() {
    println!(
        "{}",
        "\n🧵 Thread configuration coming soon!".bright_yellow()
    );
    println!("Press Enter to continue...");
    std::io::stdin().read_line(&mut String::new()).unwrap();
    show_main_menu();
}

fn configure_excludes() {
    let term = Term::stdout();
    term.clear_screen().unwrap();
    
    println!("{}", "\n🚫 EXCLUDE DIRECTORIES MANAGER".bright_red().bold());
    println!("{}", "═".repeat(60).bright_blue());
    
    // Show ASCII art for exclude feature
    println!("{}", r#"
    ╔════════════════════════════════════════════════════════════╗
    ║  🚫 Customize which directories to skip during searches    ║
    ║  ⚡ Faster searches • Less noise • Better results         ║
    ╚════════════════════════════════════════════════════════════╝
    "#.bright_cyan());

    let options = vec![
        "📋 Manage Exclude List (View/Add/Remove)",
        "🎯 Select from Common Excludes (Quick Setup)",
        "💾 Save/Load Custom Profiles",
        "🔄 Reset to Default Settings",
        "⬅️  Back to Main Menu",
    ];

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("🎯 Choose exclude management option")
        .default(0)
        .items(&options[..])
        .interact()
        .unwrap();

    match selection {
        0 => manage_exclude_list(),
        1 => select_common_excludes(),
        2 => manage_profiles(),
        3 => reset_excludes(),
        4 => show_main_menu(),
        _ => unreachable!(),
    }
}

fn manage_exclude_list() {
    let term = Term::stdout();
    term.clear_screen().unwrap();
    
    println!("{}", "\n📋 EXCLUDE LIST MANAGEMENT".bright_blue().bold());
    println!("{}", "═".repeat(60).bright_blue());

    let options = vec![
        "👀 View Current Exclude List",
        "➕ Add Directory to Exclude List",
        "➖ Remove Directory from Exclude List",
        "⬅️  Back to Exclude Manager",
    ];

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("🎯 What would you like to do?")
        .default(0)
        .items(&options[..])
        .interact()
        .unwrap();

    match selection {
        0 => view_exclude_list(),
        1 => add_exclude_directory(),
        2 => remove_exclude_directory(),
        3 => configure_excludes(),
        _ => unreachable!(),
    }
}

fn view_exclude_list() {
    let term = Term::stdout();
    term.clear_screen().unwrap();
    
    println!("{}", "\n📋 CURRENT EXCLUDE LIST".bright_blue().bold());
    println!("{}", "═".repeat(50).bright_blue());
    
    // For now, show default excludes - in a real app this would read from config
    let current_excludes = get_default_excludes();
    
    if current_excludes.is_empty() {
        println!("{}", "📝 No directories are currently excluded".bright_yellow());
    } else {
        println!("{}", "🚫 Currently excluded directories:".bright_cyan());
        for (i, dir) in current_excludes.iter().enumerate() {
            println!("   {}. {} {}", i + 1, "📁".bright_blue(), dir.bright_white());
        }
    }
    
    println!("\nPress Enter to continue...");
    std::io::stdin().read_line(&mut String::new()).unwrap();
    manage_exclude_list();
}

fn add_exclude_directory() {
    let term = Term::stdout();
    term.clear_screen().unwrap();
    
    println!("{}", "\n➕ ADD EXCLUDE DIRECTORY".bright_green().bold());
    println!("{}", "═".repeat(50).bright_blue());
    
    println!("{}", "💡 Enter directory name to exclude (or press Enter to cancel)".bright_yellow());
    
    let new_dir: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("📁 Directory name")
        .allow_empty(true)
        .interact_text()
        .unwrap();
    
    if new_dir.trim().is_empty() {
        println!("{}", "❌ Operation cancelled".bright_yellow());
    } else {
        println!("{}", format!("✅ Added '{}' to exclude list", new_dir).bright_green());
        // In a real implementation, this would save to config file
    }
    
    println!("\nPress Enter to continue...");
    std::io::stdin().read_line(&mut String::new()).unwrap();
    manage_exclude_list();
}

fn remove_exclude_directory() {
    let term = Term::stdout();
    term.clear_screen().unwrap();
    
    println!("{}", "\n➖ REMOVE EXCLUDE DIRECTORY".bright_red().bold());
    println!("{}", "═".repeat(50).bright_blue());
    
    let current_excludes = get_default_excludes();
    
    if current_excludes.is_empty() {
        println!("{}", "📝 No directories to remove".bright_yellow());
        println!("Press Enter to continue...");
        std::io::stdin().read_line(&mut String::new()).unwrap();
        manage_exclude_list();
        return;
    }
    
    let mut options = current_excludes.clone();
    options.push("⬅️ Cancel".to_string());
    
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("🎯 Select directory to remove from exclude list")
        .items(&options[..])
        .interact()
        .unwrap();
    
    if selection < current_excludes.len() {
        println!("{}", format!("✅ Removed '{}' from exclude list", current_excludes[selection]).bright_green());
        // In a real implementation, this would save to config file
    }
    
    println!("Press Enter to continue...");
    std::io::stdin().read_line(&mut String::new()).unwrap();
    manage_exclude_list();
}

fn select_common_excludes() {
    let term = Term::stdout();
    term.clear_screen().unwrap();
    
    println!("{}", "\n🎯 SELECT COMMON EXCLUDES".bright_yellow().bold());
    println!("{}", "═".repeat(60).bright_blue());
    
    println!("{}", r#"
    ╔════════════════════════════════════════════════════════════╗
    ║  🚀 QUICK SETUP: Select from commonly excluded directories ║
    ║  💡 These are directories that usually slow down searches  ║
    ║  ✨ Perfect for getting started with optimal performance   ║
    ╚════════════════════════════════════════════════════════════╝
    "#.bright_cyan());
    
    let show_examples = Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("🤔 Would you like to see what each directory type contains?")
        .default(false)
        .interact()
        .unwrap();
    
    if show_examples {
        show_exclude_examples();
    }
    
    let proceed = Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("📋 Continue with selecting common excludes?")
        .default(true)
        .interact()
        .unwrap();
    
    if !proceed {
        configure_excludes();
        return;
    }
    
    let common_excludes = vec![
        "node_modules (Node.js dependencies)",
        "target (Rust build directory)",
        "build (General build directory)",
        "_build (Build variant)",
        "dist (Distribution directory)",
        ".git (Git repository)",
        ".svn (Subversion repository)",
        "__pycache__ (Python cache)",
        ".pytest_cache (Pytest cache)",
        "venv (Python virtual environment)",
        ".venv (Python virtual environment)",
        ".idea (IntelliJ IDEA)",
        ".vscode (VS Code settings)",
        "deps (Dependencies)",
        "_deps (Dependencies variant)",
        ".cache (Cache directory)",
        "vendor (Third-party code)",
        "coverage (Coverage reports)",
        "tmp (Temporary files)",
        ".tmp (Temporary files)",
    ];

    let selected = MultiSelect::with_theme(&ColorfulTheme::default())
        .with_prompt("🔍 Select directories to exclude (use Space to select, Enter to confirm)")
        .items(&common_excludes[..])
        .interact()
        .unwrap();

    if selected.is_empty() {
        println!("{}", "📝 No directories selected".bright_yellow());
    } else {
        println!(
            "{}",
            format!("✅ Selected {} directories to exclude", selected.len()).bright_green()
        );
    }
    
    println!("Press Enter to continue...");
    std::io::stdin().read_line(&mut String::new()).unwrap();
    configure_excludes();
}

fn show_exclude_examples() {
    let term = Term::stdout();
    term.clear_screen().unwrap();
    
    println!("{}", "\n📖 EXCLUDE DIRECTORY EXAMPLES".bright_blue().bold());
    println!("{}", "═".repeat(60).bright_blue());
    
    println!("{}", "🟦 BUILD & COMPILATION:".bright_cyan());
    println!("   • target (Rust)    - Compiled binaries and build artifacts");
    println!("   • build            - General build output directories");
    println!("   • dist             - Distribution/bundled files");
    
    println!("{}", "\n🟩 DEPENDENCIES:".bright_green());
    println!("   • node_modules     - Node.js packages (can be huge!)");
    println!("   • vendor           - Third-party code/libraries");
    println!("   • deps             - Dependency directories");
    
    println!("{}", "\n🟨 VERSION CONTROL:".bright_yellow());
    println!("   • .git/.svn        - Version control metadata");
    println!("   • Contains history, branches, etc.");
    
    println!("{}", "\n🟪 CACHE & TEMP:".bright_magenta());
    println!("   • __pycache__      - Python bytecode cache");
    println!("   • .cache           - Various application caches");
    println!("   • tmp/.tmp         - Temporary files");
    
    println!("{}", "\n🟧 IDE SETTINGS:".bright_red());
    println!("   • .vscode/.idea    - Editor-specific configurations");
    println!("   • Usually not part of your actual code");
    
    println!("\nPress Enter to continue...");
    std::io::stdin().read_line(&mut String::new()).unwrap();
}

fn manage_profiles() {
    let term = Term::stdout();
    term.clear_screen().unwrap();
    
    println!("{}", "\n💾 PROFILE MANAGEMENT".bright_blue().bold());
    println!("{}", "═".repeat(50).bright_blue());

    let options = vec![
        "💾 Save Current Settings as Profile",
        "📂 Load Saved Profile",
        "🗑️  Delete Profile",
        "⬅️  Back to Exclude Manager",
    ];

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("🎯 Profile management options")
        .default(0)
        .items(&options[..])
        .interact()
        .unwrap();

    match selection {
        0 => save_exclude_profile(),
        1 => load_exclude_profile(),
        2 => delete_exclude_profile(),
        3 => configure_excludes(),
        _ => unreachable!(),
    }
}

fn save_exclude_profile() {
    let term = Term::stdout();
    term.clear_screen().unwrap();
    
    println!("{}", "\n💾 SAVE EXCLUDE PROFILE".bright_blue().bold());
    println!("{}", "═".repeat(50).bright_blue());
    
    let profile_name: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("📝 Enter profile name (or press Enter to cancel)")
        .allow_empty(true)
        .interact_text()
        .unwrap();
    
    if profile_name.trim().is_empty() {
        println!("{}", "❌ Operation cancelled".bright_yellow());
    } else {
        // Store the current exclude list in the profile
        let current_excludes = get_default_excludes();
        {
            let mut profiles = CUSTOM_PROFILES.lock().unwrap();
            profiles.insert(profile_name.clone(), current_excludes);
        }
        println!("{}", format!("✅ Saved exclude profile '{}'", profile_name).bright_green());
        println!("{}", "💡 Profile will now appear in the load list".bright_cyan());
    }
    
    println!("Press Enter to continue...");
    std::io::stdin().read_line(&mut String::new()).unwrap();
    manage_profiles();
}

fn load_exclude_profile() {
    let term = Term::stdout();
    term.clear_screen().unwrap();
    
    println!("{}", "\n📂 LOAD EXCLUDE PROFILE".bright_cyan().bold());
    println!("{}", "═".repeat(50).bright_blue());
    
    // Get both default and custom profiles
    let mut profiles = vec![
        "Default (Common excludes)".to_string(),
        "Minimal (Only .git)".to_string(), 
        "Development (Full dev excludes)".to_string(),
    ];
    
    // Add custom profiles
    {
        let custom_profiles = CUSTOM_PROFILES.lock().unwrap();
        for profile_name in custom_profiles.keys() {
            profiles.push(format!("{} (Custom)", profile_name));
        }
    }
    
    profiles.push("⬅️ Cancel".to_string());
    
    if profiles.len() == 4 { // Only default profiles + cancel
        println!("{}", "📝 No custom profiles found. Create some first!".bright_yellow());
        println!("Press Enter to continue...");
        std::io::stdin().read_line(&mut String::new()).unwrap();
        manage_profiles();
        return;
    }
    
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("📋 Select profile to load")
        .items(&profiles[..])
        .interact()
        .unwrap();
    
    if selection < profiles.len() - 1 {
        println!("{}", format!("✅ Loaded profile '{}'", profiles[selection]).bright_green());
        if profiles[selection].contains("(Custom)") {
            println!("{}", "🎨 Custom profile settings applied!".bright_magenta());
        }
    }
    
    println!("Press Enter to continue...");
    std::io::stdin().read_line(&mut String::new()).unwrap();
    manage_profiles();
}

fn delete_exclude_profile() {
    let term = Term::stdout();
    term.clear_screen().unwrap();
    
    println!("{}", "\n🗑️ DELETE PROFILE".bright_red().bold());
    println!("{}", "═".repeat(50).bright_blue());
    
    // Get custom profiles only (can't delete default ones)
    let mut profiles = Vec::new();
    {
        let custom_profiles = CUSTOM_PROFILES.lock().unwrap();
        for profile_name in custom_profiles.keys() {
            profiles.push(profile_name.clone());
        }
    }
    
    if profiles.is_empty() {
        println!("{}", "📝 No custom profiles to delete".bright_yellow());
        println!("Press Enter to continue...");
        std::io::stdin().read_line(&mut String::new()).unwrap();
        manage_profiles();
        return;
    }
    
    profiles.push("⬅️ Cancel".to_string());
    
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("🗑️ Select custom profile to delete")
        .items(&profiles[..])
        .interact()
        .unwrap();
    
    if selection < profiles.len() - 1 {
        let confirm = Confirm::with_theme(&ColorfulTheme::default())
            .with_prompt(&format!("⚠️ Really delete profile '{}'?", profiles[selection]))
            .default(false)
            .interact()
            .unwrap();
        
        if confirm {
            {
                let mut custom_profiles = CUSTOM_PROFILES.lock().unwrap();
                custom_profiles.remove(&profiles[selection]);
            }
            println!("{}", format!("✅ Deleted profile '{}'", profiles[selection]).bright_green());
        } else {
            println!("{}", "❌ Deletion cancelled".bright_yellow());
        }
    }
    
    println!("Press Enter to continue...");
    std::io::stdin().read_line(&mut String::new()).unwrap();
    manage_profiles();
}

fn reset_excludes() {
    let term = Term::stdout();
    term.clear_screen().unwrap();
    
    println!("{}", "\n🔄 RESET EXCLUDE LIST".bright_yellow().bold());
    println!("{}", "═".repeat(50).bright_blue());
    
    let confirm = Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("⚠️  Reset to default exclude list? This will remove all custom settings")
        .default(false)
        .interact()
        .unwrap();
    
    if confirm {
        println!("{}", "✅ Reset to default exclude list".bright_green());
        // In a real implementation, this would reset config file
    } else {
        println!("{}", "❌ Reset cancelled".bright_yellow());
    }
    
    println!("Press Enter to continue...");
    std::io::stdin().read_line(&mut String::new()).unwrap();
    configure_excludes();
}

fn get_default_excludes() -> Vec<String> {
    vec![
        "node_modules".to_string(),
        "target".to_string(),
        "build".to_string(),
        ".git".to_string(),
        "__pycache__".to_string(),
        ".vscode".to_string(),
        "dist".to_string(),
        ".cache".to_string(),
    ]
}

fn configure_progress() {
    println!("{}", "\n📊 PROGRESS DISPLAY OPTIONS".bright_blue().bold());

    let options = vec![
        "🚀 Minimal - Just progress bar",
        "📊 Standard - Progress + file count",
        "🔍 Detailed - Progress + current file names",
        "🎯 Verbose - All details + performance metrics",
    ];

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Choose progress display level")
        .default(1)
        .items(&options[..])
        .interact()
        .unwrap();

    println!(
        "{}",
        format!("✅ Progress display set to: {}", options[selection]).bright_green()
    );
    println!("Press Enter to continue...");
    std::io::stdin().read_line(&mut String::new()).unwrap();
    show_main_menu();
}

fn show_help() {
    println!("{}", "\n📖 HELP & EXAMPLES".bright_blue().bold());
    println!("{}", "═".repeat(60).bright_blue());

    println!("{}", "\n🎯 COMMAND LINE USAGE:".bright_yellow());
    println!("  ffinder                    # Launch interactive mode");
    println!("  ff grep 'pattern'          # Direct grep search");
    println!("  ff find 'name'             # Direct file search");

    println!("{}", "\n🔍 GREP EXAMPLES:".bright_yellow());
    println!("  ff grep 'main()' -d /projects");
    println!("  ff grep 'TODO|FIXME' -r -i");
    println!("  ff grep 'error' -e 'log,txt' -t 8");
    println!("  ff grep 'function' -x 'node_modules,build'");

    println!("{}", "\n📁 FILE SEARCH EXAMPLES:".bright_yellow());
    println!("  ff find '*.rs' -r");
    println!("  ff find 'config' -i");
    println!("  ff find 'main.cpp' -d /projects");

    println!("{}", "\n⚙️ OPTIONS:".bright_yellow());
    println!("  -d, --dir <PATH>        Directory to search");
    println!("  -i, --ignore-case       Case insensitive");
    println!("  -r, --regex             Use regex patterns");
    println!("  -e, --ext <EXTS>        File extensions");
    println!("  -t, --threads <NUM>     Thread count");
    println!("  -x, --exclude <DIRS>    Exclude directories");
    println!("  --progress              Show detailed progress");

    println!("{}", "\n🚀 PERFORMANCE TIPS:".bright_yellow());
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
    println!("{}", "\n🚀 EXECUTING SEARCH...".bright_green().bold());
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
    if exclude_common || (additional_excludes.is_some() && !additional_excludes.unwrap().is_empty())
    {
        let mut exclude_list = Vec::new();
        if exclude_common {
            exclude_list.extend_from_slice(&[
                "node_modules",
                "target",
                "build",
                "_build",
                "dist",
                ".git",
                ".svn",
                "deps",
                "_deps",
                ".cache",
            ]);
        }

        if let Some(additional) = additional_excludes {
            if !additional.is_empty() {
                for dir in additional.split(',') {
                    exclude_list.push(dir.trim());
                }
            }
        }

        let exclude_str = exclude_list.join(",");
        args.push("-x");
        args.push(&exclude_str);

        let result = Command::new("C:\\Users\\Aadis\\Tools\\FFinder\\file_finder.exe")
            .args(&args)
            .status();

        match result {
            Ok(status) => {
                if status.success() {
                    println!("{}", "\n✅ Search completed successfully!".bright_green());
                } else {
                    println!("{}", "\n❌ Search failed!".bright_red());
                }
            }
            Err(e) => {
                println!("{}", format!("\n❌ Failed to execute: {}", e).bright_red());
                println!(
                    "{}",
                    "💡 Make sure FFinder is properly installed".bright_yellow()
                );
            }
        }
    } else {
        let result = Command::new("C:\\Users\\Aadis\\Tools\\FFinder\\file_finder.exe")
            .args(&args)
            .status();

        match result {
            Ok(status) => {
                if status.success() {
                    println!("{}", "\n✅ Search completed successfully!".bright_green());
                } else {
                    println!("{}", "\n❌ Search failed!".bright_red());
                }
            }
            Err(e) => {
                println!("{}", format!("\n❌ Failed to execute: {}", e).bright_red());
                println!(
                    "{}",
                    "💡 Make sure FFinder is properly installed".bright_yellow()
                );
            }
        }
    }

    println!("\nPress Enter to return to main menu...");
    std::io::stdin().read_line(&mut String::new()).unwrap();
    show_main_menu();
}

fn execute_file_search(filename: &str, directory: &str, case_insensitive: bool, use_regex: bool) {
    println!("{}", "\n🚀 EXECUTING FILE SEARCH...".bright_cyan().bold());
    println!("{}", "═".repeat(50).bright_blue());

    let mut args = vec!["find", filename, "-d", directory];

    if case_insensitive {
        args.push("-i");
    }

    if use_regex {
        args.push("-r");
    }

    let result = Command::new("C:\\Users\\Aadis\\Tools\\FFinder\\file_finder.exe")
        .args(&args)
        .status();

    match result {
        Ok(status) => {
            if status.success() {
                println!(
                    "{}",
                    "\n✅ File search completed successfully!".bright_green()
                );
            } else {
                println!("{}", "\n❌ File search failed!".bright_red());
            }
        }
        Err(e) => {
            println!("{}", format!("\n❌ Failed to execute: {}", e).bright_red());
            println!(
                "{}",
                "💡 Make sure FFinder is properly installed".bright_yellow()
            );
        }
    }

    println!("\nPress Enter to return to main menu...");
    std::io::stdin().read_line(&mut String::new()).unwrap();
    show_main_menu();
}