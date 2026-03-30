# dx-tui Project Structure

> Production-ready terminal UI for Codex CLI integration

## Root Directory

```
dx-tui/
├── .cargo/              # Cargo build configuration
├── .codex/              # Codex CLI integration
│   ├── config.toml      # Project-specific Codex settings
│   └── AGENTS.override.md  # Critical project rules (overrides AGENTS.md)
├── .github/             # GitHub configuration
│   └── CODEX_SETUP.md   # Contributor guide for Codex CLI
├── .kiro/               # Kiro IDE configuration
├── assets/              # Logo and desktop files
├── figlet/              # FIGlet fonts (114 .dx files)
├── nix/                 # Nix package configuration
├── scripts/             # Build and validation scripts
├── snap/                # Snap package configuration
├── src/                 # Main application source code
│   ├── main.rs          # Entry point
│   ├── chat.rs          # Chat interface
│   ├── animations.rs    # Animation system
│   ├── dispatcher.rs    # Event dispatcher
│   └── file_browser/    # File browser modules (26 crates)
├── hexed/               # Development artifacts (not in production)
├── AGENTS.md            # Codex CLI project documentation
├── CODEX_INTEGRATION.md # Detailed Codex integration guide
├── QUICKSTART.md        # 5-minute setup guide
├── README.md            # Main project documentation
├── LICENSE              # MIT License
├── Cargo.toml           # Workspace manifest
└── config.toml          # Application configuration
```

## Key Files

### Production Files

- **AGENTS.md** - Primary documentation for Codex CLI (auto-loaded)
- **CODEX_INTEGRATION.md** - Comprehensive integration guide
- **QUICKSTART.md** - Quick setup for new users
- **README.md** - Main project documentation
- **Cargo.toml** - Workspace configuration (26 member crates)
- **config.toml** - Application runtime configuration

### Configuration Files

- **.codex/config.toml** - Codex CLI project settings
- **.codex/AGENTS.override.md** - Critical rules (highest priority)
- **clippy.toml** - Clippy linter configuration
- **rustfmt.toml** - Rust formatter configuration
- **.gitignore** - Git ignore patterns

### Build Files

- **build.rs** - Build script for font compression
- **Cargo.lock** - Dependency lock file
- **flake.nix** - Nix flake configuration

## Source Code Structure

```
src/
├── main.rs              # Application entry point
├── chat.rs              # Chat interface logic
├── chat_components.rs   # Chat UI components
├── chat_input.rs        # Chat input handling
├── animations.rs        # Animation system (Matrix, Train, FIGlet)
├── dispatcher.rs        # Event dispatcher
├── components.rs        # General UI components
├── effects.rs           # Visual effects
├── exit_animation.rs    # Exit animation logic
├── bridge.rs            # Bridge between components
└── file_browser/        # File browser implementation
    ├── actor/           # Async task actors
    ├── adapter/         # Image/preview adapters
    ├── boot/            # Bootstrap logic
    ├── config/          # Configuration management
    ├── core/            # Core data structures
    ├── fs/              # Filesystem operations
    ├── plugin/          # Lua plugin system
    ├── widgets/         # UI widgets
    └── ...              # 26 total member crates
```

## Development Artifacts

The `hexed/` folder contains development files not part of production:

- Development planning documents
- Temporary scripts and experiments
- Old TODO lists and notes
- Theme configuration experiments

These files are excluded from production builds via `.gitignore`.

## Build Targets

```bash
# Development build
cargo build

# Production build (optimized, stripped)
cargo build --release

# Check without building
cargo check --workspace

# Run tests
cargo test --workspace

# Format code
cargo fmt --all

# Lint code
cargo clippy --workspace --all-targets --all-features
```

## Production Status

✅ **Zero compiler warnings**  
✅ **Zero clippy warnings**  
✅ **Fully formatted codebase**  
✅ **Comprehensive documentation**  
✅ **Optimized release builds**  
✅ **Professional structure**

## Codex CLI Integration

When Codex CLI runs in this directory:

1. Reads `AGENTS.md` for project context
2. Applies `.codex/AGENTS.override.md` for critical rules
3. Uses `.codex/config.toml` for project-specific settings
4. Understands Rust 2024 conventions and architecture
5. Follows project boundaries and best practices

## License

MIT License - See LICENSE file for details.
