# Contributing to File Finder & Grep Tool

Thank you for your interest in contributing to this project! We welcome contributions from everyone.

## How to Contribute

### Reporting Bugs

If you find a bug, please create an issue with:
- A clear, descriptive title
- Steps to reproduce the bug
- Expected vs actual behavior
- Your operating system and Rust version
- Any relevant error messages

### Suggesting Features

We welcome feature suggestions! Please:
- Check if the feature has already been requested
- Clearly describe the feature and its benefits
- Provide examples of how it would be used

### Code Contributions

1. **Fork the repository**
2. **Create a feature branch**: `git checkout -b feature/amazing-feature`
3. **Make your changes**
4. **Add tests** if applicable
5. **Run the test suite**: `cargo test`
6. **Check formatting**: `cargo fmt`
7. **Run clippy**: `cargo clippy`
8. **Commit your changes**: `git commit -m 'Add amazing feature'`
9. **Push to your branch**: `git push origin feature/amazing-feature`
10. **Open a Pull Request**

### Code Style

- Follow Rust conventions and idioms
- Use `cargo fmt` to format your code
- Address any warnings from `cargo clippy`
- Write clear, descriptive commit messages
- Add documentation for public APIs

### Testing

- Add unit tests for new functionality
- Ensure all existing tests pass
- Test on multiple platforms if possible (Windows, macOS, Linux)

### Development Setup

```bash
# Clone your fork
git clone https://github.com/your-username/file_finder.git
cd file_finder

# Build the project
cargo build

# Run tests
cargo test

# Build release version
cargo build --release

# Test the CLI
./target/release/file_finder --help
```

## Project Structure

```
src/
├── main.rs          # CLI interface and main logic
├── file_search.rs   # File search functionality
├── grep_search.rs   # Grep search functionality
└── utils.rs         # Utility functions and file icons
```

## Getting Help

If you need help with contributing:
- Open an issue with the "question" label
- Check existing issues and discussions
- Review the project documentation

## Code of Conduct

Please be respectful and inclusive in all interactions. We want this to be a welcoming community for everyone.

## License

By contributing, you agree that your contributions will be licensed under the MIT License.
