# Architecture: DX Onboard

> Technical architecture and design decisions for the onboard TUI

## Overview

DX Onboard is a standalone TUI (Terminal User Interface) application built in Rust that provides an interactive onboarding experience. It's designed to be integrated into the larger DX platform as either a standalone binary or a library crate.

## Design Principles

1. **Modularity** - Each prompt type is self-contained
2. **Composability** - Prompts can be used individually or as a complete flow
3. **Theming** - Visual appearance is configurable via TOML
4. **Zero Dependencies on Parent** - Can run completely standalone
5. **Graceful Degradation** - Handles terminal limitations gracefully

## Architecture Layers

```
┌─────────────────────────────────────────────────────────┐
│                    Application Layer                     │
│                      (main.rs)                          │
│  - Orchestrates onboarding flow                         │
│  - Handles exit scenarios                               │
│  - Saves configuration                                  │
└─────────────────────────────────────────────────────────┘
                            │
                            ↓
┌─────────────────────────────────────────────────────────┐
│                   Presentation Layer                     │
│              (splash.rs, effects.rs)                    │
│  - ASCII art rendering                                  │
│  - Rainbow color effects                                │
│  - Train animations                                     │
└─────────────────────────────────────────────────────────┘
                            │
                            ↓
┌─────────────────────────────────────────────────────────┐
│                    Prompt Layer                         │
│                  (prompts/*)                            │
│  - 24 different prompt types                            │
│  - Input validation                                     │
│  - User interaction                                     │
└─────────────────────────────────────────────────────────┘
                            │
                            ↓
┌─────────────────────────────────────────────────────────┐
│                    Theme Layer                          │
│                (prompts/theme.rs)                       │
│  - Color scheme management                              │
│  - Symbol customization                                 │
│  - TOML configuration                                   │
└─────────────────────────────────────────────────────────┘
                            │
                            ↓
┌─────────────────────────────────────────────────────────┐
│                  Terminal Layer                         │
│         (owo-colors, terminal_size)                     │
│  - ANSI color codes                                     │
│  - Terminal size detection                              │
│  - Cursor control                                       │
└─────────────────────────────────────────────────────────┘
```

## Module Structure

### Core Modules

#### `main.rs`
- **Purpose**: Application entry point and orchestration
- **Responsibilities**:
  - Run complete onboarding flow
  - Collect user data through prompts
  - Handle Ctrl+C gracefully
  - Save configuration to JSON
  - Show train animations on exit
- **Key Functions**:
  - `main()` - Entry point
  - `async_main()` - Main async wrapper with exit handlers
  - `run_onboarding()` - Complete onboarding flow
  - `detect_runtime_environment()` - Detect execution environment
  - `hash_password()` / `verify_password()` - Password security

#### `splash.rs`
- **Purpose**: Visual splash screens and animations
- **Responsibilities**:
  - Render DX logo with random fonts
  - Animate train with smoke effects
  - Apply rainbow colors
- **Key Functions**:
  - `render_dx_logo()` - Show random ASCII logo
  - `render_train_animation()` - Animate train across screen
- **Constants**:
  - `DX_LOGOS` - 10 hardcoded ASCII art logos

#### `effects.rs`
- **Purpose**: Visual effects system
- **Responsibilities**:
  - Generate rainbow color gradients
  - Cycle colors smoothly
- **Key Types**:
  - `RainbowEffect` - Rainbow color generator
  - `Color` - RGB color struct

### Prompt Modules

All prompts follow a consistent pattern:

```rust
pub struct PromptType {
    message: String,
    // ... configuration fields
}

impl PromptType {
    pub fn new(message: &str) -> Self { ... }
    
    pub fn configure_option(mut self, value: T) -> Self {
        self.option = value;
        self
    }
    
    pub fn interact(self) -> Result<ReturnType> {
        // Show prompt
        // Get user input
        // Validate
        // Return result
    }
}
```

#### Input Prompts
- `input.rs` - Basic text input
- `email.rs` - Email with validation
- `password.rs` - Masked password input
- `url.rs` - URL with validation
- `phone_input.rs` - Phone number input
- `text.rs` - Multi-line text area
- `tags.rs` - Comma-separated tags

#### Numeric Prompts
- `number.rs` - Integer input with constraints
- `rating.rs` - Star rating selector
- `slider.rs` - Single value slider
- `range_slider.rs` - Range selection

#### Selection Prompts
- `select.rs` - Single selection menu
- `multiselect.rs` - Multiple selection menu
- `autocomplete.rs` - Autocomplete search
- `search_filter.rs` - Filtered search with tags

#### Boolean Prompts
- `toggle.rs` - On/off toggle switch
- `confirm.rs` - Yes/no confirmation

#### Progress Indicators
- `progress.rs` - Progress bar
- `spinner.rs` - Loading spinner

#### Workflow Prompts
- `wizard.rs` - Multi-step wizard

#### Utility Modules
- `log.rs` - Logging functions (info, success, warning, error)
- `mod.rs` - Module exports and shared traits

### Theme System

#### `theme.rs`
- **Purpose**: Centralized theming system
- **Responsibilities**:
  - Load theme from TOML
  - Provide color scheme
  - Provide symbol set
  - Apply colors to text
- **Key Types**:
  - `DxTheme` - Complete theme configuration
  - `ColorScheme` - Color definitions
  - `Symbols` - Symbol characters
  - `ThemeColor` - Individual color with apply method

## Data Flow

### Onboarding Flow

```
User starts app
      ↓
Clear screen
      ↓
Show DX logo (random font)
      ↓
Run 24 prompts sequentially
      ↓
Collect all responses
      ↓
Create OnboardingResult
      ↓
Save to dx.json
      ↓
Show success message
      ↓
Show train animation
      ↓
Exit
```

### Exit Scenarios

```
┌─────────────────┐
│  Exit Trigger   │
└────────┬────────┘
         │
    ┌────┴────┐
    │         │
    ↓         ↓
Ctrl+C    Success/Error
    │         │
    └────┬────┘
         ↓
  Show train animation
         ↓
      Exit(0/1)
```

### Prompt Interaction

```
Create prompt
      ↓
Configure options
      ↓
Call .interact()
      ↓
Display prompt
      ↓
Wait for input
      ↓
Validate input
      ↓
Return result or error
```

## State Management

### Global State
- **None** - Application is stateless
- Each prompt is independent
- Configuration is collected and returned

### Local State
- Each prompt maintains its own state during interaction
- State is discarded after prompt completes

### Persistent State
- Configuration saved to `dx.json`
- Theme loaded from `theme.toml`

## Error Handling

### Error Types
1. **User Cancellation** - User presses Ctrl+C or cancels
2. **Validation Errors** - Invalid input (email, URL, etc.)
3. **I/O Errors** - File read/write failures
4. **Terminal Errors** - Terminal not interactive

### Error Propagation
- All functions return `Result<T, anyhow::Error>`
- Errors bubble up to main
- Main handles errors and shows appropriate messages

### Recovery Strategies
1. **Retry** - For transient errors
2. **Default** - Use default values
3. **Exit** - For fatal errors
4. **Skip** - For optional features

## Performance Considerations

### Memory Usage
- **Startup**: ~5MB
- **Runtime**: ~5-10MB
- **Peak**: ~15MB (during animation)

### CPU Usage
- **Idle**: <1%
- **Animation**: 5-10%
- **Input**: <1%

### Startup Time
- **Cold start**: ~100ms
- **Warm start**: ~50ms

### Optimization Strategies
1. **Lazy Loading** - Load resources on demand
2. **Efficient Rendering** - Only redraw changed areas
3. **Minimal Dependencies** - Keep dependency tree small
4. **Release Builds** - Use `--release` for production

## Security Considerations

### Password Handling
- Passwords are masked during input
- Hashed with Argon2 (industry standard)
- Never stored in plain text
- Hash verification uses constant-time comparison

### Input Validation
- Email validation with regex
- URL validation with basic checks
- Phone number format validation
- Numeric range validation

### File Permissions
- Configuration files created with user-only permissions
- No sensitive data in logs
- No network communication

## Terminal Compatibility

### Supported Terminals
- **Linux**: GNOME Terminal, Konsole, Alacritty, Kitty, etc.
- **macOS**: Terminal.app, iTerm2, Alacritty, Kitty
- **Windows**: Windows Terminal, ConEmu, Alacritty

### Required Features
- 24-bit true color support
- ANSI escape sequences
- Minimum 80 columns width
- UTF-8 encoding

### Graceful Degradation
- Falls back to basic colors if true color unavailable
- Adjusts to terminal width
- Handles non-interactive terminals

## Testing Strategy

### Unit Tests
- Test individual prompt logic
- Test validation functions
- Test theme loading
- Test color generation

### Integration Tests
- Test complete onboarding flow (with mocked input)
- Test configuration saving/loading
- Test error scenarios

### Manual Tests
- Visual inspection of animations
- Color rendering verification
- Terminal compatibility testing

## Build Configuration

### Debug Build
```bash
cargo build
```
- Includes debug symbols
- No optimizations
- Fast compilation
- ~3MB binary

### Release Build
```bash
cargo build --release
```
- Optimized for speed
- Stripped symbols
- Slower compilation
- ~1.2MB binary

### Dependencies
- **Runtime**: 45 crates
- **Build**: 2 additional crates
- **Total**: 47 crates

## Future Enhancements

### Planned Features
1. **More Prompt Types**
   - Date/time picker
   - Color picker
   - File browser
   - Tree selector

2. **Enhanced Animations**
   - More ASCII art options
   - Configurable animation speed
   - Custom animation frames

3. **Accessibility**
   - Screen reader support
   - High contrast mode
   - Keyboard shortcuts

4. **Internationalization**
   - Multi-language support
   - Locale-aware formatting
   - RTL text support

5. **Web Preview**
   - Browser-based preview mode
   - Share onboarding flow
   - Remote configuration

### Technical Debt
- Add comprehensive unit tests
- Improve error messages
- Add logging framework
- Document internal APIs

## Integration Points

### As Library
```rust
use onboard::run_onboarding;
let config = run_onboarding()?;
```

### As Binary
```bash
./onboard
cat dx.json
```

### As Workspace Member
```toml
[workspace]
members = ["onboard", "main-app"]
```

## Deployment

### Standalone Binary
- Single executable
- No runtime dependencies
- Cross-platform compatible

### Library Crate
- Path dependency
- Workspace member
- Git submodule

### Distribution
- GitHub releases
- Cargo registry (future)
- Docker image (future)

## Monitoring & Observability

### Logging
- Currently uses `println!` and `eprintln!`
- Future: Structured logging with `tracing`

### Metrics
- No metrics currently
- Future: Track completion rates, prompt times

### Telemetry
- Optional telemetry flag in preferences
- No telemetry implemented yet

## License & Attribution

Same license as parent DX project.

Uses open-source dependencies:
- `owo-colors` - Terminal colors
- `terminal_size` - Terminal dimensions
- `serde` + `toml` - Configuration
- `argon2` - Password hashing
- `chrono` - Timestamps
- `anyhow` - Error handling
- `ctrlc` - Signal handling
