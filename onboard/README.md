# DX Onboard

> Interactive TUI onboarding experience for DX - A Rust-based AI agent development platform

## Overview

DX Onboard is a standalone terminal user interface (TUI) application that provides an engaging, interactive onboarding flow for the DX platform. It showcases 24 different prompt types with beautiful rainbow animations, ASCII art, and a celebratory train animation on exit.

## Features

- **24 Interactive Prompt Types**: Complete showcase of all available prompt components
- **Rainbow Visual Effects**: Dynamic color cycling for an engaging visual experience
- **10 Random ASCII Logos**: Randomly selected figlet fonts for the DX logo (Bloody, 3d, Doh, Diamond, Electronic, Fraktur, Marquee, Reverse, Stellar, Tubular)
- **Train Animation**: Animated ASCII train with smoke effects on all exit scenarios
- **Theme System**: TOML-based theming with customizable colors and symbols
- **Runtime Detection**: Automatically detects environment (Real OS, VPS, Container, Restricted)
- **Configuration Export**: Saves complete onboarding data to `dx.json`

## Architecture

```
onboard/
├── src/
│   ├── main.rs              # Entry point, orchestrates onboarding flow
│   ├── splash.rs            # ASCII art rendering (logo + train animation)
│   ├── effects.rs           # Rainbow color effects
│   ├── prompts/             # All 24 prompt type implementations
│   │   ├── mod.rs           # Prompt exports and core traits
│   │   ├── theme.rs         # Theme system with TOML support
│   │   ├── input.rs         # Text input prompt
│   │   ├── email.rs         # Email validation prompt
│   │   ├── password.rs      # Secure password input
│   │   ├── url.rs           # URL validation prompt
│   │   ├── phone_input.rs   # Phone number input
│   │   ├── number.rs        # Numeric input with min/max
│   │   ├── rating.rs        # Star rating selector
│   │   ├── slider.rs        # Single value slider
│   │   ├── range_slider.rs  # Range selection slider
│   │   ├── toggle.rs        # Boolean toggle switch
│   │   ├── select.rs        # Single selection menu
│   │   ├── multiselect.rs   # Multiple selection menu
│   │   ├── tags.rs          # Tag input (comma-separated)
│   │   ├── autocomplete.rs  # Autocomplete search
│   │   ├── search_filter.rs # Searchable filtered list
│   │   ├── text.rs          # Multi-line text area
│   │   ├── wizard.rs        # Multi-step wizard
│   │   ├── progress.rs      # Progress bar
│   │   ├── spinner.rs       # Loading spinner
│   │   ├── log.rs           # Logging utilities
│   │   ├── confirm.rs       # Yes/No confirmation
│   │   └── trash/           # Archived unused prompts
│   └── ...
├── theme.toml               # Default theme configuration
├── Cargo.toml               # Dependencies and metadata
└── README.md                # This file
```

## Prompt Types Demonstrated

1. **Input** - Basic text input with placeholder
2. **Email** - Email validation with initial value
3. **Password** - Secure password input with hashing (Argon2)
4. **URL** - URL validation
5. **Phone** - Phone number input
6. **Number** - Numeric input with min/max constraints
7. **Rating** - Star rating (1-5)
8. **Slider** - Single value slider (0-100)
9. **Range Slider** - Range selection (e.g., work hours)
10. **Toggle** - Boolean switches (notifications, updates, telemetry)
11. **Select** - Single selection (theme, editor, shell)
12. **Multiselect** - Multiple selections (components, providers, skills)
13. **Tags** - Comma-separated tag input (programming languages)
14. **Autocomplete** - Searchable autocomplete (favorite language)
15. **Search Filter** - Filtered search with tags (frameworks)
16. **Text Area** - Multi-line text input (bio)
17. **Wizard** - Multi-step process
18. **Progress Bar** - Visual progress indicator
19. **Spinner** - Loading animation
20. **Log** - Info/Success/Warning/Error/Step messages
21. **Confirm** - Yes/No confirmation
22. **Section** - Formatted text sections
23. **Intro/Outro** - Welcome and farewell messages
24. **Custom Animations** - Train animation on exit

## Theme System

The theme system uses TOML configuration for easy customization:

```toml
[colors]
primary = "white"
secondary = "cyan"
success = "green"
warning = "yellow"
error = "red"
dim = "bright-black"

[symbols]
checkmark = "√"
cross = "×"
info = "i"
warning = "!"
error = "×"
arrow = "→"
bullet = "•"
```

## Dependencies

- `owo-colors` - Terminal color support
- `terminal_size` - Dynamic terminal sizing
- `rand` - Random logo selection
- `serde` + `toml` - Theme configuration
- `argon2` - Password hashing
- `chrono` - Timestamp generation
- `anyhow` - Error handling
- `ctrlc` - Graceful exit handling

## Integration Guide

### 1. As a Standalone Binary

```bash
cd onboard
cargo build --release
./target/release/onboard
```

### 2. As a Library Crate

Add to your `Cargo.toml`:

```toml
[dependencies]
onboard = { path = "./onboard" }
```

Use in your code:

```rust
use onboard::run_onboarding;

fn main() -> anyhow::Result<()> {
    let result = run_onboarding()?;
    println!("User: {}", result.name);
    println!("Email: {}", result.email);
    Ok(())
}
```

### 3. Embedding in Larger Application

```rust
// In your main application
mod onboard;

fn setup_wizard() -> anyhow::Result<()> {
    // Run onboarding flow
    let config = onboard::run_onboarding()?;
    
    // Use the configuration
    apply_user_preferences(&config)?;
    
    Ok(())
}
```

## Configuration Output

The onboarding process generates a `dx.json` file with complete user data:

```json
{
  "name": "Developer Name",
  "email": "dev@example.com",
  "website": "https://example.com",
  "phone": "+1234567890",
  "bio": "Software developer...",
  "experience_years": 5,
  "satisfaction_rating": 4,
  "productivity_level": 75,
  "work_hours": [9, 17],
  "programming_languages": ["rust", "typescript"],
  "favorite_language": "rust",
  "framework": "React",
  "project_type": "web_frontend",
  "selected_skills": ["frontend", "backend"],
  "runtime_environment": "real_os",
  "selected_components": ["desktop_app", "tui"],
  "selected_providers": ["openai", "anthropic"],
  "preferences": {
    "theme": "dark",
    "editor": "vscode",
    "shell": "zsh",
    "notifications": true,
    "auto_updates": false,
    "telemetry": false
  },
  "wizard_completed_steps": 3,
  "timestamp": "2026-03-19T10:30:00Z"
}
```

## Exit Scenarios

The train animation plays on ALL exit scenarios:

1. **Successful Completion** - Shows celebration train (15 frames)
2. **User Cancellation** - Shows farewell train (15 frames)
3. **Ctrl+C / SIGINT** - Shows farewell train (15 frames)
4. **Error Exit** - Shows farewell train (15 frames)

## Customization

### Adding New Prompt Types

1. Create new file in `src/prompts/your_prompt.rs`
2. Implement the prompt logic
3. Export from `src/prompts/mod.rs`
4. Add to onboarding flow in `src/main.rs`

### Customizing Themes

Edit `theme.toml` to change colors and symbols:

```toml
[colors]
primary = "magenta"  # Change primary color

[symbols]
checkmark = "✓"      # Use Unicode symbols
```

### Adding More Logo Fonts

Edit `src/splash.rs` and add to `DX_LOGOS` array:

```rust
const DX_LOGOS: [&str; 11] = [
    // ... existing logos
    r#"Your new ASCII art here"#,
];
```

## Runtime Environment Detection

The onboarding automatically detects:

- **Real OS** - Desktop workstation (full features)
- **VPS** - Cloud virtual machine (limited components)
- **Container** - Docker/Kubernetes (minimal components)
- **Restricted** - CI/CD environment (non-interactive)

## Performance

- **Startup Time**: ~100ms
- **Memory Usage**: ~5MB
- **Binary Size**: ~1.2MB (release build)
- **Dependencies**: 45 crates

## Testing

```bash
# Run all tests
cargo test

# Run with output
cargo test -- --nocapture

# Run specific test
cargo test test_theme_loading
```

## Building

```bash
# Debug build
cargo build

# Release build (optimized)
cargo build --release

# With specific features
cargo build --release --features "custom-theme"
```

## Troubleshooting

### Colors Not Showing

Ensure your terminal supports 24-bit true color:

```bash
echo $COLORTERM  # Should output "truecolor" or "24bit"
```

### Train Animation Glitchy

The animation requires a terminal width of at least 80 characters:

```bash
# Check terminal size
tput cols  # Should be >= 80
```

### Theme Not Loading

Verify `theme.toml` exists and is valid TOML:

```bash
# Validate TOML syntax
cargo run --example validate_theme
```

## License

Same as parent DX project.

## Contributing

This is a standalone component of the DX project. For contributions:

1. Test thoroughly with `cargo test`
2. Ensure no regressions in prompt types
3. Maintain theme compatibility
4. Document new features

## Roadmap

- [ ] Add more prompt types (date picker, color picker)
- [ ] Support custom animation frames
- [ ] Add sound effects (optional)
- [ ] Multi-language support
- [ ] Accessibility improvements
- [ ] Web-based preview mode

## Contact

Part of the DX project - Enhanced Development Experience for Rust AI agents.
