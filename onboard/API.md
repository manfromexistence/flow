# API Documentation: DX Onboard

> Complete API reference for the onboard TUI library

## Core Functions

### `run_onboarding() -> Result<OnboardingResult>`

Main entry point that runs the complete onboarding flow.

**Returns**: `OnboardingResult` containing all collected user data

**Errors**: Returns `anyhow::Error` if:
- User cancels the onboarding
- Terminal is not interactive
- I/O errors occur
- Configuration save fails

**Example**:
```rust
use onboard::run_onboarding;

fn main() -> anyhow::Result<()> {
    let result = run_onboarding()?;
    println!("Welcome, {}!", result.name);
    Ok(())
}
```

## Data Structures

### `OnboardingResult`

Complete onboarding data collected from the user.

```rust
pub struct OnboardingResult {
    // Basic Info
    pub name: String,
    pub email: String,
    pub website: String,
    pub phone: String,
    pub bio: String,
    
    // Experience & Skills
    pub experience_years: i64,
    pub satisfaction_rating: usize,
    pub productivity_level: i64,
    pub work_hours: (i64, i64),
    pub programming_languages: Vec<String>,
    pub favorite_language: String,
    pub framework: String,
    pub project_type: String,
    pub selected_skills: Vec<String>,
    
    // System & Environment
    pub runtime_environment: String,
    pub selected_components: Vec<String>,
    pub selected_providers: Vec<String>,
    
    // Preferences
    pub preferences: OnboardingPreferences,
    
    // Workflow Data
    pub wizard_completed_steps: usize,
    
    // Metadata
    pub timestamp: String,
}
```

### `OnboardingPreferences`

User preference settings.

```rust
pub struct OnboardingPreferences {
    pub theme: String,
    pub editor: String,
    pub shell: String,
    pub notifications: bool,
    pub auto_updates: bool,
    pub telemetry: bool,
}
```

## Prompt Modules

All prompts are available under `onboard::prompts::*`

### Input Prompts

#### `input::input(message: &str) -> InputPrompt`

Basic text input prompt.

**Methods**:
- `.placeholder(text: &str)` - Set placeholder text
- `.initial_value(value: &str)` - Set initial value
- `.interact() -> Result<String>` - Show prompt and get input

**Example**:
```rust
use onboard::prompts::input;

let name = input::input("What's your name?")
    .placeholder("John Doe")
    .interact()?;
```

#### `email::email(message: &str) -> EmailPrompt`

Email input with validation.

**Methods**:
- `.initial_value(email: &str)` - Set initial email
- `.interact() -> Result<String>` - Show prompt and get validated email

**Example**:
```rust
use onboard::prompts::email;

let email = email::email("Your email?")
    .initial_value("user@example.com")
    .interact()?;
```

#### `password::password(message: &str) -> PasswordPrompt`

Secure password input (masked).

**Methods**:
- `.interact() -> Result<String>` - Show prompt and get password

**Example**:
```rust
use onboard::prompts::password;

let pwd = password::password("Enter password")
    .interact()?;
```

#### `url::url(message: &str) -> UrlPrompt`

URL input with validation.

**Methods**:
- `.interact() -> Result<String>` - Show prompt and get validated URL

**Example**:
```rust
use onboard::prompts::url;

let website = url::url("Your website?")
    .interact()?;
```

#### `phone_input::phone_input(message: &str) -> PhonePrompt`

Phone number input.

**Methods**:
- `.interact() -> Result<String>` - Show prompt and get phone number

**Example**:
```rust
use onboard::prompts::phone_input;

let phone = phone_input::phone_input("Phone number?")
    .interact()?;
```

### Numeric Prompts

#### `number::number(message: &str) -> NumberPrompt`

Numeric input with constraints.

**Methods**:
- `.min(value: i64)` - Set minimum value
- `.max(value: i64)` - Set maximum value
- `.interact() -> Result<i64>` - Show prompt and get number

**Example**:
```rust
use onboard::prompts::number;

let age = number::number("Your age?")
    .min(0)
    .max(120)
    .interact()?;
```

#### `rating::rating(message: &str) -> RatingPrompt`

Star rating selector.

**Methods**:
- `.max(stars: usize)` - Set maximum stars (default: 5)
- `.interact() -> Result<usize>` - Show prompt and get rating

**Example**:
```rust
use onboard::prompts::rating;

let rating = rating::rating("Rate your experience")
    .max(5)
    .interact()?;
```

#### `slider::slider(message: &str, min: i64, max: i64) -> SliderPrompt`

Single value slider.

**Methods**:
- `.initial_value(value: i64)` - Set initial position
- `.interact() -> Result<i64>` - Show prompt and get value

**Example**:
```rust
use onboard::prompts::slider;

let volume = slider::slider("Volume", 0, 100)
    .initial_value(50)
    .interact()?;
```

#### `range_slider::range_slider(message: &str, min: i64, max: i64) -> RangeSliderPrompt`

Range selection slider.

**Methods**:
- `.initial_range(start: i64, end: i64)` - Set initial range
- `.interact() -> Result<(i64, i64)>` - Show prompt and get range

**Example**:
```rust
use onboard::prompts::range_slider;

let (start, end) = range_slider::range_slider("Work hours", 0, 24)
    .initial_range(9, 17)
    .interact()?;
```

### Selection Prompts

#### `select(message: &str) -> SelectPrompt`

Single selection menu.

**Methods**:
- `.item(value: &str, label: &str, hint: &str)` - Add option
- `.interact() -> Result<String>` - Show prompt and get selection

**Example**:
```rust
use onboard::prompts::select;

let theme = select("Choose theme")
    .item("dark", "Dark Theme", "For night owls")
    .item("light", "Light Theme", "For day time")
    .interact()?;
```

#### `multiselect(message: &str) -> MultiSelectPrompt`

Multiple selection menu.

**Methods**:
- `.item(value: String, label: String, hint: &str)` - Add option
- `.required(required: bool)` - Require at least one selection
- `.interact() -> Result<Vec<String>>` - Show prompt and get selections

**Example**:
```rust
use onboard::prompts::multiselect;

let langs = multiselect("Select languages")
    .item("rust".to_string(), "Rust".to_string(), "Systems language")
    .item("python".to_string(), "Python".to_string(), "Scripting language")
    .interact()?;
```

#### `autocomplete::autocomplete(message: &str) -> AutocompletePrompt`

Autocomplete search prompt.

**Methods**:
- `.item(value: &str, label: &str)` - Add option
- `.interact() -> Result<String>` - Show prompt and get selection

**Example**:
```rust
use onboard::prompts::autocomplete;

let lang = autocomplete::autocomplete("Favorite language?")
    .item("rust", "Rust")
    .item("python", "Python")
    .interact()?;
```

#### `search_filter::search_filter(message: &str) -> SearchFilterPrompt`

Searchable filtered list.

**Methods**:
- `.item(value: &str, label: &str, tags: Vec<String>)` - Add option with tags
- `.interact() -> Result<String>` - Show prompt and get selection

**Example**:
```rust
use onboard::prompts::search_filter;

let framework = search_filter::search_filter("Choose framework")
    .item("react", "React", vec!["frontend".to_string(), "javascript".to_string()])
    .item("vue", "Vue.js", vec!["frontend".to_string(), "javascript".to_string()])
    .interact()?;
```

### Text Prompts

#### `text::text(message: &str) -> TextPrompt`

Multi-line text area.

**Methods**:
- `.placeholder(text: &str)` - Set placeholder
- `.interact() -> Result<String>` - Show prompt and get text

**Example**:
```rust
use onboard::prompts::text;

let bio = text::text("Tell us about yourself")
    .placeholder("Write your bio...")
    .interact()?;
```

#### `tags::tags(message: &str) -> TagsPrompt`

Comma-separated tag input.

**Methods**:
- `.placeholder(text: &str)` - Set placeholder
- `.interact() -> Result<Vec<String>>` - Show prompt and get tags

**Example**:
```rust
use onboard::prompts::tags;

let skills = tags::tags("Your skills?")
    .placeholder("rust, python, javascript")
    .interact()?;
```

### Boolean Prompts

#### `toggle::toggle(message: &str) -> TogglePrompt`

Boolean toggle switch.

**Methods**:
- `.initial_value(value: bool)` - Set initial state
- `.interact() -> Result<bool>` - Show prompt and get value

**Example**:
```rust
use onboard::prompts::toggle;

let enabled = toggle::toggle("Enable notifications?")
    .initial_value(true)
    .interact()?;
```

#### `confirm(message: &str) -> ConfirmPrompt`

Yes/No confirmation.

**Methods**:
- `.initial_value(value: bool)` - Set initial value
- `.interact() -> Result<bool>` - Show prompt and get confirmation

**Example**:
```rust
use onboard::prompts::confirm;

let proceed = confirm("Continue with setup?")
    .initial_value(true)
    .interact()?;
```

### Progress Indicators

#### `progress::ProgressBar::new(message: &str, total: usize) -> ProgressBar`

Progress bar indicator.

**Methods**:
- `.start() -> Result<()>` - Start showing progress
- `.set(value: usize) -> Result<()>` - Update progress
- `.set_message(msg: &str) -> Result<()>` - Update message
- `.finish(msg: &str) -> Result<()>` - Complete progress

**Example**:
```rust
use onboard::prompts::progress::ProgressBar;

let mut progress = ProgressBar::new("Installing", 100);
progress.start()?;
progress.set(50)?;
progress.set_message("Almost done...")?;
progress.finish("Complete!")?;
```

#### `spinner::spinner(message: &str) -> Spinner`

Loading spinner.

**Methods**:
- `.start() -> Result<()>` - Start spinner
- `.stop(message: &str) -> Result<()>` - Stop spinner with message

**Example**:
```rust
use onboard::prompts::spinner;

let mut spinner = spinner::spinner("Loading...");
spinner.start()?;
// Do work
spinner.stop("Done!")?;
```

### Workflow Prompts

#### `wizard::wizard(title: &str) -> WizardPrompt`

Multi-step wizard.

**Methods**:
- `.step(name: &str, description: &str)` - Add step
- `.interact() -> Result<usize>` - Show wizard and get completed steps

**Example**:
```rust
use onboard::prompts::wizard;

let steps = wizard::wizard("Setup Wizard")
    .step("Step 1", "Basic info")
    .step("Step 2", "Preferences")
    .step("Step 3", "Review")
    .interact()?;
```

### Logging Functions

#### `log::info(message: String) -> Result<()>`

Display info message.

#### `log::success(message: String) -> Result<()>`

Display success message.

#### `log::warning(message: String) -> Result<()>`

Display warning message.

#### `log::error(message: String) -> Result<()>`

Display error message.

#### `log::step(message: &str) -> Result<()>`

Display step message.

**Example**:
```rust
use onboard::prompts::log;

log::info("Starting setup...".to_string())?;
log::success("Configuration saved!".to_string())?;
log::warning("Using default theme".to_string())?;
log::error("Failed to connect".to_string())?;
log::step("Installing dependencies")?;
```

### Layout Functions

#### `intro(message: &str) -> Result<()>`

Display intro message.

#### `outro(message: &str) -> Result<()>`

Display outro message.

#### `section_with_width(title: &str, width: usize, content: F) -> Result<()>`

Display formatted section.

**Example**:
```rust
use onboard::prompts::{intro, outro, section_with_width};

intro("Welcome to DX!")?;

section_with_width("Summary", 80, |lines| {
    lines.push("Line 1".to_string());
    lines.push("Line 2".to_string());
})?;

outro("Setup complete!")?;
```

## Visual Effects

### `RainbowEffect`

Rainbow color cycling effect.

```rust
use onboard::effects::RainbowEffect;

let rainbow = RainbowEffect::new();
let color = rainbow.color_at(index);
// color has .r, .g, .b fields (u8)
```

## Splash Screen

### `render_dx_logo(rainbow: &RainbowEffect) -> io::Result<()>`

Render random DX logo with rainbow colors.

### `render_train_animation(rainbow: &RainbowEffect, frame: usize) -> io::Result<()>`

Render animated train with smoke effects.

**Example**:
```rust
use onboard::splash::{render_dx_logo, render_train_animation};
use onboard::effects::RainbowEffect;

let rainbow = RainbowEffect::new();

// Show logo
render_dx_logo(&rainbow)?;

// Show train animation
for frame in 0..15 {
    render_train_animation(&rainbow, frame)?;
    std::thread::sleep(std::time::Duration::from_millis(200));
}
```

## Theme System

### `DxTheme`

Theme configuration structure.

```rust
use onboard::prompts::theme::{DxTheme, ColorScheme, Symbols};

let theme = DxTheme {
    colors: ColorScheme {
        primary: "cyan".to_string(),
        secondary: "blue".to_string(),
        success: "green".to_string(),
        warning: "yellow".to_string(),
        error: "red".to_string(),
        dim: "bright-black".to_string(),
    },
    symbols: Symbols {
        checkmark: "√".to_string(),
        cross: "×".to_string(),
        info: "i".to_string(),
        warning: "!".to_string(),
        error: "×".to_string(),
        arrow: "→".to_string(),
        bullet: "•".to_string(),
    },
};
```

## Error Handling

All functions return `Result<T, anyhow::Error>` for consistent error handling.

Common error scenarios:
- User cancellation (Ctrl+C or explicit cancel)
- Terminal not interactive
- I/O errors
- Validation failures

**Example**:
```rust
use onboard::run_onboarding;

match run_onboarding() {
    Ok(config) => {
        // Success
    }
    Err(e) if e.to_string().contains("cancelled") => {
        // User cancelled
    }
    Err(e) => {
        // Other error
        eprintln!("Error: {}", e);
    }
}
```

## Type Aliases

```rust
pub type Result<T> = anyhow::Result<T>;
```

## Feature Flags

Currently no feature flags. All functionality is included by default.

## Platform Support

- **Linux**: Full support
- **macOS**: Full support
- **Windows**: Full support (requires Windows Terminal or compatible)

## Minimum Rust Version

Requires Rust 1.75.0 or later (uses `let-else` statements).

## Thread Safety

The onboard library is not thread-safe. Run onboarding on the main thread only.

## Performance Characteristics

- **Memory**: ~5MB runtime
- **Startup**: ~100ms
- **Animation**: 60 FPS (limited by terminal refresh rate)

## Deprecation Policy

No deprecated APIs currently. Future deprecations will follow semantic versioning.
