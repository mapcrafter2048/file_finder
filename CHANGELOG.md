# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] - 2025-07-14

### Added
- Initial release of File Finder & Grep Tool
- **File Search Functionality:**
  - Recursive file search through directories
  - Literal text matching and regex pattern support
  - Case-sensitive/insensitive search options
  - Beautiful file information display with icons, sizes, and timestamps

- **Grep Functionality:**
  - Text pattern search within files (Windows grep -r alternative)
  - Syntax highlighting with yellow background for matches
  - File type filtering by extensions
  - Line number display with context
  - Regex support for complex pattern matching
  - Binary file detection to skip non-text files

- **CLI Interface:**
  - Colorful output with emojis and styled text
  - Progress indicators for long-running searches
  - Command-line interface with comprehensive help
  - Interactive mode with menu selection

- **Utility Scripts:**
  - PowerShell wrapper script (ff.ps1)
  - Windows batch wrapper script (ff.bat)
  - Demo script showcasing features (demo.bat)

- **Performance Features:**
  - Efficient directory traversal using walkdir
  - Smart binary file detection
  - Memory-conscious file processing
  - Optimized release build

- **File Type Support:**
  - 50+ file type icons for easy recognition
  - Support for programming languages, documents, archives, and more

### Technical Details
- Built with Rust 2021 edition
- Dependencies: clap, colored, walkdir, regex, chrono, indicatif, console, dialoguer
- Cross-platform compatibility (primary focus on Windows)
- Comprehensive error handling and user-friendly messages
