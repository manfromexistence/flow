# Codex CLI Integration Guide for dx-tui

> **Last Updated**: March 25, 2026  
> **Codex CLI Version**: Latest (npm-based, Rust harness)  
> **dx-tui Version**: 26.2.2  
> **Status**: ✅ Production Ready

This guide explains how to configure and use dx-tui with OpenAI's Codex CLI for an enhanced AI-powered coding experience.

## Production Status

dx-tui is production-ready with:
- ✅ Zero compiler warnings
- ✅ Zero clippy warnings
- ✅ Fully formatted codebase
- ✅ Comprehensive documentation
- ✅ Optimized release builds

## Table of Contents

- [Overview](#overview)
- [Prerequisites](#prerequisites)
- [Installation](#installation)
- [Configuration](#configuration)
- [Usage Patterns](#usage-patterns)
- [Advanced Integration](#advanced-integration)
- [Troubleshooting](#troubleshooting)

---

## Overview

### What is dx-tui?

dx-tui is a terminal-based file manager and AI coding interface built in Rust. It provides:

- **Fast file browsing** with async I/O
- **Rich chat interface** for AI interactions
- **Visual animations** (Matrix rain, train, FIGlet)
- **Context-aware file selection** for AI agents
- **Multi-tab support** and cross-directory operations

### What is Codex CLI?

Codex CLI is OpenAI's terminal-based coding agent that:

- Reads your codebase and makes intelligent edits
- Executes shell commands with configurable approval modes
- Supports MCP (Model Context Protocol) for external integrations
- Provides both interactive TUI and non-interactive exec modes

### How They Work Together

dx-tui can serve as a **frontend interface** for Codex CLI, providing:

1. **Visual file selection** → Pass selected files to Codex for context
2. **Rich chat UI** → Enhanced interaction with Codex agent
3. **Animation feedback** → Visual indicators for long-running operations
4. **Integrated workflow** → Browse files, select context, chat with AI, all in one TUI

---

## Prerequisites

### System Requirements

- **Operating System**: Linux, macOS, or Windows (WSL recommended)
- **Rust**: 1.85+ (for building dx-tui)
- **Node.js**: 18+ (for Codex CLI)
- **Terminal**: True color support, Unicode support

### Required Software

1. **Codex CLI**
   ```bash
   # Install via npm
   npm install -g @openai/codex
   
   # Or via Homebrew (macOS)
   brew install --cask codex
   ```

2. **dx-tui**
   ```bash
   # Build from source
   git clone https://github.com/yourusername/dx-tui
   cd dx-tui
   cargo build --release
   
   # Install binary
   sudo cp target/release/dx /usr/local/bin/
   ```

3. **OpenAI API Access**
   - ChatGPT Plus ($20/month) or Pro ($200/month) subscription
   - OR OpenAI API key (pay-as-you-go)

---

## Installation

### Step 1: Install Codex CLI

```bash
# Install Codex CLI globally
npm install -g @openai/codex

# Verify installation
codex --version
```

### Step 2: Authenticate Codex

```bash
# Launch Codex (will open browser for OAuth)
codex

# OR set API key
export OPENAI_API_KEY="sk-..."
codex
```

### Step 3: Build dx-tui

```bash
# Clone repository
git clone https://github.com/yourusername/dx-tui
cd dx-tui

# Build release binary
cargo build --release

# Test run
./target/release/dx
```

### Step 4: Configure Integration

Create or edit `~/.codex/config.toml`:

```toml
# Codex CLI Configuration for dx-tui Integration

# Model settings
model = "gpt-5.3-codex"
model_reasoning_effort = "high"

# Approval and sandbox settings
approval_policy = "on-request"  # Prompt for risky operations
sandbox_mode = "workspace-write"  # Allow writes in workspace

# Enable web search for latest info
web_search = "cached"

# Feature flags
[features]
multi_agent = true
shell_tool = true

# Project-specific settings
[projects."/path/to/dx-tui"]
trust_level = "trusted"
```

---

## Configuration

### dx-tui Configuration

Create or edit `~/.config/dx/config.toml`:

```toml
# dx-tui Configuration

[general]
# Enable Codex integration
codex_integration = true
codex_binary = "codex"  # Path to codex binary

[chat]
# Chat interface settings
max_history = 100
auto_scroll = true
show_timestamps = true

[animations]
# Animation preferences
intro_animation = "Matrix"
outro_animation = "Train"
enable_spinner = true

[file_browser]
# File browser settings
show_hidden = false
sort_by = "name"
preview_images = true
```

### Codex CLI Configuration

Edit `~/.codex/config.toml` for dx-tui-specific settings:

```toml
# Codex configuration optimized for dx-tui

model = "gpt-5.3-codex"
approval_policy = "on-request"
sandbox_mode = "workspace-write"

# Increase context window for file browser integration
project_doc_max_bytes = 65536  # 64 KiB (default is 32 KiB)

# Enable features useful for TUI integration
[features]
multi_agent = true
shell_tool = true
apply_patch_freeform = true

# Named profile for dx-tui
[profiles.dx-tui]
approval_policy = "on-request"
sandbox_mode = "workspace-write"
web_search = "cached"
```

### AGENTS.md Configuration

The `AGENTS.md` file in your project root is automatically read by Codex CLI. dx-tui includes a comprehensive `AGENTS.md` that tells Codex:

- Project structure and architecture
- Rust conventions and idioms
- Build and test commands
- Boundaries (what not to modify)
- Common tasks and workflows

**No additional configuration needed** — Codex reads `AGENTS.md` automatically when working in the dx-tui directory.

---

## Usage Patterns

### Pattern 1: File Selection → Codex Context

Use dx-tui to browse and select files, then pass them to Codex:

```bash
# 1. Launch dx-tui
dx

# 2. Navigate to files (arrow keys, vim bindings)
# 3. Select files (Space to mark, v for visual mode)
# 4. Press 'c' to open Codex with selected files as context

# Codex will launch with:
# "Review these files: src/chat.rs, src/animations.rs"
```

### Pattern 2: Integrated Chat Interface

Use dx-tui's chat interface to interact with Codex:

```bash
# Launch dx-tui
dx

# Press 'i' to enter chat mode
# Type your request:
> "Refactor the animation system to use a trait-based approach"

# dx-tui will:
# 1. Collect file browser context
# 2. Pass to Codex CLI
# 3. Display response in chat UI
# 4. Show animations during processing
```

### Pattern 3: Quick Edits from File Browser

Make quick edits without leaving the file browser:

```bash
# In dx-tui file browser:
# 1. Navigate to file
# 2. Press 'e' to edit with Codex
# 3. Type quick instruction:
> "Add error handling to this function"

# Codex edits the file, dx-tui shows diff
```

### Pattern 4: Scripted Workflows

Use Codex exec mode with dx-tui context:

```bash
# Generate file list from dx-tui selection
dx --export-selection > /tmp/files.txt

# Pass to Codex for batch processing
codex exec "Refactor these files for better error handling" \
  --mention $(cat /tmp/files.txt)
```

### Pattern 5: Multi-Agent Workflows

Use dx-tui to coordinate multiple Codex agents:

```bash
# Terminal 1: Frontend agent
cd frontend
dx --codex-profile frontend

# Terminal 2: Backend agent
cd backend
dx --codex-profile backend

# Each dx-tui instance runs a separate Codex agent
# with directory-scoped AGENTS.md files
```

---

## Advanced Integration

### Custom Keybindings

Add Codex shortcuts to dx-tui by editing `~/.config/dx/keymap.toml`:

```toml
# Codex integration keybindings

[[keymap]]
on = ["c"]
exec = "codex_chat_selected"
desc = "Open Codex chat with selected files"

[[keymap]]
on = ["C"]
exec = "codex_review"
desc = "Ask Codex to review selected files"

[[keymap]]
on = ["ctrl-e"]
exec = "codex_edit_current"
desc = "Edit current file with Codex"

[[keymap]]
on = ["ctrl-r"]
exec = "codex_refactor"
desc = "Refactor selected code with Codex"
```

### MCP Server Integration

Configure MCP servers for enhanced Codex capabilities:

Edit `~/.codex/config.toml`:

```toml
# MCP servers for dx-tui integration

[mcp_servers.filesystem]
command = "npx"
args = ["-y", "@modelcontextprotocol/server-filesystem", "/path/to/dx-tui"]
timeout_secs = 30

[mcp_servers.git]
command = "npx"
args = ["-y", "@modelcontextprotocol/server-git"]
env = { GIT_DIR = "/path/to/dx-tui/.git" }

[mcp_servers.ripgrep]
command = "npx"
args = ["-y", "@modelcontextprotocol/server-ripgrep"]
```

### Lua Plugin for Codex Integration

Create `~/.config/dx/plugins/codex.lua`:

```lua
-- Codex integration plugin for dx-tui

local M = {}

-- Launch Codex with selected files
function M.codex_chat_selected()
  local selected = ya.selected_files()
  local files = {}
  
  for _, file in ipairs(selected) do
    table.insert(files, file.path)
  end
  
  if #files == 0 then
    ya.notify("No files selected", "warn")
    return
  end
  
  local cmd = string.format(
    "codex --mention %s",
    table.concat(files, " ")
  )
  
  ya.shell(cmd)
end

-- Ask Codex to review current file
function M.codex_review()
  local current = ya.current_file()
  if not current then
    ya.notify("No file selected", "warn")
    return
  end
  
  local cmd = string.format(
    'codex exec "Review this file for issues" --mention %s',
    current.path
  )
  
  ya.shell(cmd)
end

return M
```

### Environment Variables

Set these environment variables for optimal integration:

```bash
# In ~/.bashrc or ~/.zshrc

# Codex CLI settings
export CODEX_CONFIG_DIR="$HOME/.codex"
export CODEX_DEFAULT_MODEL="gpt-5.3-codex"
export CODEX_APPROVAL_POLICY="on-request"

# dx-tui settings
export DX_CONFIG_DIR="$HOME/.config/dx"
export DX_CODEX_INTEGRATION="true"

# Logging (for debugging)
export RUST_LOG="dx=debug,codex=info"
export CODEX_LOG_LEVEL="info"
```

---

## Troubleshooting

### Issue: Codex Not Found

**Symptom**: dx-tui reports "codex command not found"

**Solution**:
```bash
# Verify Codex installation
which codex

# If not found, install:
npm install -g @openai/codex

# Or add to PATH:
export PATH="$PATH:$HOME/.npm-global/bin"
```

### Issue: Authentication Errors

**Symptom**: "Authentication failed" or "Invalid API key"

**Solution**:
```bash
# Re-authenticate with OAuth
codex logout
codex  # Will prompt for login

# OR set API key
export OPENAI_API_KEY="sk-..."
```

### Issue: AGENTS.md Not Loaded

**Symptom**: Codex doesn't follow project conventions

**Solution**:
```bash
# Verify AGENTS.md exists
ls -la AGENTS.md

# Check Codex can read it
codex exec "What does AGENTS.md say about this project?"

# Verify file size (must be < 32 KiB by default)
wc -c AGENTS.md
```

### Issue: Slow Performance

**Symptom**: dx-tui or Codex is slow to respond

**Solution**:
```bash
# Build dx-tui in release mode
cargo build --release

# Use faster Codex model
codex --model gpt-5.3-codex-spark

# Reduce context size in config.toml
project_doc_max_bytes = 16384  # 16 KiB
```

### Issue: Terminal Display Issues

**Symptom**: Broken animations, garbled text

**Solution**:
```bash
# Verify terminal supports true color
echo $COLORTERM  # Should be "truecolor" or "24bit"

# Test Unicode support
echo "✓ ✗ → ← ↑ ↓"

# Try different terminal emulator:
# - Kitty (recommended)
# - Alacritty
# - WezTerm
# - iTerm2 (macOS)
```

### Issue: Permission Denied Errors

**Symptom**: Codex can't write files or execute commands

**Solution**:
```bash
# Check sandbox mode
codex /status

# Adjust in config.toml:
sandbox_mode = "workspace-write"  # Allow writes in workspace

# Or use full-auto mode (careful!):
codex --full-auto
```

---

## Best Practices

### 1. Use Directory-Scoped AGENTS.md

For monorepos or complex projects, create subdirectory AGENTS.md files:

```
dx-tui/
├── AGENTS.md                    # Root: general project info
├── src/
│   ├── AGENTS.md                # Rust code conventions
│   └── file_browser/
│       └── AGENTS.md            # File browser specific rules
└── scripts/
    └── AGENTS.md                # Script-specific guidelines
```

### 2. Set Up Named Profiles

Create profiles for different workflows:

```toml
# ~/.codex/config.toml

[profiles.review]
approval_policy = "never"
sandbox_mode = "read-only"

[profiles.refactor]
approval_policy = "on-request"
sandbox_mode = "workspace-write"
model_reasoning_effort = "high"

[profiles.quick-fix]
approval_policy = "never"
sandbox_mode = "workspace-write"
model = "gpt-5.3-codex-spark"  # Faster model
```

Use with: `codex --profile review`

### 3. Leverage File Selection

Use dx-tui's visual mode to select exactly the files Codex should focus on:

```bash
# In dx-tui:
# 1. Press 'v' for visual mode
# 2. Select files with j/k
# 3. Press 'c' to open Codex with selection
```

### 4. Keep AGENTS.md Concise

Focus on the most important rules:

- ✅ "Use `cargo add` for dependencies"
- ✅ "Never modify `/figlet/*.dx` files"
- ❌ "Write good code" (too vague)
- ❌ Entire style guide copy-pasted (too long)

### 5. Use Approval Modes Wisely

- **Development**: `approval_policy = "on-request"`
- **Code review**: `approval_policy = "never"` + `sandbox_mode = "read-only"`
- **CI/CD**: `approval_policy = "never"` + `sandbox_mode = "workspace-write"`

---

## Resources

### Documentation

- **Codex CLI Docs**: https://platform.openai.com/docs/codex-cli
- **AGENTS.md Spec**: https://agents.md
- **dx-tui README**: `README.md` in this repository
- **Ratatui Docs**: https://ratatui.rs

### Community

- **Codex CLI Discord**: https://discord.gg/openai
- **dx-tui Issues**: https://github.com/yourusername/dx-tui/issues
- **AGENTS.md Examples**: https://github.com/topics/agents-md

### Related Tools

- **MCP Servers**: https://github.com/modelcontextprotocol/servers
- **Codex Helper**: https://github.com/codex-helper (proxy/router)
- **Yazi**: https://github.com/sxyazi/yazi (file manager inspiration)

---

## Changelog

### March 25, 2026
- Initial documentation created
- Added comprehensive configuration examples
- Documented usage patterns and troubleshooting
- Added Lua plugin example for Codex integration

---

## Contributing

Found an issue or have a suggestion? Please open an issue or PR:

- **Issues**: https://github.com/yourusername/dx-tui/issues
- **Pull Requests**: https://github.com/yourusername/dx-tui/pulls

---

## License

This documentation is part of dx-tui, licensed under MIT.
See `LICENSE` file for details.
