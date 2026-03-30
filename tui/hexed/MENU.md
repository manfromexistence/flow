in our tui please add these new menu items:
This is an excellent project! Based on my deep dive into the codex-rs source code (`features.rs`, `config/mod.rs`), the official config reference, the sample config, changelogs, and the Elixir SDK documentation, here is a **comprehensive menu structure** for your TUI that covers every hidden and known feature:

---

## 🎛️ Complete TUI Menu Structure

### ✅ Your Existing 5 Menus
1. **Providers**
2. **Keyboard Shortcuts**
3. **Theme**
4. **Worktree**
5. **Sandbox**

---

### 🆕 Menus to Add (20+ menus mined from codex-rs internals)

---

#### 6. 🤖 **Model Configuration**
Covers `model`, `model_reasoning_effort` (low/medium/high/xhigh), `model_personality` (e.g. "concise", "pragmatic"), `review_model`, and `service_tier` (fast/standard).

| Setting | Config Key | Values |
|---|---|---|
| Default Model | `model` | any model string |
| Reasoning Effort | `model_reasoning_effort` | `low`, `medium`, `high`, `xhigh` |
| Personality | `model_personality` | `concise`, `pragmatic`, etc. |
| Review Model | `review_model` | any model string |
| Service Tier | `service_tier` | `fast`, `standard` |
| Model Catalog JSON | `model_catalog_json` | path to JSON file |

---

#### 7. 🔐 **Approval Policy**
Controls when Codex asks for permission before acting. Four values: `untrusted` asks before every single action — every file read, every command, every write. Also `on-request`, `never`, and granular approval policy for individual prompt categories like `request_permissions` or skill-script prompts.

| Setting | Config Key | Values |
|---|---|---|
| Approval Policy | `approval_policy` | `untrusted`, `on-request`, `never`, `{granular = {...}}` |

---

#### 8. 🌐 **Web Search**
Optional web search tool configuration. The legacy boolean form is still accepted, but the object form lets you set search context size, allowed domains, and approximate user location.

| Setting | Config Key | Values |
|---|---|---|
| Mode | `web_search` | `"live"`, `"cached"`, `"disabled"` |
| Context Size | `web_search.context_size` | `low`, `medium`, `high` |
| Allowed Domains | `web_search.allowed_domains` | `[string]` |
| Location | `web_search.location` | `{country, region, city, timezone}` |

---

#### 9. 🔌 **MCP Servers**
Codex CLI functions as an MCP client that allows the Codex CLI and IDE extension to connect to MCP servers on startup. Codex can be launched as an MCP server, allowing other MCP clients to use Codex as a tool for another agent.

| Setting | Config Key | Values |
|---|---|---|
| STDIO Server | `[mcp_servers.<id>]` | `command`, `enabled`, `startup_timeout_sec` |
| HTTP Server | `[mcp_servers.<id>]` | `url`, `enabled` |
| OAuth Credential Store | `mcp_oauth_credentials_store` | `keyring`, `file`, `auto` |
| OAuth Callback Port | `mcp_oauth_callback_port` | port number |

---

#### 10. 🎭 **Profiles**
Define profiles under `[profiles.<name>]` in config.toml, then run `codex --profile <name>`. Codex loads that profile unless you override it on the command line.

| Setting | Config Key | Values |
|---|---|---|
| Active Profile | `profile` | string name |
| Profile Definitions | `[profiles.<name>]` | model, reasoning, approval, etc. |

---

#### 11. 🎯 **Feature Flags** *(the hidden beast)*
The feature flag system provides centralized control over optional and experimental capabilities. Features are defined in a static registry and resolved at runtime.

Extracted from `features.rs` enum:

| Feature Flag | Config Key | Description |
|---|---|---|
| Unified Exec | `unified_exec` | Unified command execution pipeline |
| Shell Snapshot | `shell_snapshot` | Experimental shell snapshotting for speed |
| Request Rule | `request_rule` | Allow model to propose exec rules |
| Undo | `undo` | Undo support for actions |
| Search Tool | `search_tool` | Web search tool |
| Codex Git Commit | `codex_git_commit` | Git commit attribution guidance |
| Runtime Metrics | `runtime_metrics` | Under-development: runtime metrics snapshots |
| SQLite | `sqlite` | SQLite-backed state DB for agent jobs and resumable state |
| Child Agents MD | `child_agents_md` | Pass AGENTS.md to spawned child agents |
| Image Detail Original | `image_detail_original` | Allow `detail: "original"` image outputs |
| Request Compression | `enable_request_compression` | Zstd compression for streaming requests |
| Collab | `collab` | Multi-agent collaboration tools |
| Spawn CSV | `spawn_csv` | CSV-backed agent job tools |
| Apps | `apps` | ChatGPT Apps/Connectors |
| Tool Suggest | `tool_suggest` | Discoverable tool suggestions for apps |
| Plugins | `plugins` | Plugin system |
| Image Generation | `image_generation` | Built-in image generation tool |
| Skill MCP Dependency Install | `skill_mcp_dependency_install` | Auto-install missing MCP deps |
| Skill Env Var Dependency Prompt | `skill_env_var_dependency_prompt` | Prompt for missing skill env vars |
| Steer | `steer` | Enter submits immediately instead of queuing |
| PowerShell UTF8 | `powershell_utf8` | Enforce UTF8 output in PowerShell |
| Windows Sandbox | `experimental_windows_sandbox` | Windows sandbox (restricted token) |
| Windows Sandbox Elevated | `elevated_windows_sandbox` | Elevated Windows sandbox pipeline |
| JS REPL | `js_repl` | Persistent Node-backed JavaScript REPL |
| Auto Approval Agent | `auto_approval_agent` | Security reviewer subagent for approvals |
| Prevent Sleep | `prevent_sleep` | Prevent sleep while agent is running |
| Suppress Unstable Warning | `suppress_unstable_features_warning` | Hide under-development warnings |

---

#### 12. 🎙️ **Voice / Realtime (TTS & STT)**
Realtime sessions now start with recent thread context and are less likely to self-interrupt during audio playback. Realtime voice sessions now let you pick microphone and speaker devices, persist those choices, and send audio in a format better aligned with transcription.

| Setting | Description |
|---|---|
| Microphone Device | Select input audio device |
| Speaker Device | Select output audio device |
| Audio Format | Transcription-aligned format |
| Voice Pipeline | Non-realtime STT → Workflow → TTS pipeline |
| TTS Voice | e.g. `nova`, `alloy`, etc. |
| Realtime Mode | WebSocket-based bidirectional voice |

---

#### 13. 🔔 **Notifications**
Codex can run a notification hook when the agent finishes a turn. When Codex knows which client started the turn, the legacy notify JSON payload also includes a top-level client field. The TUI reports codex-tui.

| Setting | Config Key | Description |
|---|---|---|
| Notify Script | `notify` | Script run on turn completion |
| TUI Notifications | `[tui]` | Inline desktop notifications |

---

#### 14. 🧠 **Memory & History**
Memories are now configurable, and there is a new `codex debug clear-memories` command to fully reset saved memory state. Token budget for storing individual tool/function outputs in history.

| Setting | Config Key | Description |
|---|---|---|
| Memories Path | `~/.codex/memories` | Writable memory storage |
| Tool Output Token Budget | `tool_output_token_budget` | Max tokens per tool output in history |
| Session Persistence | `state_dir` | SQLite-backed state DB directory |
| Clear Memories | action | `codex debug clear-memories` |

---

#### 15. 🛡️ **Shell Environment Policy**
Controls which env vars Codex forwards to spawned commands via `[shell_environment_policy]`.

| Setting | Config Key | Values |
|---|---|---|
| Policy Type | `policy` | `exclude`, `include_only` |
| Exclude List | `exclude` | `["AWS_SECRET_ACCESS_KEY", ...]` |
| Include Only | `include_only` | `["PATH", "HOME", ...]` |

---

#### 16. 🤝 **Multi-Agent / Agents**
Configure `[agents]` with `max_threads` (default 6), `max_depth` (default 1), `job_max_runtime_seconds` (default 1800), and role definitions like `[agents.reviewer]` with description, config_file, and nickname_candidates.

| Setting | Config Key | Description |
|---|---|---|
| Max Threads | `agents.max_threads` | Max concurrent agent threads (default: 6) |
| Max Depth | `agents.max_depth` | Max nested spawn depth (default: 1) |
| Job Max Runtime | `agents.job_max_runtime_seconds` | Per-worker timeout (default: 1800s) |
| Role Definitions | `[agents.<role>]` | description, config_file, nickname_candidates |

---

#### 17. ⚡ **Skills**
Skills extend Codex with reusable task-specific capabilities. They are scanned from `.agents/skills/` (project-level) and `~/.codex/skills/` or `~/.agents/skills/` (personal).

| Setting | Config Key | Description |
|---|---|---|
| Per-Skill Toggle | `[skills.<name>]` | Enable or disable the referenced skill |
| Skill Path | `[skills.<name>].path` | Path to a skill folder containing SKILL.md |
| Scan Directories | auto | project + user + global |

---

#### 18. 📜 **Execution Rules**
Define fine-grained rules for what commands Codex can run. Rules use Starlark format in `~/.codex/rules/`.

| Setting | Description |
|---|---|
| Prefix Rules | pattern + decision (`allow`, `prompt`, `forbidden`) |
| Justification | Human-readable reason for the rule |

---

#### 19. 🔑 **Authentication**
CLI auth credentials store supports: `file` (default, in Codex home directory), `keyring` (OS-specific keyring service), or `auto` (keyring if available, otherwise file).

| Setting | Config Key | Values |
|---|---|---|
| Credential Store | `cli_auth_credentials_store` | `file`, `keyring`, `auto` |
| Auth File | `CODEX_AUTH_FILE` | path |

---

#### 20. 📝 **Developer Instructions**
Set `developer_instructions` for inline instructions (like AGENTS.md but in config). Rename `experimental_instructions_file` to `model_instructions_file`.

| Setting | Config Key | Description |
|---|---|---|
| Inline Instructions | `developer_instructions` | Direct text in config |
| Instructions File | `model_instructions_file` | Path to external file |
| Project Instructions | `AGENTS.md` | Auto-discovered per project |

---

#### 21. 🖼️ **Image & Vision**
Enable the local-image attachment tool `view_image`. Image generation tool allows the model to invoke built-in image generation.

| Setting | Config Key | Description |
|---|---|---|
| View Image | `view_image` | Enable local-image attachment tool |
| Image Generation | `[features] image_generation` | Built-in image gen |
| Image Detail Original | `[features] image_detail_original` | High-res image output |

---

#### 22. 📂 **Project Trust & Roots**
Codex discovers project configuration by walking up from the working directory. By default, it treats a `.git` directory as the project root. Customize with `project_root_markers`.

| Setting | Config Key | Description |
|---|---|---|
| Root Markers | `project_root_markers` | `[".git", ".hg", ".sl"]` |
| Trust Mode | per-project | trusted / untrusted |

---

#### 23. 🧩 **Plugins & Apps**
Expanded plugin workflows with curated marketplace discovery, richer metadata, install-time auth checks, and a plugin/uninstall endpoint.

| Setting | Description |
|---|---|
| Plugin Enable/Disable | Manage installed plugins |
| Marketplace Discovery | Browse curated plugins |
| Connector Apps | `$ mentions` in composer |
| Suggestion Allowlist | Honor a configured suggestion allowlist for plugins |

---

#### 24. 🔄 **Session Resume & History**
Codex stores transcripts locally for resume. `codex resume` launches a picker, `codex resume --last` jumps to most recent, `codex resume --all` shows all sessions.

| Setting | Description |
|---|---|
| Resume Last | Skip picker, open last session |
| Resume All | Show sessions across all directories |
| Fork Session | Fork a previous session into a new thread preserving transcript |

---

#### 25. 📡 **Network & Proxy**
Set `openai_base_url` in config.toml to point the built-in OpenAI provider at an LLM proxy, router, or data-residency project.

| Setting | Config Key | Description |
|---|---|---|
| OpenAI Base URL | `openai_base_url` | Proxy/router endpoint |
| Sandbox Network Access | `sandbox_workspace_write.network_access` | `true`/`false` |
| Custom Model Providers | `[model_providers.<id>]` | name, base_url, env_key, wire_api |

---

#### 26. 🪝 **Hooks & Events**
Added a `userpromptsubmit` hook so prompts can be blocked or augmented before execution and before they enter history.

| Setting | Description |
|---|---|
| User Prompt Submit Hook | Block/augment prompts pre-execution |
| Notify Hook | Script on turn completion |
| Request Permissions Tool | Runtime permission requests during turns |

---

### 📋 Final Complete Menu Order

```
 ┌─────────────────────────────────────────────┐
 │           🚀  YOUR TUI MENUS               │
 ├─────────────────────────────────────────────┤
 │  1.  🔌  Providers (& Custom Model Provs)  │
 │  2.  ⌨️   Keyboard Shortcuts               │
 │  3.  🎨  Theme                              │
 │  4.  🌳  Worktree                           │
 │  5.  🛡️   Sandbox                           │
 │  6.  🤖  Model Configuration               │
 │  7.  ✅  Approval Policy                    │
 │  8.  🌐  Web Search                         │
 │  9.  🔌  MCP Servers                        │
 │ 10.  🎭  Profiles                           │
 │ 11.  🎯  Feature Flags (Experimental)       │
 │ 12.  🎙️   Voice / Realtime (TTS & STT)      │
 │ 13.  🔔  Notifications & Hooks              │
 │ 14.  🧠  Memory & History                   │
 │ 15.  🛡️   Shell Environment Policy           │
 │ 16.  🤝  Multi-Agent / Agents               │
 │ 17.  ⚡  Skills                             │
 │ 18.  📜  Execution Rules (Starlark)         │
 │ 19.  🔑  Authentication                     │
 │ 20.  📝  Developer Instructions             │
 │ 21.  🖼️   Image & Vision                     │
 │ 22.  📂  Project Trust & Root Markers       │
 │ 23.  🧩  Plugins & Apps                     │
 │ 24.  🔄  Session Resume & History           │
 │ 25.  📡  Network & Proxy                    │
 │ 26.  🪝  Hooks & Events                     │
 └─────────────────────────────────────────────┘
```

This covers **every config key, feature flag, and hidden capability** exposed by codex-rs — from the obvious (`model`, `sandbox_mode`) to the deeply buried (`prevent_sleep`, `auto_approval_agent`, `spawn_csv`, `image_detail_original`, `enable_request_compression`). Your TUI will be the first frontend to surface all of these in a discoverable way! 🔥
