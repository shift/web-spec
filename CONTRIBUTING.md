# Contributing to web-spec

Thank you for your interest in contributing to web-spec! This document provides guidelines and instructions for contributing.

## Quick Start

1. Fork the repository
2. Clone your fork: `git clone https://github.com/YOUR-USERNAME/web-spec.git`
3. Create a feature branch: `git checkout -b my-feature`
4. Make your changes
5. Run tests: `cargo test`
6. Submit a pull request

## Development Setup

### With Nix (Recommended)

```bash
nix develop -c -- cargo build
nix develop -c -- cargo test
```

### Without Nix

```bash
cargo build
cargo test
```

## Coding Standards

### Rust

- Follow Rust naming conventions
- Use `cargo fmt` before committing
- Run `cargo clippy` to check for improvements
- Add documentation for public APIs
- Write tests for new functionality

### Gherkin Features

- Use Given-When-Then structure
- Keep scenarios focused and atomic
- Include descriptive feature and scenario names
- Add background steps for common prerequisites

## Testing

All tests must pass before merging:

```bash
# Run all tests
cargo test

# Run specific test suite
cargo test --test bdd_comprehensive
cargo test --test cli_integration

# Run with full features
cargo test --features chromiumoxide-backend
```

## Pull Request Process

1. Ensure all tests pass
2. Update documentation as needed
3. Add tests for new functionality
4. Keep PRs focused and atomic
5. Describe your changes clearly

## Commit Messages

Follow conventional commits:
- `feat:` New features
- `fix:` Bug fixes
- `docs:` Documentation changes
- `refactor:` Code restructuring
- `test:` Adding/updating tests
- `chore:` Maintenance tasks

Example: `feat(cli): add batch execution command`

## Reporting Issues

When reporting issues, include:


When- Clear description of the problem
- Steps to reproduce
- Expected behavior
- Actual behavior
- Rust version: `rustc --version`
- Platform info: `uname -a` or `systeminfo`

## Questions?

- Open a GitHub Discussion for questions
- Open an Issue for bugs
