# Contributing to ZCode

Thank you for your interest in contributing to ZCode! We welcome contributions from everyone, whether you're reporting bugs, suggesting features, or submitting code.

## Code of Conduct

We are committed to providing a welcoming and inspiring community for all. Please be respectful and constructive in all interactions.

## How to Report Bugs

Before creating a bug report, please check if the issue already exists. When creating a bug report, include:

- **A clear, descriptive title**
- **A detailed description of the problem**
- **Steps to reproduce the issue**
- **Expected vs. actual behavior**
- **Screenshots or error logs if applicable**
- **Environment details** (OS, Rust version, Zellij version)

## Suggesting Features

Feature suggestions are welcome! When proposing a new feature:

- Use a clear, descriptive title
- Provide a detailed description of the proposed feature
- Explain the use case and benefits
- Suggest how it might be implemented (if applicable)
- Reference related issues or features

## Development Setup

### Prerequisites

- Rust 1.70+ (install from [rustup.rs](https://rustup.rs/))
- Zellij (for testing the plugin)
- wasm32-wasip1 target: `rustup target add wasm32-wasip1`

### Building

```bash
# Clone the repository
git clone https://github.com/kodyberry23/zcode
cd zcode

# Build for WASM
cargo build --release --target wasm32-wasip1

# Run tests
cargo test --lib

# Run linter
cargo clippy -- -D warnings

# Format code
cargo fmt --all
```

## Making Changes

1. **Fork the repository** and create a feature branch: `git checkout -b feature/your-feature-name`

2. **Make your changes** following Rust best practices:
   - Use clear variable and function names
   - Add comments for complex logic
   - Include documentation for public APIs
   - Follow the existing code style (use `cargo fmt` before committing)

3. **Add tests** for new functionality:
   - Write unit tests in the same file as the implementation (within `#[cfg(test)]` modules)
   - Aim for reasonable code coverage
   - Test edge cases and error conditions

4. **Update documentation** if you change APIs or add features

5. **Test your changes**:
   ```bash
   cargo test --lib --verbose
   cargo clippy -- -D warnings
   cargo fmt --all -- --check
   ```

6. **Commit with clear messages**:
   ```bash
   git commit -m "feat: add support for custom parsers

   - Add new parse_custom function
   - Add tests for custom parser
   - Update documentation"
   ```

7. **Push and create a Pull Request** with:
   - A clear title
   - A description of what changed and why
   - Reference to any related issues (e.g., "Fixes #42")

## Pull Request Process

1. **Title**: Use clear, descriptive titles. Prefix with: `feat:`, `fix:`, `docs:`, `refactor:`, `test:`, `chore:`

2. **Description**: Explain the changes and why they're needed

3. **Tests**: Ensure all tests pass:
   ```bash
   cargo test --lib
   ```

4. **CI**: All GitHub Actions checks must pass before merging

5. **Review**: Respond to any feedback or suggestions

## Code Quality Standards

### Rust Style

- Follow [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- Use `cargo fmt` for formatting (configured in `rustfmt.toml`)
- Use `cargo clippy` for linting (configured in `clippy.toml`)
- Prefer explicit error handling with `Result` types
- Use `anyhow` for error context

### Testing

- Write tests for all public functions
- Use descriptive test names: `test_function_does_something_specific`
- Test both happy paths and error cases
- Aim for coverage of edge cases

### Documentation

- Add doc comments for all public items
- Use examples in doc comments where helpful
- Document error cases in doc comments
- Keep README.md up to date

## Release Process

Releases are handled by the maintainers. Version numbers follow [Semantic Versioning](https://semver.org/):
- MAJOR.MINOR.PATCH (e.g., 1.2.3)
- Breaking changes increment MAJOR
- New features increment MINOR
- Bug fixes increment PATCH

## Questions?

- Check existing issues and discussions
- Create a new discussion or issue with your question
- Reach out to the maintainers

## License

By contributing, you agree that your contributions will be licensed under the MIT License.

---

Thank you for making ZCode better! 🎉
