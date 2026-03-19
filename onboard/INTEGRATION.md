# Integration Guide: DX Onboard

> Step-by-step guide to integrate the onboard TUI into your Rust application

## Quick Start

### Option 1: Standalone Executable

The simplest way to use onboard is as a standalone binary:

```bash
# Build the onboard binary
cd onboard
cargo build --release

# Run it
./target/release/onboard

# Configuration saved to dx.json
cat dx.json
```

### Option 2: Library Integration

Add onboard as a workspace member or path dependency:

```toml
# In your main Cargo.toml
[dependencies]
onboard = { path = "./onboard" }
```

Then call it from your code:

```rust
use onboard::run_onboarding;

fn main() -> anyhow::Result<()> {
    // Run the full onboarding flow
    let config = run_onboarding()?;
    
    // Access user data
    println!("Welcome, {}!", config.name);
    println!("Email: {}", config.email);
    
    // Use configuration
    setup_user_environment(&config)?;
    
    Ok(())
}
```

## Integration Patterns

### Pattern 1: First-Run Setup

Detect if this is the first run and trigger onboarding:

```rust
use std::path::Path;
use onboard::run_onboarding;

fn main() -> anyhow::Result<()> {
    let config_path = "dx.json";
    
    if !Path::new(config_path).exists() {
        println!("First run detected! Starting onboarding...");
        let config = run_onboarding()?;
        
        // Config is automatically saved to dx.json
        println!("Setup complete!");
    } else {
        println!("Loading existing configuration...");
        let config = load_config(config_path)?;
    }
    
    // Continue with main application
    run_main_app()?;
    
    Ok(())
}
```

### Pattern 2: Re-run Setup Command

Add a CLI command to re-run onboarding:

```rust
use clap::{Parser, Subcommand};
use onboard::run_onboarding;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Run the interactive setup wizard
    Setup,
    /// Start the main application
    Run,
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    
    match cli.command {
        Some(Commands::Setup) => {
            run_onboarding()?;
        }
        Some(Commands::Run) | None => {
            run_main_app()?;
        }
    }
    
    Ok(())
}
```

### Pattern 3: Conditional Onboarding

Only run certain parts of onboarding based on conditions:

```rust
use onboard::{prompts, effects::RainbowEffect};

fn setup_ai_providers() -> anyhow::Result<Vec<String>> {
    let rainbow = RainbowEffect::new();
    
    // Show just the provider selection prompt
    let providers = prompts::multiselect("Which AI providers would you like to configure?")
        .item("openai", "OpenAI", "GPT models")
        .item("anthropic", "Anthropic", "Claude models")
        .item("ollama", "Ollama", "Local models")
        .interact()?;
    
    Ok(providers)
}

fn main() -> anyhow::Result<()> {
    // Run only specific prompts as needed
    let providers = setup_ai_providers()?;
    
    for provider in providers {
        configure_provider(&provider)?;
    }
    
    Ok(())
}
```

### Pattern 4: Background Service Integration

Run onboarding before starting a background service:

```rust
use onboard::run_onboarding;
use tokio;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Run onboarding synchronously first
    let config = run_onboarding()?;
    
    // Then start async services
    let service = MyService::new(config);
    service.start().await?;
    
    Ok(())
}
```

## Accessing Configuration Data

The `OnboardingResult` struct contains all collected data:

```rust
use onboard::{run_onboarding, OnboardingResult};

fn process_config(config: &OnboardingResult) {
    // Basic info
    println!("Name: {}", config.name);
    println!("Email: {}", config.email);
    println!("Website: {}", config.website);
    println!("Phone: {}", config.phone);
    
    // Experience
    println!("Years of experience: {}", config.experience_years);
    println!("Satisfaction: {}/5", config.satisfaction_rating);
    println!("Productivity: {}%", config.productivity_level);
    
    // Work schedule
    let (start, end) = config.work_hours;
    println!("Work hours: {}:00 - {}:00", start, end);
    
    // Skills
    println!("Languages: {:?}", config.programming_languages);
    println!("Favorite: {}", config.favorite_language);
    println!("Framework: {}", config.framework);
    
    // Environment
    println!("Runtime: {}", config.runtime_environment);
    println!("Components: {:?}", config.selected_components);
    println!("Providers: {:?}", config.selected_providers);
    
    // Preferences
    println!("Theme: {}", config.preferences.theme);
    println!("Editor: {}", config.preferences.editor);
    println!("Shell: {}", config.preferences.shell);
    println!("Notifications: {}", config.preferences.notifications);
    println!("Auto-updates: {}", config.preferences.auto_updates);
    println!("Telemetry: {}", config.preferences.telemetry);
}
```

## Loading Saved Configuration

```rust
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Deserialize)]
struct SavedConfig {
    name: String,
    email: String,
    preferences: Preferences,
    // ... other fields
}

#[derive(Deserialize)]
struct Preferences {
    theme: String,
    editor: String,
    shell: String,
    notifications: bool,
    auto_updates: bool,
    telemetry: bool,
}

fn load_config(path: &str) -> anyhow::Result<SavedConfig> {
    let json = fs::read_to_string(path)?;
    let config: SavedConfig = serde_json::from_str(&json)?;
    Ok(config)
}

fn main() -> anyhow::Result<()> {
    let config = load_config("dx.json")?;
    
    // Apply theme
    apply_theme(&config.preferences.theme)?;
    
    // Configure editor integration
    setup_editor(&config.preferences.editor)?;
    
    Ok(())
}
```

## Custom Theme Integration

### Loading Custom Theme

```rust
use onboard::prompts::theme::DxTheme;
use std::fs;

fn load_custom_theme() -> anyhow::Result<DxTheme> {
    let theme_toml = fs::read_to_string("custom_theme.toml")?;
    let theme: DxTheme = toml::from_str(&theme_toml)?;
    Ok(theme)
}

fn main() -> anyhow::Result<()> {
    let theme = load_custom_theme()?;
    
    // Use theme in your prompts
    let name = onboard::prompts::input::input("Your name?")
        .interact()?;
    
    Ok(())
}
```

### Creating Theme Presets

```rust
use onboard::prompts::theme::{DxTheme, ColorScheme, Symbols};

fn create_dark_theme() -> DxTheme {
    DxTheme {
        colors: ColorScheme {
            primary: "cyan".to_string(),
            secondary: "blue".to_string(),
            success: "green".to_string(),
            warning: "yellow".to_string(),
            error: "red".to_string(),
            dim: "bright-black".to_string(),
        },
        symbols: Symbols::default(),
    }
}

fn create_light_theme() -> DxTheme {
    DxTheme {
        colors: ColorScheme {
            primary: "blue".to_string(),
            secondary: "magenta".to_string(),
            success: "green".to_string(),
            warning: "yellow".to_string(),
            error: "red".to_string(),
            dim: "black".to_string(),
        },
        symbols: Symbols::default(),
    }
}
```

## Error Handling

### Graceful Degradation

```rust
use onboard::run_onboarding;

fn main() -> anyhow::Result<()> {
    match run_onboarding() {
        Ok(config) => {
            println!("Onboarding completed!");
            save_config(&config)?;
        }
        Err(e) => {
            eprintln!("Onboarding failed: {}", e);
            
            // Fall back to defaults
            let default_config = create_default_config();
            save_config(&default_config)?;
            
            println!("Using default configuration");
        }
    }
    
    Ok(())
}
```

### User Cancellation

```rust
use onboard::run_onboarding;

fn main() -> anyhow::Result<()> {
    match run_onboarding() {
        Ok(config) => {
            // User completed onboarding
            proceed_with_setup(&config)?;
        }
        Err(e) if e.to_string().contains("cancelled") => {
            // User cancelled - exit gracefully
            println!("Setup cancelled. You can run 'dx setup' later.");
            std::process::exit(0);
        }
        Err(e) => {
            // Other error - report and exit
            eprintln!("Setup error: {}", e);
            std::process::exit(1);
        }
    }
    
    Ok(())
}
```

## Testing Integration

### Mock Onboarding for Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    fn mock_config() -> OnboardingResult {
        OnboardingResult {
            name: "Test User".to_string(),
            email: "test@example.com".to_string(),
            // ... other fields with test data
        }
    }
    
    #[test]
    fn test_config_processing() {
        let config = mock_config();
        assert_eq!(config.name, "Test User");
        
        // Test your config processing logic
        let result = process_config(&config);
        assert!(result.is_ok());
    }
}
```

### Integration Tests

```rust
#[cfg(test)]
mod integration_tests {
    use std::fs;
    
    #[test]
    fn test_config_file_creation() {
        // Run onboarding (would need to mock user input)
        // For now, test loading existing config
        
        let config_json = fs::read_to_string("test_fixtures/dx.json")
            .expect("Test config should exist");
        
        let config: serde_json::Value = serde_json::from_str(&config_json)
            .expect("Config should be valid JSON");
        
        assert!(config["name"].is_string());
        assert!(config["email"].is_string());
    }
}
```

## Performance Considerations

### Lazy Loading

```rust
use once_cell::sync::Lazy;
use onboard::OnboardingResult;

static CONFIG: Lazy<OnboardingResult> = Lazy::new(|| {
    load_config("dx.json").expect("Failed to load config")
});

fn main() {
    // Config is loaded only when first accessed
    println!("User: {}", CONFIG.name);
}
```

### Async Integration

```rust
use tokio;
use onboard::run_onboarding;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Run onboarding in blocking task
    let config = tokio::task::spawn_blocking(|| {
        run_onboarding()
    }).await??;
    
    // Continue with async code
    start_async_services(config).await?;
    
    Ok(())
}
```

## Workspace Integration

If onboard is part of a Cargo workspace:

```toml
# Root Cargo.toml
[workspace]
members = [
    "onboard",
    "main-app",
    "other-crates",
]

# main-app/Cargo.toml
[dependencies]
onboard = { path = "../onboard" }
```

Then use it:

```rust
// In main-app/src/main.rs
use onboard::run_onboarding;

fn main() -> anyhow::Result<()> {
    let config = run_onboarding()?;
    // Use config in main app
    Ok(())
}
```

## Environment Variables

Pass environment-specific settings:

```rust
use std::env;

fn main() -> anyhow::Result<()> {
    // Skip onboarding in CI
    if env::var("CI").is_ok() {
        println!("CI detected, using default config");
        return Ok(());
    }
    
    // Run onboarding for interactive sessions
    let config = onboard::run_onboarding()?;
    
    Ok(())
}
```

## Best Practices

1. **Always handle cancellation gracefully** - Users should be able to exit at any time
2. **Save configuration immediately** - Don't lose user input if something crashes later
3. **Validate loaded config** - Ensure saved JSON is valid before using
4. **Provide defaults** - Have sensible defaults if onboarding is skipped
5. **Make it optional** - Don't force users through onboarding every time
6. **Test thoroughly** - Mock user input for automated tests
7. **Document exit codes** - Make it clear what each exit code means

## Troubleshooting

### Issue: Onboarding hangs

**Solution**: Ensure stdin is available and terminal is interactive:

```rust
use std::io::IsTerminal;

fn main() -> anyhow::Result<()> {
    if !std::io::stdin().is_terminal() {
        eprintln!("Error: Onboarding requires an interactive terminal");
        std::process::exit(1);
    }
    
    run_onboarding()?;
    Ok(())
}
```

### Issue: Colors not showing

**Solution**: Check terminal capabilities:

```rust
use supports_color::Stream;

fn main() -> anyhow::Result<()> {
    if supports_color::on(Stream::Stdout).is_none() {
        eprintln!("Warning: Terminal doesn't support colors");
        // Continue anyway or exit
    }
    
    run_onboarding()?;
    Ok(())
}
```

### Issue: Train animation glitchy

**Solution**: Ensure terminal is wide enough:

```rust
use terminal_size::{Width, terminal_size};

fn main() -> anyhow::Result<()> {
    if let Some((Width(w), _)) = terminal_size() {
        if w < 80 {
            eprintln!("Warning: Terminal width ({}) is less than 80 columns", w);
            eprintln!("Animations may not display correctly");
        }
    }
    
    run_onboarding()?;
    Ok(())
}
```

## Support

For issues specific to onboard integration, check:
- README.md for general usage
- API.md for detailed API documentation
- Examples in the `examples/` directory
