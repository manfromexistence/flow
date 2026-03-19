# Integrating DX Onboard into Your Project

> Complete guide for integrating the onboard TUI library into the main DX project

## Overview

The `onboard/` directory contains a standalone TUI library with 24 interactive prompt types. It's designed to be integrated as a workspace member or path dependency.

## Project Structure

```
onboard/
├── src/
│   ├── lib.rs              # Public API - use this for integration
│   ├── main.rs             # Standalone binary (for testing)
│   ├── effects.rs          # Rainbow color effects
│   ├── splash.rs           # ASCII art and animations
│   └── prompts/            # 24 prompt types
│       ├── mod.rs
│       ├── theme.rs
│       ├── input.rs
│       ├── email.rs
│       ├── password.rs
│       └── ... (21 more)
├── theme.toml              # Theme configuration
├── Cargo.toml              # Dependencies
├── README.md               # Project overview
├── PROMPTS.md              # Complete prompt usage guide
├── THEMING.md              # Theme customization
└── EXAMPLES.md             # Integration patterns
```

## Integration Steps

### Step 1: Add to Workspace

In your root `Cargo.toml`:

```toml
[workspace]
members = [
    "onboard",
    "your-main-crate",
    # ... other crates
]
```

### Step 2: Add Dependency

In your main crate's `Cargo.toml`:

```toml
[dependencies]
onboard = { path = "../onboard" }
```

### Step 3: Use in Code

```rust
// Import the library
use onboard::prompts::*;

fn main() -> anyhow::Result<()> {
    // Use individual prompts
    let name = input::input("Your name?")
        .placeholder("Developer")
        .interact()?;
    
    println!("Hello, {}!", name);
    Ok(())
}
```

## Quick Integration Examples

### Example 1: Simple Setup Flow

```rust
use onboard::prompts::*;

fn setup() -> anyhow::Result<()> {
    intro("Welcome to DX!")?;
    
    let name = input::input("Your name?").interact()?;
    let email = email::email("Your email?").interact()?;
    
    let theme = select("Choose theme")
        .item("dark", "Dark", "Night mode")
        .item("light", "Light", "Day mode")
        .interact()?;
    
    log::success(format!("Setup complete, {}!", name))?;
    outro("Thanks for using DX!")?;
    
    Ok(())
}
```

### Example 2: Use Complete Onboarding

```rust
use onboard::run_onboarding;

fn main() -> anyhow::Result<()> {
    // Runs all 24 prompts and saves to dx.json
    let config = run_onboarding()?;
    
    // Use the configuration
    println!("User: {}", config.name);
    println!("Email: {}", config.email);
    println!("Theme: {}", config.preferences.theme);
    
    Ok(())
}
```

### Example 3: First-Run Detection

```rust
use std::path::Path;
use onboard::run_onboarding;

fn main() -> anyhow::Result<()> {
    if !Path::new("dx.json").exists() {
        println!("First run! Starting setup...");
        run_onboarding()?;
    } else {
        println!("Welcome back!");
    }
    
    Ok(())
}
```

## Available Prompt Types

The library provides 24 different prompt types:

### Text Input
- `input::input()` - Basic text input
- `email::email()` - Email with validation
- `password::password()` - Secure password input
- `url::url()` - URL with validation
- `phone_input::phone_input()` - Phone number
- `text::text()` - Multi-line text
- `tags::tags()` - Comma-separated tags

### Numeric
- `number::number()` - Integer input with constraints
- `rating::rating()` - Star rating
- `slider::slider()` - Single value slider
- `range_slider::range_slider()` - Range selection

### Selection
- `select()` - Single selection menu
- `multiselect()` - Multiple selection menu
- `autocomplete::autocomplete()` - Autocomplete search
- `search_filter::search_filter()` - Filtered search

### Boolean
- `toggle::toggle()` - On/off toggle
- `confirm()` - Yes/no confirmation

### Progress
- `progress::ProgressBar` - Progress bar
- `spinner::spinner()` - Loading spinner

### Workflow
- `wizard::wizard()` - Multi-step wizard

### Logging
- `log::info()`, `log::success()`, `log::warning()`, `log::error()`, `log::step()`
- `intro()`, `outro()`, `section_with_width()`

### Visual
- `splash::render_dx_logo()` - Random ASCII logo
- `splash::render_train_animation()` - Train animation

## Documentation

All documentation is in the `onboard/` directory:

- **[README.md](onboard/README.md)** - Project overview and quick start
- **[PROMPTS.md](onboard/PROMPTS.md)** - Complete guide to all 24 prompts with code examples
- **[THEMING.md](onboard/THEMING.md)** - Theme customization guide
- **[EXAMPLES.md](onboard/EXAMPLES.md)** - Integration patterns and recipes

## API Documentation

Generate and view the API docs:

```bash
cargo doc --manifest-path onboard/Cargo.toml --open
```

## Key Features

1. **24 Interactive Prompts** - Complete set of TUI components
2. **Rainbow Effects** - Beautiful color animations
3. **Train Animation** - ASCII train on exit
4. **Theme System** - TOML-based customization
5. **Password Hashing** - Argon2 security
6. **Runtime Detection** - Automatic environment detection
7. **JSON Export** - Configuration saved automatically

## Dependencies

The onboard crate requires:
- `owo-colors` - Terminal colors
- `terminal_size` - Terminal dimensions
- `serde` + `toml` - Configuration
- `argon2` - Password hashing
- `chrono` - Timestamps
- `anyhow` - Error handling
- `ctrlc` - Signal handling
- `rand` - Random selection

These are already in `onboard/Cargo.toml` and will be included automatically.

## Terminal Requirements

- 24-bit true color support
- Minimum 80 columns width
- UTF-8 encoding
- ANSI escape sequences

## Common Integration Patterns

### Pattern 1: CLI Subcommand

```rust
use clap::{Parser, Subcommand};

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Setup,
    Run,
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    
    match cli.command {
        Commands::Setup => onboard::run_onboarding()?,
        Commands::Run => run_main_app()?,
    }
    
    Ok(())
}
```

### Pattern 2: Conditional Setup

```rust
fn main() -> anyhow::Result<()> {
    if needs_setup() {
        onboard::run_onboarding()?;
    }
    
    run_main_app()?;
    Ok(())
}

fn needs_setup() -> bool {
    !std::path::Path::new("dx.json").exists()
}
```

### Pattern 3: Custom Prompts Only

```rust
use onboard::prompts::*;

fn collect_user_info() -> anyhow::Result<(String, String)> {
    let name = input::input("Name?").interact()?;
    let email = email::email("Email?").interact()?;
    Ok((name, email))
}

fn main() -> anyhow::Result<()> {
    let (name, email) = collect_user_info()?;
    // Use the data
    Ok(())
}
```

## Testing

The onboard library is fully tested. To run tests:

```bash
cd onboard
cargo test
```

## Building

### Debug Build
```bash
cargo build --manifest-path onboard/Cargo.toml
```

### Release Build
```bash
cargo build --manifest-path onboard/Cargo.toml --release
```

### Standalone Binary
```bash
cd onboard
cargo run --release
```

## Customization

### Custom Theme

Edit `onboard/theme.toml`:

```toml
[colors]
primary = "cyan"
secondary = "blue"
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

### Custom Prompts

You can use individual prompts without running the full onboarding:

```rust
use onboard::prompts::*;

// Just get a name
let name = input::input("Name?").interact()?;

// Just get a theme choice
let theme = select("Theme?")
    .item("dark", "Dark", "")
    .item("light", "Light", "")
    .interact()?;
```

## Troubleshooting

### Colors Not Showing
- Ensure terminal supports 24-bit color
- Check `$COLORTERM` environment variable
- Try a different terminal emulator

### Train Animation Glitchy
- Terminal width should be at least 80 columns
- Check with `tput cols`

### Compilation Errors
- Ensure all dependencies are up to date
- Run `cargo clean` and rebuild
- Check Rust version (requires 1.75.0+)

## Next Steps

1. Read [onboard/PROMPTS.md](onboard/PROMPTS.md) for detailed prompt usage
2. Check [onboard/EXAMPLES.md](onboard/EXAMPLES.md) for integration patterns
3. Browse [onboard/THEMING.md](onboard/THEMING.md) for customization
4. Explore `onboard/src/prompts/` for implementation details
5. Generate API docs: `cargo doc --manifest-path onboard/Cargo.toml --open`

## Support

For issues or questions:
- Check the documentation in `onboard/`
- Review examples in `onboard/EXAMPLES.md`
- Look at prompt implementations in `onboard/src/prompts/`
- Generate API docs for detailed function signatures

## License

Same as parent DX project.
