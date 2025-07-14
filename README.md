# File Finder & Grep Tool 🚀

[![CI](https://github.com/mapcrafter2048/file_finder/workflows/CI/badge.svg)](https://github.com/mapcrafter2048/file_finder/actions)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/rust-1.70+-blue.svg)](https://www.rust-lang.org/)
[![Platform](https://img.shields.io/badge/platform-Windows%20%7C%20macOS%20%7C%20Linux-lightgrey.svg)](https://github.com/mapcrafter2048/file_finder)

A powerful, colorful, and aesthetically pleasing CLI tool for searching files and text patterns in directories, built with Rust.

![Demo](https://img.shields.io/badge/CLI-Colorful%20%26%20Beautiful-brightgreen.svg)

## Features

### 🔍 File Search
- **Recursive file search** through directories
- **Multiple search modes**: literal text matching or regex patterns
- **Case-sensitive or case-insensitive** search options
- **Beautiful file information display** including:
  - File size with human-readable formatting
  - Last modified timestamp
  - File type icons based on extensions
  - Full file path and directory location

### 🔎 Grep Functionality
- **Text pattern search** within files (like `grep -r` on Unix systems)
- **Syntax highlighting** for matched text
- **File type filtering** by extensions
- **Line number display** with context
- **Regex support** for complex pattern matching
- **Binary file detection** to avoid searching non-text files

### 🎨 Beautiful CLI Interface
- **Colorful output** with emojis and styled text
- **Progress indicators** for long-running searches
- **Clean, organized results** with visual separators
- **File type icons** for easy recognition
- **Interactive mode** for guided usage

## Installation

1. Make sure you have Rust installed on your system
2. Clone or navigate to the project directory
3. Build the project:
   ```bash
   cargo build --release
   ```
4. The executable will be available at `target/release/file_finder.exe`

## Usage

### Command Line Interface

#### File Search
```bash
# Search for a specific filename
file_finder find "main.rs"

# Search in a specific directory
file_finder find "config.json" --dir "C:\Projects"

# Case-insensitive search
file_finder find "readme" --ignore-case

# Use regex patterns
file_finder find ".*\.rs$" --regex

# Combine options
file_finder find "test.*\.py$" --dir "src" --regex --ignore-case
```

#### Grep Search
```bash
# Search for text pattern in all files
file_finder grep "TODO"

# Search in specific directory
file_finder grep "function" --dir "src"

# Search only in specific file types
file_finder grep "import" --ext "py,js,ts"

# Case-insensitive text search
file_finder grep "error" --ignore-case

# Use regex patterns
file_finder grep "\w+@\w+\.\w+" --regex

# Combine all options
file_finder grep "async fn" --dir "src" --ext "rs" --ignore-case
```

### Interactive Mode

Simply run the tool without any arguments to enter interactive mode:
```bash
file_finder
```

This will present you with a menu to choose between file search and grep search, with guided prompts for all options.

## Examples

### 1. Find All Rust Files
```bash
file_finder find ".*\.rs$" --regex
```

### 2. Search for Function Definitions
```bash
file_finder grep "pub fn" --ext "rs"
```

### 3. Find Configuration Files
```bash
file_finder find "config" --ignore-case
```

### 4. Search for Email Addresses
```bash
file_finder grep "\b[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\.[A-Z|a-z]{2,}\b" --regex
```

### 5. Find Large Files (using filename patterns)
```bash
file_finder find ".*\.(zip|rar|tar|gz)$" --regex
```

## Output Features

### File Search Results Show:
- 📄 File type icon
- 📍 Directory location
- 📏 File size (human-readable: B, KB, MB, GB)
- 🕒 Last modified date and time
- 🔗 Full file path

### Grep Results Show:
- 📄 Filename with match count
- 📍 Directory location
- Line numbers with context
- **Highlighted matched text** (yellow background)
- Multiple matches per file organized clearly

## Supported File Types & Icons

The tool automatically detects file types and displays appropriate icons:

- 🦀 Rust (.rs)
- 🐍 Python (.py)
- 🟨 JavaScript (.js)
- 🔷 TypeScript (.ts)
- ☕ Java (.java)
- 🌐 HTML (.html)
- 🎨 CSS (.css)
- 📖 Markdown (.md)
- 📋 JSON (.json)
- ⚙️ Config files (.cfg, .conf, .yml)
- 📦 Archives (.zip, .rar, .tar)
- 🖼️ Images (.png, .jpg, .gif)
- And many more...

## Performance Features

- **Smart binary file detection** - automatically skips binary files during grep searches
- **Progress indicators** for long-running operations
- **Efficient walking** of directory trees
- **Memory-conscious** processing of large files
- **Configurable file type filtering** to avoid unnecessary file reads

## Command Reference

### File Search Options
- `filename` - The filename or pattern to search for
- `-d, --dir <PATH>` - Directory to search in (default: current directory)
- `-i, --ignore-case` - Case insensitive search
- `-r, --regex` - Use regex pattern matching

### Grep Search Options
- `pattern` - The text pattern to search for
- `-d, --dir <PATH>` - Directory to search in (default: current directory)
- `-i, --ignore-case` - Case insensitive search
- `-r, --regex` - Use regex pattern matching
- `-e, --ext <EXTENSIONS>` - File extensions to search (comma-separated, e.g., "rs,py,js")

### Global Options
- `-h, --help` - Show help information
- `-V, --version` - Show version information

## Building from Source

```bash
# Clone the repository
git clone <repository-url>
cd file_finder

# Install dependencies (automatically handled by Cargo)
cargo build

# Run tests
cargo test

# Build release version
cargo build --release

# Run directly
cargo run -- find "*.rs" --regex
```

## Dependencies

- **clap** - Command line argument parsing
- **colored** - Terminal color support
- **walkdir** - Recursive directory traversal
- **regex** - Regular expression support
- **chrono** - Date and time formatting
- **indicatif** - Progress bars
- **console** - Terminal interaction
- **dialoguer** - Interactive prompts

## License

This project is open source and available under the MIT License.

## Contributing

Contributions are welcome! Please feel free to submit pull requests or open issues for bugs and feature requests.

---

**Note**: This tool is designed to be a Windows-friendly alternative to Unix tools like `find` and `grep`, with enhanced visual output and user experience.
