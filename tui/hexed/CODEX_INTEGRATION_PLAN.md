# Codex-Rust CLI TUI Integration Plan
**Date:** March 24, 2026  
**Project:** Converting dx-based TUI fork to DX (Codex CLI Integration)

## Executive Summary

Based on current research, OpenAI's Codex CLI is a Rust-based terminal coding agent that reads codebases, writes files, and executes commands locally. Your TUI built on dx's codebase is nearly complete and needs to be rebranded and integrated with the Codex Rust CLI ecosystem.

## Current State Analysis

### What You Have
- ✅ Functional TUI built on dx's file browser codebase
- ✅ Custom menu system with Command Palette
- ✅ Theme system (DX theme with bright green primary)
- ✅ Keyboard shortcuts system with recording
- ✅ Animation effects and visual polish
- ✅ Chat interface with LLM integration
- ✅ File picker and browser functionality

### What Needs Transformation
- ❌ dx-branded folder names (dx-fm, dx-core, etc.)
- ❌ dx-specific terminology in code
- ❌ Package names and crate structure
- ❌ Configuration paths (~/.dx → ~/.codex or ~/.dx)
- ❌ Integration points with Codex CLI

---

## Phase 1: Codebase Rebranding (Week 1)

### 1.1 Folder Structure Transformation

**Current dx Structure:**
```
dx-fm/          → Main frontend
dx-core/        → Core logic
dx-config/      → Configuration
dx-adapter/     → Adapters
dx-plugin/      → Plugin system
dx-shared/      → Shared utilities
dx-proxy/       → Proxy layer
dx-dds/         → Data distribution
dx-boot/        → Bootstrap
dx-cli/         → CLI interface
```

**Proposed DX Structure (Option A - Codex-aligned):**
```
codex-tui/        → Main TUI interface (your custom UI)
codex-core/       → Core agent logic
codex-config/     → Configuration management
codex-adapter/    → Terminal/display adapters
codex-plugin/     → Plugin/MCP server system
codex-shared/     → Shared utilities
codex-bridge/     → Bridge to Codex CLI (renamed from proxy)
codex-session/    → Session management (renamed from dds)
codex-boot/       → Bootstrap and initialization
codex-cli/        → CLI entry point
```

**Proposed DX Structure (Option B - DX-branded):**
```
dx-tui/           → Main TUI interface
dx-core/          → Core logic
dx-config/        → Configuration
dx-adapter/       → Adapters
dx-plugin/        → Plugin system
dx-shared/        → Shared utilities
dx-bridge/        → Bridge layer
dx-session/       → Session management
dx-boot/          → Bootstrap
dx-cli/           → CLI interface
```

**Recommendation:** Use Option A (codex-*) for better alignment with the Codex ecosystem, or Option B (dx-*) if you want a distinct brand identity.

### 1.2 Automated Renaming Script

Create a migration script to handle the bulk renaming:

```bash
#!/bin/bash
# rename-to-codex.sh

OLD_PREFIX="dx"
NEW_PREFIX="codex"  # or "dx"

# Rename directories
for dir in ${OLD_PREFIX}-*; do
    if [ -d "$dir" ]; then
        new_name="${dir/${OLD_PREFIX}/${NEW_PREFIX}}"
        git mv "$dir" "$new_name"
        echo "Renamed: $dir → $new_name"
    fi
done

# Update Cargo.toml files
find . -name "Cargo.toml" -type f -exec sed -i "s/${OLD_PREFIX}-/${NEW_PREFIX}-/g" {} +

# Update Rust source files
find . -name "*.rs" -type f -exec sed -i "s/${OLD_PREFIX}_/${NEW_PREFIX}_/g" {} +
find . -name "*.rs" -type f -exec sed -i "s/use ${OLD_PREFIX}::/use ${NEW_PREFIX}::/g" {} +

# Update config paths
find . -name "*.rs" -type f -exec sed -i "s/\\.${OLD_PREFIX}/\\.${NEW_PREFIX}/g" {} +
find . -name "*.rs" -type f -exec sed -i "s/~\\/\\.${OLD_PREFIX}/~\\/\\.${NEW_PREFIX}/g" {} +

echo "✅ Renaming complete! Run 'cargo check' to verify."
```

### 1.3 Package Metadata Updates

Update all `Cargo.toml` files:

```toml
[package]
name = "codex-tui"  # or "dx-tui"
version = "0.1.0"
edition = "2024"
authors = ["Your Name <your.email@example.com>"]
description = "Terminal UI for Codex CLI - AI-powered coding agent"
repository = "https://github.com/yourusername/codex-tui"
license = "MIT"
keywords = ["codex", "tui", "ai", "coding-agent", "terminal"]
categories = ["command-line-utilities", "development-tools"]

[dependencies]
# Update all dx-* dependencies to codex-* or dx-*
codex-core = { path = "../codex-core" }
codex-config = { path = "../codex-config" }
# ... etc
```

---

## Phase 2: Codex CLI Integration (Week 2-3)

### 2.1 Understanding Codex Architecture

Based on research, Codex CLI has these key components:

1. **Agent Loop** - Orchestrates user → LLM → tools interactions
2. **Approval System** - Controls when to prompt for permissions
3. **Sandbox Mode** - Manages filesystem/shell access levels
4. **MCP Integration** - Model Context Protocol for external tools
5. **Session Management** - Persistent conversation history
6. **AGENTS.md** - Project-specific instructions

### 2.2 Integration Points

**Your TUI should integrate at these levels:**

```rust
// codex-bridge/src/lib.rs
pub struct CodexBridge {
    /// Connection to Codex CLI process
    cli_handle: CodexCliHandle,
    
    /// Session state
    session: SessionState,
    
    /// Approval policy
    approval_policy: ApprovalPolicy,
    
    /// Sandbox configuration
    sandbox: SandboxConfig,
    
    /// MCP server connections
    mcp_servers: Vec<McpServer>,
}

impl CodexBridge {
    /// Start Codex CLI in JSON mode
    pub fn spawn_codex_cli(&mut self) -> Result<()> {
        // codex exec --experimental-json
        // Parse JSONL output
    }
    
    /// Send message to Codex agent
    pub fn send_message(&mut self, msg: &str) -> Result<()> {
        // Send via JSONL protocol
    }
    
    /// Receive agent response
    pub fn receive_response(&mut self) -> Result<AgentResponse> {
        // Parse JSONL events
    }
    
    /// Handle approval requests
    pub fn handle_approval_request(&mut self, req: ApprovalRequest) -> Result<bool> {
        // Show TUI dialog for user approval
    }
}
```

### 2.3 JSONL Protocol Integration

Codex CLI supports `--experimental-json` mode for programmatic control:

```rust
// codex-session/src/jsonl.rs
#[derive(Deserialize)]
#[serde(tag = "type")]
pub enum CodexEvent {
    #[serde(rename = "message")]
    Message { content: String, role: String },
    
    #[serde(rename = "tool_call")]
    ToolCall { tool: String, args: serde_json::Value },
    
    #[serde(rename = "approval_request")]
    ApprovalRequest { action: String, risk_level: String },
    
    #[serde(rename = "status")]
    Status { tokens_used: usize, model: String },
    
    #[serde(rename = "error")]
    Error { message: String },
}

pub fn parse_codex_output(line: &str) -> Result<CodexEvent> {
    serde_json::from_str(line)
}
```

### 2.4 Configuration Compatibility

Make your TUI read Codex's config format:

```rust
// codex-config/src/lib.rs
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct CodexConfig {
    /// Model selection
    pub model: String,
    
    /// Reasoning effort level
    pub model_reasoning_effort: ReasoningEffort,
    
    /// Approval policy
    pub approval_policy: ApprovalPolicy,
    
    /// Sandbox mode
    pub sandbox_mode: SandboxMode,
    
    /// Web search setting
    pub web_search: WebSearchMode,
    
    /// Feature flags
    pub features: FeatureFlags,
    
    /// MCP servers
    pub mcp_servers: HashMap<String, McpServerConfig>,
    
    /// Named profiles
    pub profiles: HashMap<String, ProfileConfig>,
}

impl CodexConfig {
    /// Load from ~/.codex/config.toml
    pub fn load() -> Result<Self> {
        let path = dirs::home_dir()
            .ok_or("No home directory")?
            .join(".codex")
            .join("config.toml");
        
        let content = std::fs::read_to_string(path)?;
        toml::from_str(&content)
    }
}
```

---

## Phase 3: Feature Alignment (Week 3-4)

### 3.1 Command Palette Enhancement

Your existing Command Palette should map to Codex features:

```
Current Menu Items          →  Codex Integration
─────────────────────────────────────────────────
1. Theme                    →  Keep (TUI-specific)
2. Keyboard Shortcuts       →  Keep (TUI-specific)
3. Providers                →  Map to Model Selection
4. Plugins & Apps           →  Map to MCP Servers (/mcp, /apps)
5. Skills                   →  Map to AGENTS.md management
6. Sandbox                  →  Map to Sandbox Mode settings
7. Web Search               →  Map to Web Search toggle
8. MCP Servers              →  Direct MCP configuration
9. Memory & History         →  Map to Session management (/resume)
10. Multi-Agent             →  Map to multi_agent feature flag
11. Notifications           →  TUI notification system
12. Voice / Realtime        →  Future: voice input
13. Image & Vision          →  Map to --image flag support
14. Profiles                →  Map to --profile system
15. Worktree                →  Git integration
16. Authentication          →  OAuth/API key management
17. Network & Proxy         →  Network settings
18. Hooks & Events          →  Event system
19. Session Resume          →  /resume command
20. Approval Policy         →  /permissions command
21. Shell Environment       →  Shell tool configuration
22. Execution Rules         →  Approval rules
23. Project Trust           →  Project trust levels
24. Developer Instructions  →  AGENTS.md editor
25. Feature Flags           →  /experimental command
```

### 3.2 Slash Commands Integration

Implement Codex's slash commands in your TUI:

```rust
// codex-tui/src/commands.rs
pub enum SlashCommand {
    // Session
    New,
    Resume,
    Fork,
    Compact,
    Status,
    Quit,
    
    // Config
    Model,
    Permissions,
    Personality,
    Plan,
    Experimental,
    DebugConfig,
    Statusline,
    
    // Dev tools
    Diff,
    Review,
    Mention,
    Init,
    Mcp,
    Apps,
    
    // Other
    Feedback,
    Logout,
}

impl SlashCommand {
    pub fn parse(input: &str) -> Option<Self> {
        match input.trim_start_matches('/') {
            "new" => Some(Self::New),
            "resume" => Some(Self::Resume),
            // ... etc
            _ => None,
        }
    }
    
    pub fn execute(&self, ctx: &mut TuiContext) -> Result<()> {
        match self {
            Self::Model => ctx.show_model_picker(),
            Self::Permissions => ctx.show_permissions_dialog(),
            Self::Mcp => ctx.show_mcp_servers(),
            // ... etc
        }
    }
}
```

### 3.3 AGENTS.md Editor

Add a built-in editor for AGENTS.md files:

```rust
// codex-tui/src/agents_editor.rs
pub struct AgentsEditor {
    /// Current file being edited
    file_path: PathBuf,
    
    /// Editor content
    content: String,
    
    /// Cursor position
    cursor: (usize, usize),
}

impl AgentsEditor {
    /// Load AGENTS.md from project root
    pub fn load_project_agents() -> Result<Self> {
        let git_root = find_git_root()?;
        let path = git_root.join("AGENTS.md");
        // ... load file
    }
    
    /// Generate scaffold with /init
    pub fn generate_scaffold() -> String {
        r#"## Working agreements
- Always run tests after modifying source files
- Follow the project's code style guidelines

## Architecture
- Describe your project structure here

## Conventions
- List your coding conventions here
"#.to_string()
    }
}
```

---

## Phase 4: Visual Identity (Week 4)

### 4.1 Branding Updates

Replace all dx branding with DX/Codex branding:

```rust
// codex-tui/src/splash.rs
pub const SPLASH_LOGO: &str = r#"
   ██████╗ ██████╗ ██████╗ ███████╗██╗  ██╗
  ██╔════╝██╔═══██╗██╔══██╗██╔════╝╚██╗██╔╝
  ██║     ██║   ██║██║  ██║█████╗   ╚███╔╝ 
  ██║     ██║   ██║██║  ██║██╔══╝   ██╔██╗ 
  ╚██████╗╚██████╔╝██████╔╝███████╗██╔╝ ██╗
   ╚═════╝ ╚═════╝ ╚═════╝ ╚══════╝╚═╝  ╚═╝
   
   AI-Powered Terminal Coding Agent
"#;
```

### 4.2 Theme Consistency

Your DX theme (bright green rgb(0, 255, 42)) is already set up. Ensure it's used consistently:

```rust
// Keep your existing DX theme as the default
pub const DEFAULT_THEME: &str = "dx";
pub const DEFAULT_PRIMARY_COLOR: (u8, u8, u8) = (0, 255, 42);
```

---

## Phase 5: Documentation & Distribution (Week 5)

### 5.1 Update Documentation

Create comprehensive docs:

```
docs/
├── README.md              → Project overview
├── INSTALLATION.md        → Installation guide
├── CONFIGURATION.md       → Config reference
├── INTEGRATION.md         → Codex CLI integration
├── KEYBOARD_SHORTCUTS.md  → Shortcut reference
├── AGENTS_MD_GUIDE.md     → AGENTS.md best practices
└── DEVELOPMENT.md         → Contributing guide
```

### 5.2 Package Distribution

Prepare for distribution:

```toml
# Cargo.toml (workspace root)
[workspace]
members = [
    "codex-tui",
    "codex-core",
    "codex-config",
    "codex-adapter",
    "codex-plugin",
    "codex-shared",
    "codex-bridge",
    "codex-session",
    "codex-boot",
    "codex-cli",
]

[workspace.package]
version = "0.1.0"
edition = "2024"
license = "MIT"
repository = "https://github.com/yourusername/codex-tui"
```

### 5.3 Installation Methods

Support multiple installation methods:

```bash
# Cargo
cargo install codex-tui

# Homebrew (future)
brew install codex-tui

# Binary releases
# Provide pre-built binaries for:
# - x86_64-unknown-linux-gnu
# - x86_64-apple-darwin
# - aarch64-apple-darwin
# - x86_64-pc-windows-msvc
```

---

## Phase 6: Testing & Polish (Week 6)

### 6.1 Integration Testing

Test with real Codex CLI:

```bash
# Test basic integration
codex-tui --codex-path /usr/local/bin/codex

# Test with different models
codex-tui --model gpt-5.3-codex

# Test approval flows
codex-tui --approval-policy on-request

# Test MCP integration
codex-tui --mcp-server my-db
```

### 6.2 Performance Optimization

- Ensure TUI renders at 60fps
- Optimize file browser for large directories
- Cache theme data
- Lazy-load MCP servers

### 6.3 Error Handling

Add comprehensive error handling:

```rust
#[derive(Debug, thiserror::Error)]
pub enum CodexTuiError {
    #[error("Codex CLI not found: {0}")]
    CodexNotFound(String),
    
    #[error("Failed to parse Codex output: {0}")]
    ParseError(String),
    
    #[error("Configuration error: {0}")]
    ConfigError(String),
    
    #[error("MCP server error: {0}")]
    McpError(String),
}
```

---

## Recommended Action Plan

### Immediate (This Week)
1. ✅ Run the automated renaming script
2. ✅ Update all Cargo.toml files
3. ✅ Test compilation with `cargo check`
4. ✅ Update README.md with new branding

### Short-term (Next 2 Weeks)
1. Implement CodexBridge for CLI integration
2. Add JSONL protocol parsing
3. Implement slash commands
4. Add AGENTS.md editor
5. Test with real Codex CLI

### Medium-term (Next Month)
1. Polish UI/UX
2. Add comprehensive documentation
3. Create installation packages
4. Set up CI/CD pipeline
5. Prepare for public release

---

## Key Decisions Needed

1. **Naming:** `codex-tui` or `dx-tui`?
   - Recommendation: `codex-tui` for ecosystem alignment

2. **Integration Level:** Standalone or embedded?
   - Recommendation: Standalone TUI that launches Codex CLI

3. **Distribution:** Separate project or Codex CLI plugin?
   - Recommendation: Separate project, can be adopted by OpenAI later

4. **License:** MIT, Apache-2.0, or GPL?
   - Recommendation: MIT (matches Codex CLI)

---

## Resources & References

- [Codex CLI Documentation](https://github.com/openai/codex)
- [Codex CLI Cheat Sheet](https://shipyard.build/blog/codex-cli-cheat-sheet/)
- [MCP Protocol Specification](https://modelcontextprotocol.io/)
- [Rust Edition 2024 Guide](https://doc.rust-lang.org/edition-guide/)

---

## Success Metrics

- ✅ All dx references removed
- ✅ Compiles without warnings
- ✅ Successfully launches Codex CLI
- ✅ Can send/receive messages via JSONL
- ✅ Approval dialogs work correctly
- ✅ MCP servers can be configured
- ✅ AGENTS.md can be edited
- ✅ All slash commands implemented
- ✅ Documentation complete
- ✅ Ready for public release

---

**Next Steps:** Review this plan, make naming decisions, and start with Phase 1 automated renaming.
