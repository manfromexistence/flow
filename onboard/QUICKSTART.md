# Quick Start Guide: DX Onboard

> Get up and running with DX Onboard in 5 minutes

## Installation

### As Standalone Binary

```bash
# Clone or navigate to the onboard directory
cd onboard

# Build release binary
cargo build --release

# Run it
./target/release/onboard
```

### As Library Dependency

Add to your `Cargo.toml`:

```toml
[dependencies]
onboard = { path = "./onboard" }
```

## Basic Usage

### Run Complete Onboarding

```rust
use onboard::run_onboarding;

fn main() -> anyhow::Result<()> {
    // Run the full interactive onboarding
    let config = run_onboarding()?;
    
    // Access the results
    println!("Welcome, {}!", config.name);
    println!("Email: {}", config.email);
    
    // Configuration is automatically saved to dx.json
    
    Ok(())
}
```

### Use Individual Prompts

```rust
use onboard::prompts::*;

fn main() -> anyhow::Result<()> {
    // Text input
    let name = input::input("Your name?")
        .placeholder("John Doe")
        .interact()?;
    
    // Email with validation
    let email = email::email("Your email?")
        .interact()?;
    
    // Single selection
    let theme = select("Choose theme")
        .item("dark", "Dark", "Night mode")
        .item("light", "Light", "Day mode")
        .interact()?;
    
    // Multiple selection
    let languages = multiselect("Select languages")
        .item("rust".to_string(), "Rust".to_string(), "Systems")
        .item("python".to_string(), "Python".to_string(), "Scripting")
        .interact()?;
    
    // Boolean toggle
    let notifications = toggle::toggle("Enable notifications?")
        .initial_value(true)
        .interact()?;
    
    println!("Name: {}", name);
    println!("Email: {}", email);
    println!("Theme: {}", theme);
    println!("Languages: {:?}", languages);
    println!("Notifications: {}", notifications);
    
    Ok(())
}
```

## Common Patterns

### First-Run Detection

```rust
use std::path::Path;
use onboard::run_onboarding;

fn main() -> anyhow::Result<()> {
    if !Path::new("dx.json").exists() {
        println!("First run! Let's set things up...");
        run_onboarding()?;
    } else {
        println!("Welcome back!");
    }
    
    Ok(())
}
```

### CLI Integration

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
        Commands::Setup => {
            onboard::run_onboarding()?;
        }
        Commands::Run => {
            // Run main app
        }
    }
    
    Ok(())
}
```

### Load Saved Configuration

```rust
use serde::Deserialize;
use std::fs;

#[derive(Deserialize)]
struct Config {
    name: String,
    email: String,
    preferences: Preferences,
}

#[derive(Deserialize)]
struct Preferences {
    theme: String,
    notifications: bool,
}

fn main() -> anyhow::Result<()> {
    let json = fs::read_to_string("dx.json")?;
    let config: Config = serde_json::from_str(&json)?;
    
    println!("User: {}", config.name);
    println!("Theme: {}", config.preferences.theme);
    
    Ok(())
}
```

## Visual Effects

### Rainbow Colors

```rust
use onboard::effects::RainbowEffect;
use owo_colors::OwoColorize;

fn main() {
    let rainbow = RainbowEffect::new();
    
    let text = "Hello, Rainbow!";
    for (i, ch) in text.chars().enumerate() {
        let color = rainbow.color_at(i);
        print!("{}", ch.to_string().truecolor(color.r, color.g, color.b));
    }
    println!();
}
```

### Splash Screen

```rust
use onboard::splash::render_dx_logo;
use onboard::effects::RainbowEffect;

fn main() -> std::io::Result<()> {
    let rainbow = RainbowEffect::new();
    
    // Clear screen
    print!("\x1B[2J\x1B[H");
    
    // Show logo
    render_dx_logo(&rainbow)?;
    
    Ok(())
}
```

### Train Animation

```rust
use onboard::splash::render_train_animation;
use onboard::effects::RainbowEffect;
use std::thread;
use std::time::Duration;

fn main() -> std::io::Result<()> {
    let rainbow = RainbowEffect::new();
    
    // Clear screen
    print!("\x1B[2J\x1B[H");
    
    // Animate train for 15 frames
    for frame in 0..15 {
        print!("\x1B[H"); // Move cursor to top
        render_train_animation(&rainbow, frame)?;
        thread::sleep(Duration::from_millis(200));
    }
    
    Ok(())
}
```

## Customization

### Custom Theme

Create `custom_theme.toml`:

```toml
[colors]
primary = "magenta"
secondary = "cyan"
success = "green"
warning = "yellow"
error = "red"
dim = "bright-black"

[symbols]
checkmark = "✓"
cross = "✗"
info = "ℹ"
warning = "⚠"
error = "✗"
arrow = "→"
bullet = "•"
```

Load it:

```rust
use onboard::prompts::theme::DxTheme;
use std::fs;

fn main() -> anyhow::Result<()> {
    let theme_toml = fs::read_to_string("custom_theme.toml")?;
    let theme: DxTheme = toml::from_str(&theme_toml)?;
    
    // Use theme in your prompts
    // (Theme is automatically loaded from theme.toml by default)
    
    Ok(())
}
```

## Error Handling

### Handle User Cancellation

```rust
use onboard::run_onboarding;

fn main() -> anyhow::Result<()> {
    match run_onboarding() {
        Ok(config) => {
            println!("Setup complete!");
        }
        Err(e) if e.to_string().contains("cancelled") => {
            println!("Setup cancelled. Run 'dx setup' to try again.");
            std::process::exit(0);
        }
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    }
    
    Ok(())
}
```

### Graceful Degradation

```rust
use onboard::run_onboarding;

fn main() -> anyhow::Result<()> {
    let config = match run_onboarding() {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Onboarding failed: {}", e);
            eprintln!("Using default configuration");
            return Ok(()); // Continue with defaults
        }
    };
    
    // Use config
    println!("User: {}", config.name);
    
    Ok(())
}
```

## Testing

### Mock Configuration

```rust
#[cfg(test)]
mod tests {
    use onboard::OnboardingResult;
    
    fn mock_config() -> OnboardingResult {
        // Create test configuration
        OnboardingResult {
            name: "Test User".to_string(),
            email: "test@example.com".to_string(),
            // ... other fields
        }
    }
    
    #[test]
    fn test_config() {
        let config = mock_config();
        assert_eq!(config.name, "Test User");
    }
}
```

## Tips & Tricks

### Skip Onboarding in CI

```rust
use std::env;

fn main() -> anyhow::Result<()> {
    if env::var("CI").is_ok() {
        println!("CI detected, skipping onboarding");
        return Ok(());
    }
    
    onboard::run_onboarding()?;
    Ok(())
}
```

### Check Terminal Capabilities

```rust
use std::io::IsTerminal;

fn main() -> anyhow::Result<()> {
    if !std::io::stdin().is_terminal() {
        eprintln!("Error: Interactive terminal required");
        std::process::exit(1);
    }
    
    onboard::run_onboarding()?;
    Ok(())
}
```

### Async Integration

```rust
use tokio;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Run onboarding in blocking task
    let config = tokio::task::spawn_blocking(|| {
        onboard::run_onboarding()
    }).await??;
    
    // Continue with async code
    println!("User: {}", config.name);
    
    Ok(())
}
```

## What's Next?

- Read [README.md](README.md) for complete feature overview
- Check [INTEGRATION.md](INTEGRATION.md) for detailed integration patterns
- Browse [API.md](API.md) for full API reference
- Explore `src/prompts/` for prompt implementations
- Customize `theme.toml` for your brand colors

## Common Issues

**Q: Colors not showing?**  
A: Ensure your terminal supports 24-bit color (most modern terminals do)

**Q: Train animation glitchy?**  
A: Terminal width should be at least 80 columns

**Q: Onboarding hangs?**  
A: Make sure stdin is available and terminal is interactive

**Q: How to skip certain prompts?**  
A: Use individual prompt functions instead of `run_onboarding()`

## Support

For issues or questions:
- Check the documentation files in this directory
- Review examples in `src/main.rs`
- Look at prompt implementations in `src/prompts/`

## License

Same as parent DX project.
