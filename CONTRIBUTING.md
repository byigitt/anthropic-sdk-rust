# Contributing

Thank you for your interest in contributing to the Anthropic Rust SDK!

## Setting up the environment

### Prerequisites

- Rust 1.75 or higher
- Cargo (comes with Rust)

### Getting Started

1. Clone the repository:

```sh
git clone https://github.com/byigitt/anthropic-sdk-rust.git
cd anthropic-sdk-rust
```

2. Build the project:

```sh
cargo build
```

3. Run tests:

```sh
cargo test
```

## Development Workflow

### Building

```sh
# Debug build
cargo build

# Release build
cargo build --release

# Check for errors without building
cargo check
```

### Running Examples

All files in the `examples/` directory can be run with:

```sh
# Set your API key
export ANTHROPIC_API_KEY="your-api-key"

# Run an example
cargo run --example basic
cargo run --example streaming
cargo run --example tool_use
```

### Running Tests

```sh
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run a specific test
cargo test test_name
```

### Linting and Formatting

This repository uses `rustfmt` and `clippy` for code formatting and linting.

To format code:

```sh
cargo fmt
```

To run clippy:

```sh
cargo clippy -- -D warnings
```

## Code Style

- Follow the [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- Use `rustfmt` for formatting
- All public APIs should have documentation comments
- Write tests for new functionality

## Pull Request Process

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/my-feature`)
3. Make your changes
4. Run `cargo fmt` and `cargo clippy`
5. Run `cargo test` to ensure all tests pass
6. Commit your changes with a clear commit message
7. Push to your fork and create a Pull Request

## Commit Messages

We follow conventional commit messages:

- `feat:` - New features
- `fix:` - Bug fixes
- `docs:` - Documentation changes
- `test:` - Adding or updating tests
- `refactor:` - Code refactoring
- `chore:` - Maintenance tasks

Examples:
```
feat: add support for tool use streaming
fix: handle rate limit errors correctly
docs: update README with async examples
```

## Reporting Issues

When reporting issues, please include:

- Rust version (`rustc --version`)
- SDK version
- Operating system
- Minimal code example to reproduce the issue
- Full error message or stack trace

## License

By contributing, you agree that your contributions will be licensed under the MIT License.
