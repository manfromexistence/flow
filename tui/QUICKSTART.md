# dx-tui Quick Start Guide

> Get up and running with dx-tui and Codex CLI in 5 minutes

## ✅ Production Ready

dx-tui is a production-ready terminal UI with zero warnings, fully formatted code, and comprehensive Codex CLI integration.

## Prerequisites

- Rust 1.85+ installed
- Node.js 18+ installed
- OpenAI account (ChatGPT Plus or API key)

## Installation

### 1. Install Codex CLI

```bash
npm install -g @openai/codex
```

### 2. Build dx-tui

```bash
# Clone and build
git clone https://github.com/yourusername/dx-tui
cd dx-tui
cargo build --release

# Optional: Install globally
sudo cp target/release/dx /usr/local/bin/
```

### 3. Authenticate Codex

```bash
# Launch Codex (opens browser for OAuth)
codex

# OR use API key
export OPENAI_API_KEY="sk-..."
```

## Basic Usage

### Launch dx-tui

```bash
# From project directory
cargo run --release

# Or if installed globally
dx
```

### Navigation

- **Arrow Keys**: Navigate file browser
- **Left Arrow** (from splash): Go to animation carousel
- **Right Arrow** (from splash): Go to file browser
- **Up/Down** (in carousel): Set intro/outro animations
- **Space**: Hold to show spinner
- **i**: Enter chat mode
- **Esc**: Go back
- **Ctrl+C**: Exit

### Using with Codex

```bash
# In dx-tui chat mode, type:
> "Refactor this module for better error handling"

# Or from command line:
codex "Review the animation system in dx-tui"
```

## Configuration

### Minimal Setup

Create `~/.codex/config.toml`:

```toml
model = "gpt-5.3-codex"
approval_policy = "on-request"
sandbox_mode = "workspace-write"
```

That's it! The `AGENTS.md` file in the project root will be automatically read by Codex.

## Next Steps

- Read [CODEX_INTEGRATION.md](CODEX_INTEGRATION.md) for detailed integration guide
- Check [AGENTS.md](AGENTS.md) to understand project conventions
- Explore [README.md](README.md) for full feature list

## Common Commands

```bash
# Build and run (production-ready)
cargo run --release

# Run with debug logging
RUST_LOG=debug cargo run

# Verify code quality (all checks pass)
cargo fmt --all --check
cargo clippy --workspace --all-targets --all-features
cargo check --workspace

# Build for production
cargo build --release --locked
```

All commands run cleanly with zero warnings.

## Troubleshooting

**Codex not found?**
```bash
npm install -g @openai/codex
```

**Build errors?**
```bash
rustup update
cargo clean
cargo build --release
```

**Authentication issues?**
```bash
codex logout
codex  # Re-authenticate
```

## Resources

- **Full Integration Guide**: [CODEX_INTEGRATION.md](CODEX_INTEGRATION.md)
- **Project Conventions**: [AGENTS.md](AGENTS.md)
- **Codex CLI Docs**: https://platform.openai.com/docs/codex-cli
