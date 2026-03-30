# Codex Rust CLI TUI Integration Guide (Fork Strategy)

**Date**: March 25, 2026  
**Target**: Fork and integrate Codex Rust CLI codebase directly into dx-tui

## Overview

This guide explains how to fork the entire Codex Rust CLI codebase and integrate it directly into dx-tui. This approach gives you full control to modify Codex's rendering, remove branding, and customize behavior without being limited by the SDK.

### Strategy: Fork, Don't Depend

Instead of using `codex-client-sdk` as a dependency (one-way, limited control), you'll:

1. **Fork the Codex Rust repository** from OpenAI
2. **Extract the TUI rendering components** you need
3. **Integrate them directly** into your dx-tui codebase
4. **Modify freely** - strip branding, customize rendering, merge with your local LLM

This gives you bidirectional control and the ability to maintain your own version.

## Architecture

### Current State (dx-tui)

```
src/
├── chat.rs                 # ChatPanel with local LLM
├── chat_components.rs      # MessageList, Message, InputBox
└── chat_input.rs          # Input handling
```

### Target State (Forked Integration)

```
src/
├── chat.rs                 # ChatPanel with mode switching
├── chat_components.rs      # MessageList with variant support
├── chat_input.rs          # Input handling
├── codex/                  # NEW: Forked Codex components
│   ├── mod.rs             # Module exports
│   ├── client.rs          # Codex client (from fork)
│   ├── renderer.rs        # Message renderer (stripped/modified)
│   ├── events.rs          # Thread events
│   ├── items.rs           # Thread items
│   └── markdown.rs        # Markdown rendering (customized)
└── llm.rs                 # Your local LLM
```

## Step 1: Fork Codex Rust Repository

### 1.1 Find the Codex Rust Source

The Codex Rust CLI is part of the main Codex repository. You need to locate the Rust TUI implementation:

```bash
# Clone the Codex repository
git clone https://github.com/openai/codex.git codex-fork
cd codex-fork

# Find the Rust TUI code (likely in a subdirectory)
# Look for: rust/, tui-rust/, or similar
find . -name "Cargo.toml" -type f
```

### 1.2 Identify Key Components

Look for these files in the Codex Rust codebase:

- **Message rendering**: TUI components for displaying messages
- **Markdown parser**: Integration with `tui-markdown`
- **Event system**: `ThreadEvent`, `ThreadItem` types
- **Client logic**: API communication (you may not need this)
- **Theme/styling**: Color schemes and styling

### 1.3 Create Your Fork

```bash
# On GitHub, fork openai/codex to your account
# Then clone your fork
git clone https://github.com/YOUR_USERNAME/codex.git codex-fork
cd codex-fork

# Create a branch for dx-tui integration
git checkout -b dx-tui-integration
```

## Step 2: Extract Codex Components

### 2.1 Copy Relevant Files

From your Codex fork, copy the TUI rendering files into `src/codex/`:

```bash
# In your dx-tui project
mkdir -p src/codex

# Copy from Codex fork (adjust paths based on actual structure)
cp ../codex-fork/rust-tui/src/renderer.rs src/codex/
cp ../codex-fork/rust-tui/src/events.rs src/codex/
cp ../codex-fork/rust-tui/src/items.rs src/codex/
cp ../codex-fork/rust-tui/src/markdown.rs src/codex/
```

### 2.2 Create Module Structure

Create `src/codex/mod.rs`:

```rust
// Re-export the components you need
pub mod renderer;
pub mod events;
pub mod items;
pub mod markdown;

pub use renderer::CodexRenderer;
pub use events::{ThreadEvent, TurnEvent};
pub use items::{ThreadItem, MessageItem, ToolCallItem};
```

### 2.3 Strip Codex Branding

In the copied files, remove/modify:

- **Branding text**: Remove "Codex" logos, ASCII art, branding messages
- **Menu systems**: Strip out menu bars, status bars with Codex branding
- **Keyboard shortcuts overlay**: Remove or customize help screens
- **Color schemes**: Replace with your theme system

Example modifications in `src/codex/renderer.rs`:

```rust
// BEFORE (Codex original)
fn render_header(&self, area: Rect, buf: &mut Buffer) {
    let title = "Codex CLI - AI Coding Assistant";
    // ... branding rendering
}

// AFTER (Your version)
fn render_header(&self, area: Rect, buf: &mut Buffer) {
    // Remove or use your own branding
    let title = ""; // No branding
    // ... minimal rendering
}
```

## Step 3: Create Message Rendering Variants

### 3.1 Define Rendering Mode Enum

In `src/chat_components.rs`, add:

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MessageRenderMode {
    Local,  // Your existing local LLM rendering
    Codex,  // Codex CLI inline rendering
}
```

### 3.2 Update MessageList Structure

Modify `MessageList` to support both modes:

```rust
pub struct MessageList<'a> {
    messages: &'a [Message],
    theme: &'a ChatTheme,
    scroll_offset: usize,
    mode: MessageRenderMode,  // NEW
    // ... existing fields
}

impl<'a> MessageList<'a> {
    pub fn with_mode(
        messages: &'a [Message],
        theme: &'a ChatTheme,
        scroll_offset: usize,
        mode: MessageRenderMode,
    ) -> Self {
        Self {
            messages,
            theme,
            scroll_offset,
            mode,
            // ... existing fields
        }
    }
}
```

## Step 4: Adapt Codex Renderer to Your Theme

### 4.1 Replace Codex Theme System

In `src/codex/renderer.rs`, replace Codex's theme with yours:

```rust
// Import your theme
use crate::chat_components::ChatTheme;

pub struct CodexRenderer<'a> {
    items: &'a [ThreadItem],
    theme: &'a ChatTheme,  // Use YOUR theme, not Codex's
    scroll_offset: usize,
}

impl<'a> CodexRenderer<'a> {
    pub fn new(
        items: &'a [ThreadItem],
        theme: &'a ChatTheme,
        scroll_offset: usize,
    ) -> Self {
        Self {
            items,
            theme,
            scroll_offset,
        }
    }

    // Modify all rendering methods to use self.theme
    fn render_message(&self, item: &MessageItem) -> Vec<Line<'a>> {
        let role_style = match item.role.as_str() {
            "user" => Style::default()
                .fg(self.theme.user_message_fg),  // Your theme
            "assistant" => Style::default()
                .fg(self.theme.assistant_message_fg),  // Your theme
            _ => Style::default(),
        };
        
        // ... rest of rendering
    }
}
```

### 4.2 Remove Codex-Specific UI Elements

Strip out these components from the forked code:

```rust
// REMOVE: Status bar with Codex branding
// fn render_status_bar() { ... }

// REMOVE: Menu system
// fn render_menu() { ... }

// REMOVE: Keyboard shortcuts overlay
// fn render_shortcuts() { ... }

// KEEP: Core message rendering
fn render_message() { ... }
fn render_tool_call() { ... }
fn render_markdown() { ... }
```

## Step 5: Integrate with ChatPanel

In `src/chat.rs`, integrate the forked Codex components:

```rust
use crate::codex::{CodexRenderer, ThreadItem};

pub struct ChatPanel {
    pub input: InputState,
    pub messages: Vec<Message>,  // Local LLM messages
    pub codex_items: Vec<ThreadItem>,  // Codex thread items (from fork)
    pub is_loading: bool,
    pub mode: MessageRenderMode,
    pub llm: Arc<LocalLlm>,
    // No external Codex client - you control everything
    pub llm_tx: Sender<String>,
    pub llm_rx: Receiver<String>,
}

impl ChatPanel {
    pub fn new() -> Self {
        let (llm_tx, llm_rx) = channel();

        Self {
            input: InputState::new(),
            messages: Vec::new(),
            codex_items: Vec::new(),
            is_loading: false,
            mode: MessageRenderMode::Local,
            llm: Arc::new(LocalLlm::new()),
            llm_tx,
            llm_rx,
        }
    }

    /// You'll implement your own Codex API integration
    /// using the forked client code
    pub async fn send_codex_message(&mut self, content: String) -> anyhow::Result<()> {
        // Use your forked Codex client implementation
        // Customize API calls, add your own logic, etc.
        todo!("Implement using forked Codex client")
    }

    /// Toggle between Local and Codex modes
    pub fn toggle_mode(&mut self) {
        self.mode = match self.mode {
            MessageRenderMode::Local => MessageRenderMode::Codex,
            MessageRenderMode::Codex => MessageRenderMode::Local,
        };
    }
}
```

## Step 6: Update Rendering Logic

In `src/chat_components.rs`, update the `MessageList::render` method:

```rust
impl Widget for MessageList<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        match self.mode {
            MessageRenderMode::Local => {
                // Your existing local rendering logic
                self.render_local(area, buf);
            }
            MessageRenderMode::Codex => {
                // Use Codex renderer
                use crate::codex_renderer::CodexMessageRenderer;
                
                let renderer = CodexMessageRenderer::new(
                    &self.codex_items,  // Need to pass this
                    self.theme,
                    self.scroll_offset,
                );
                
                renderer.render(area, buf);
            }
        }
    }

    fn render_local(&self, area: Rect, buf: &mut Buffer) {
        // Move your existing rendering logic here
        // ... (current MessageList rendering code)
    }
}
```

## Step 7: Maintain Your Fork

### 7.1 Keep Fork Updated

```bash
# Add upstream remote
cd codex-fork
git remote add upstream https://github.com/openai/codex.git

# Fetch upstream changes
git fetch upstream

# Merge upstream changes into your branch
git checkout dx-tui-integration
git merge upstream/main

# Resolve conflicts (keep your customizations)
```

### 7.2 Track Your Modifications

Create a `FORK_CHANGES.md` in your fork:

```markdown
# Codex Fork Modifications for dx-tui

## Removed Components
- Status bar with Codex branding
- Menu system
- Keyboard shortcuts overlay
- Codex logo/ASCII art

## Modified Components
- Theme system (replaced with dx-tui ChatTheme)
- Message renderer (stripped branding)
- Markdown parser (customized styling)

## Added Components
- Integration with dx-tui local LLM
- Dual-mode rendering support
```

### 7.3 Sync Changes Back to dx-tui

When you update the fork:

```bash
# Copy updated files back to dx-tui
cp codex-fork/rust-tui/src/renderer.rs dx-tui/src/codex/
cp codex-fork/rust-tui/src/events.rs dx-tui/src/codex/

# Test integration
cd dx-tui
cargo build
cargo test
```

In your main event loop (likely in `src/main.rs` or dispatcher):

```rust
// Add key binding for mode toggle (e.g., Ctrl+M)
KeyCode::Char('m') if key.modifiers.contains(KeyModifiers::CONTROL) => {
    chat_panel.toggle_mode();
}

// When sending messages, route based on mode
KeyCode::Enter => {
    let content = chat_panel.input.content.clone();
    
    match chat_panel.mode {
        MessageRenderMode::Local => {
            chat_panel.send_message(content);
        }
        MessageRenderMode::Codex => {
            tokio::spawn(async move {
                if let Err(e) = chat_panel.send_codex_message(content).await {
                    eprintln!("Codex error: {}", e);
                }
            });
        }
    }
}
```

## Step 8: Handle Mode Switching in Main Loop

The key to stripping Codex branding is to:

1. **Don't use Codex's built-in TUI**: We're only using the SDK (`codex-client-sdk`), not the full CLI TUI
2. **Extract only ThreadItems**: The SDK gives you raw `ThreadItem` data without UI
3. **Custom rendering**: Your `CodexMessageRenderer` controls all visual aspects

### What to Skip

- ❌ Codex status bar
- ❌ Codex menu system
- ❌ Codex branding/logo
- ❌ Codex keyboard shortcuts overlay

### What to Keep

- ✅ Message content (user/assistant)
- ✅ Tool calls and results
- ✅ Markdown rendering
- ✅ Code blocks with syntax highlighting

## Step 8: Testing

### Test Local Mode

```bash
cargo run
# Should show your existing local LLM chat
```

### Test Codex Mode

```bash
# Set up Codex authentication first
export CODEX_API_KEY="your_key"

cargo run
# Press Ctrl+M to switch to Codex mode
# Send a message - should render via Codex
```

## Key Differences: Local vs Codex Rendering

| Feature | Local Mode | Codex Mode |
|---------|-----------|------------|
| Data Source | `Vec<Message>` | `Vec<ThreadItem>` |
| Markdown | Custom parser | `tui-markdown` |
| Tool Calls | N/A | Rendered inline |
| Streaming | Custom via mpsc | Codex SDK events |
| Branding | Your theme | Stripped (custom) |

## Performance Considerations

1. **Lazy Loading**: Only render visible messages in both modes
2. **Caching**: Cache markdown parsing results for Codex messages
3. **Async**: Keep Codex API calls non-blocking with tokio::spawn
4. **Memory**: Clear old `codex_items` after a threshold (e.g., 1000 items)

## Step 11: Finding Codex Rust Source

### Where to Look

The Codex repository structure (as of March 2026):

```
openai/codex/
├── src/                    # TypeScript CLI (main)
├── rust-tui/              # Rust TUI implementation (what you need!)
│   ├── src/
│   │   ├── renderer.rs    # Message rendering
│   │   ├── events.rs      # Event types
│   │   ├── items.rs       # Thread items
│   │   ├── markdown.rs    # Markdown parsing
│   │   ├── client.rs      # API client
│   │   └── main.rs        # Entry point
│   └── Cargo.toml
└── README.md
```

### If Rust Code Isn't in Main Repo

Check these locations:

1. **Separate repo**: `openai/codex-rust` or `openai/codex-tui`
2. **Branch**: Look for `rust-tui` or `tui-rust` branches
3. **Submodule**: Check `.gitmodules` for Rust submodules
4. **Releases**: Download Rust source from GitHub releases

### Verify You Have the Right Code

Look for these markers:

```rust
// Should see Ratatui usage
use ratatui::{...};

// Should see tui-markdown
use tui_markdown::...;

// Should see ThreadItem types
pub enum ThreadItem {
    Message { ... },
    ToolCall { ... },
    ...
}
```

## Next Steps

1. **Fork Codex**: Create your fork on GitHub
2. **Locate Rust TUI code**: Find the Rust implementation in the repo
3. **Extract components**: Copy renderer, events, items to `src/codex/`
4. **Strip branding**: Remove Codex UI elements you don't want
5. **Integrate**: Wire up with your ChatPanel and MessageList
6. **Test both modes**: Verify Local and Codex rendering work
7. **Maintain fork**: Set up upstream tracking for updates

## Troubleshooting

### Can't Find Rust Code in Codex Repo

- Check if there's a separate `codex-rust` repository
- Look in GitHub releases for Rust source archives
- Search issues/discussions for "Rust TUI" or "Ratatui"
- The Rust implementation might be closed-source (check license)

### Compilation Errors After Copying

- Update `Cargo.toml` dependencies to match your versions
- Adjust imports to match your module structure
- Replace Codex-specific types with your equivalents
- Check Rust edition compatibility (2024 vs 2021)

### Rendering Differences

- Codex may use different Ratatui widgets than you
- Theme color mappings might need adjustment
- Markdown parser behavior could differ
- Test with simple messages first, then complex ones

## References

- [Codex GitHub](https://github.com/openai/codex) - Main repository
- [Ratatui docs](https://ratatui.rs) - TUI framework
- [tui-markdown](https://lib.rs/crates/tui-markdown) - Markdown rendering
- [Rust Edition 2024](https://doc.rust-lang.org/edition-guide/) - Latest Rust features

## Fork Strategy Summary

| Approach | Control | Maintenance | Flexibility |
|----------|---------|-------------|-------------|
| **SDK Dependency** | ❌ Limited | ✅ Easy | ❌ Constrained |
| **Fork (Your Choice)** | ✅ Full | ⚠️ Manual | ✅ Unlimited |

You chose the fork strategy for maximum control and customization. This lets you modify Codex's rendering directly, strip branding completely, and integrate deeply with your local LLM system.

---

**Implementation Time Estimate**: 
- Fork setup: 1-2 hours
- Component extraction: 2-4 hours  
- Branding removal: 1-2 hours
- Integration: 2-4 hours
- Testing & polish: 2-4 hours
- **Total**: 1-2 days for complete integration
