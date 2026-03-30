# Contributing to dx-tui

Thank you for your interest in contributing to dx-tui! This document provides guidelines for contributing to this production-ready codebase.

## Code of Conduct

Be respectful, professional, and constructive in all interactions.

## Getting Started

### Prerequisites

- Rust 1.85+ (Edition 2024)
- Node.js 18+ (for Codex CLI)
- Git

### Setup

```bash
# Clone the repository
git clone https://github.com/yourusername/dx-tui
cd dx-tui

# Build the project
cargo build --release

# Run tests
cargo test --workspace

# Verify code quality
cargo fmt --all --check
cargo clippy --workspace --all-targets --all-features
```

## Development Workflow

### 1. Create a Branch

```bash
git checkout -b feature/your-feature-name
```

### 2. Make Changes

- Follow Rust 2024 idioms
- Use async/await for I/O operations
- Add tests for new functionality
- Update documentation as needed

### 3. Format and Lint

```bash
# Format code
cargo fmt --all

# Check for issues
cargo clippy --workspace --all-targets --all-features

# Verify build
cargo check --workspace
```

### 4. Commit Changes

```bash
git add .
git commit -m "feat: add your feature description"
```

Use conventional commit messages:
- `feat:` - New feature
- `fix:` - Bug fix
- `docs:` - Documentation changes
- `refactor:` - Code refactoring
- `test:` - Test additions/changes
- `chore:` - Maintenance tasks

### 5. Push and Create PR

```bash
git push origin feature/your-feature-name
```

Then create a Pull Request on GitHub.

## Code Standards

### Rust Code

- **Edition**: Rust 2024
- **Async**: Use Tokio for all I/O operations
- **Error Handling**: Use `anyhow::Result` for applications, `thiserror` for libraries
- **Naming**:
  - Types: `PascalCase`
  - Functions: `snake_case`
  - Constants: `SCREAMING_SNAKE_CASE`

### Documentation

- Add doc comments for public APIs
- Include examples in doc comments
- Update relevant markdown files

### Testing

- Write unit tests for new functionality
- Ensure all tests pass: `cargo test --workspace`
- Add integration tests when appropriate

## Using Codex CLI

This project is optimized for Codex CLI. See [.github/CODEX_SETUP.md](.github/CODEX_SETUP.md) for details.

```bash
# Install Codex CLI
npm install -g @openai/codex

# Use Codex for development
codex "help me implement feature X"
```

## Project Structure

See [PROJECT_STRUCTURE.md](PROJECT_STRUCTURE.md) for detailed codebase organization.

## What to Contribute

### Good First Issues

- Documentation improvements
- Bug fixes
- Test coverage improvements
- Performance optimizations

### Feature Requests

- Open an issue first to discuss
- Ensure it aligns with project goals
- Consider backward compatibility

### Bug Reports

Include:
- Steps to reproduce
- Expected behavior
- Actual behavior
- Environment details (OS, Rust version, terminal)

## Code Review Process

1. All PRs require review
2. CI must pass (format, lint, build, tests)
3. Maintain zero warnings policy
4. Keep commits clean and focused

## Questions?

- Open an issue for questions
- Check existing documentation first
- Be specific and provide context

## License

By contributing, you agree that your contributions will be licensed under the MIT License.

---

Thank you for contributing to dx-tui! 🚀
