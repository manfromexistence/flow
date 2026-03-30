# dx-tui Codex Override Instructions

> This file overrides global AGENTS.md settings when working on dx-tui

## ✅ Production-Ready Codebase

This project is production-ready with:
- Zero compiler warnings
- Zero clippy warnings (silenced at crate level)
- Fully formatted codebase
- Comprehensive test coverage
- Optimized release builds

## Critical Rules for This Project

### Never Remove Features

- **DO NOT** remove or simplify the animation system (Matrix, Train, FIGlet)
- **DO NOT** remove the file browser functionality
- **DO NOT** remove Lua plugin support
- **DO NOT** simplify async/await patterns to sync code

### Always Preserve

- Rust Edition 2024 syntax and features
- Tokio async runtime (never suggest sync alternatives)
- Ratatui UI framework (never suggest cursive, termion, etc.)
- Workspace structure (26 member crates)

### Before Any Code Changes

1. Read the relevant module documentation
2. Check `AGENTS.md` for project conventions
3. Verify changes won't break animations or file browser
4. Test with `cargo check --workspace` before committing

### Rust-Specific Reminders

- This is a **Rust project**, not Node.js (despite Codex being npm-based)
- Use `cargo add` for dependencies, never manually edit Cargo.toml
- Use `workspace = true` for dependencies in member crates
- Run `cargo fmt --all` before suggesting completion
- Run `cargo clippy --workspace --all-targets --all-features` to verify

### File Browser Architecture

The file browser is based on yazi's architecture:
- **actor/**: Async task actors (don't simplify)
- **adapter/**: Image protocol adapters (complex, test thoroughly)
- **widgets/**: Ratatui widgets (follow existing patterns)

### Animation System

- Animations are a core feature, not optional
- FIGlet fonts in `/figlet/*.dx` are pre-generated
- Animation state is managed in `ChatState`
- Intro/outro animations are user-configurable

## Quick Reference

```bash
# Build (production-ready)
cargo build --release

# Test
cargo test --workspace

# Lint (passes cleanly)
cargo clippy --workspace --all-targets --all-features

# Format (already formatted)
cargo fmt --all

# Run
cargo run --release
```

All commands execute cleanly with zero warnings.

## When in Doubt

- Check `AGENTS.md` for detailed conventions
- Check `CODEX_INTEGRATION.md` for integration patterns
- Check `README.md` for feature documentation
- Ask the user before making architectural changes
