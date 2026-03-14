Let me research the latest Codex CLI commands and OpenClaw features to build the ultimate combined CLI argument list.Now I have everything. Here's the COMPLETE unified CLI argument list for DX — combining Codex CLI's full command surface, OpenClaw's gateway/messaging/automation features, and DX's unique capabilities. 🔥

---

```
# ============================================================================
# DX CLI — COMPLETE COMMAND & ARGUMENT REFERENCE
# ============================================================================
# The unified CLI that combines:
#   - Codex CLI's production-grade coding agent (inherited via fork)
#   - OpenClaw's gateway, messaging, scheduling, and multi-agent runtime
#   - DX's unique token savings, universal auth, and beautiful TUI
#
# Usage: dx [GLOBAL FLAGS] [COMMAND] [COMMAND FLAGS] [PROMPT|ARGS]
# ============================================================================


# ╔═══════════════════════════════════════════════════════════════════════════╗
# ║                         GLOBAL FLAGS                                     ║
# ║  Apply to ALL commands. Place after the subcommand when using one.       ║
# ╚═══════════════════════════════════════════════════════════════════════════╝

--model, -m <MODEL>             # Select model (e.g., gpt-5.4, claude-4-sonnet, gemini-3)
--provider <PROVIDER>           # Select provider (openai, anthropic, google, groq, ollama, openrouter, etc.)
--profile, -p <NAME>            # Load a named config profile (e.g., dx --profile ci)
--config, -c <KEY=VALUE>        # Override config values (repeatable; parsed as TOML, then string fallback)
--cd, -C <DIR>                  # Set working directory before execution
--add-dir <DIR>                 # Grant additional directory write access (repeatable)
--sandbox, -s <MODE>            # Sandbox mode: read-only | workspace-write | danger-full-access
--approval, -a <POLICY>         # Approval policy: on-request | never | untrusted
--full-auto                     # Auto-approve + workspace-write sandbox (convenience shortcut)
--json                          # Emit newline-delimited JSON/JSONL instead of formatted output
--no-color                      # Disable colored output (for piping/CI)
--quiet, -q                     # Suppress non-essential output
--search                        # Enable live web search (default: cached)
--offline                       # Force offline mode — use only local models, no network
--oss                           # Use local OSS provider (Ollama/LM Studio/MLX auto-detected)
--image, -i <PATH>              # Attach image(s) to prompt (repeatable, comma-separated)
--reasoning <LEVEL>             # Reasoning effort: minimal | low | medium | high | xhigh
--version, -V                   # Print DX version and exit
--help, -h                      # Show help for any command or subcommand
--verbose                       # Enable verbose/debug output
--dev                           # Developer mode (extra diagnostics)


# ╔═══════════════════════════════════════════════════════════════════════════╗
# ║                    1. INTERACTIVE TUI (DEFAULT)                          ║
# ║  Inherited from Codex CLI — enhanced with DX's beautiful Ratatui TUI    ║
# ╚═══════════════════════════════════════════════════════════════════════════╝

dx                                          # Launch interactive TUI
dx "Fix the login bug"                      # TUI with initial prompt
dx -m claude-4-sonnet "Refactor auth"       # TUI with specific model
dx -i screenshot.png "Match this design"    # TUI with image attachment
dx --offline "Explain this function"        # TUI in offline mode

# TUI-SPECIFIC FLAGS
--search                    # Enable live web search for this session
--theme <THEME>             # Set TUI color theme (e.g., dx --theme catppuccin)


# ╔═══════════════════════════════════════════════════════════════════════════╗
# ║                    2. EXEC — Non-Interactive Mode                        ║
# ║  Inherited from Codex CLI — for scripting, CI/CD, automation             ║
# ╚═══════════════════════════════════════════════════════════════════════════╝

dx exec <PROMPT>                            # Run non-interactively (alias: dx e)
dx exec --full-auto "Fix tests"             # Auto-approve everything
dx exec -m gpt-5.4 -s workspace-write "Implement feature"
dx exec --json "Analyze code"               # JSONL streaming output

# EXEC SUBCOMMANDS
dx exec resume --last "Continue"            # Resume most recent session non-interactively
dx exec resume <SESSION_ID> "Continue"      # Resume specific session
dx exec resume --all --last "Continue"      # Resume across all directories
dx exec fork <SESSION_ID> "Try alt"         # Fork a session non-interactively (DX addition)
dx exec review --uncommitted                # Non-interactive code review

# EXEC FLAGS
--prompt <TEXT>             # Explicit prompt (alternative to positional)
--last                      # Auto-select most recent session
--all                       # Include sessions from all directories
--output <FILE>             # Write final output to file
--schema <FILE>             # Constrain output to JSON schema
--timeout <SECONDS>         # Maximum execution time
--enable <FEATURE>          # Enable feature flag for this run (e.g., web_search_request)


# ╔═══════════════════════════════════════════════════════════════════════════╗
# ║                    3. RESUME — Continue Previous Sessions                ║
# ║  Inherited from Codex CLI                                                ║
# ╚═══════════════════════════════════════════════════════════════════════════╝

dx resume                                   # Interactive session picker
dx resume <SESSION_ID>                      # Resume specific session
dx resume --last                            # Resume most recent (cwd-scoped)
dx resume --last --all                      # Resume most recent (any directory)

# RESUME FLAGS
--cd <DIR>                  # Override working directory for resumed session
--add-dir <DIR>             # Add extra writable roots


# ╔═══════════════════════════════════════════════════════════════════════════╗
# ║                    4. FORK — Branch a Session                            ║
# ║  Inherited from Codex CLI                                                ║
# ╚═══════════════════════════════════════════════════════════════════════════╝

dx fork                                     # Interactive fork picker
dx fork --last                              # Fork most recent session
dx fork --last "Try a different approach"   # Fork with new prompt
dx fork <SESSION_ID>                        # Fork specific session


# ╔═══════════════════════════════════════════════════════════════════════════╗
# ║                    5. REVIEW — AI Code Review                            ║
# ║  Inherited from Codex CLI                                                ║
# ╚═══════════════════════════════════════════════════════════════════════════╝

dx review                                   # Review with default settings
dx review --uncommitted                     # Review staged/unstaged/untracked changes
dx review --base main                       # Review against base branch
dx review --commit <SHA>                    # Review a specific commit
dx review --uncommitted "Focus on security" # Review with custom instructions


# ╔═══════════════════════════════════════════════════════════════════════════╗
# ║                    6. APPLY — Apply Diffs to Working Tree                ║
# ║  Inherited from Codex CLI (alias: dx a)                                  ║
# ╚═══════════════════════════════════════════════════════════════════════════╝

dx apply                                    # Apply latest local diff
dx apply <TASK_ID>                          # Apply diff from cloud task


# ╔═══════════════════════════════════════════════════════════════════════════╗
# ║                    7. CLOUD — Cloud Task Management                      ║
# ║  Inherited from Codex CLI (alias: dx cloud-tasks)                        ║
# ╚═══════════════════════════════════════════════════════════════════════════╝

dx cloud                                    # Interactive cloud task picker
dx cloud exec --env <ENV_ID> "Fix tests"    # Start a cloud task
dx cloud exec --env <ENV_ID> --attempts 3 "Summarize bugs"  # Best-of-N
dx cloud status <TASK_ID>                   # Check task progress
dx cloud diff <TASK_ID>                     # View task diff
dx cloud list                               # List recent tasks
dx cloud list --json                        # JSON output
dx cloud list --env <ENV_ID>                # Filter by environment
dx cloud list --limit <N>                   # Limit results
dx cloud list --cursor <CURSOR>             # Pagination
dx cloud apply <TASK_ID>                    # Apply task diff locally


# ╔═══════════════════════════════════════════════════════════════════════════╗
# ║                    8. APP — Desktop Application                          ║
# ║  Inherited from Codex CLI, enhanced with GPUI                            ║
# ╚═══════════════════════════════════════════════════════════════════════════╝

dx app                                      # Launch desktop app (auto-downloads)
dx app <PATH>                               # Open specific workspace in app
dx app-server                               # Launch app server (dev/debug)


# ╔═══════════════════════════════════════════════════════════════════════════╗
# ║                    9. LOGIN / LOGOUT / AUTH                              ║
# ║  Inherited from Codex + DX universal auth additions                      ║
# ╚═══════════════════════════════════════════════════════════════════════════╝

dx login                                    # Interactive auth (opens browser)
dx login --device                           # Device code flow (no browser)
dx login --with-api-key                     # Read API key from stdin
dx login --provider openai                  # Auth with specific provider
dx login --provider anthropic               # Auth with Anthropic
dx login --provider google                  # Auth with Google AI
dx login --provider github-copilot          # Auth with GitHub Copilot subscription
dx login --provider claude-max              # Auth with Claude Max subscription (DX)
dx login --provider chatgpt-plus            # Auth with ChatGPT Plus subscription (DX)
dx login --provider ollama                  # Configure local Ollama (DX)
dx login status                             # Show current auth mode and status
dx login list                               # List all authenticated providers (DX)
dx logout                                   # Remove stored credentials
dx logout --provider <PROVIDER>             # Remove specific provider credentials (DX)
dx logout --all                             # Remove all credentials (DX)


# ╔═══════════════════════════════════════════════════════════════════════════╗
# ║                   10. FEATURES — Feature Flag Management                 ║
# ║  Inherited from Codex CLI + DX feature additions                         ║
# ╚═══════════════════════════════════════════════════════════════════════════╝

dx features list                            # List all feature flags with status
dx features list --json                     # JSON output (DX fix: no more broken TSV)
dx features enable <FEATURE>                # Enable a feature persistently
dx features disable <FEATURE>               # Disable a feature persistently

# KEY FEATURES:
#   voice_transcription     — Hold spacebar to dictate prompts
#   multi_agent             — Enable multi-agent workflows
#   shell_tool              — Shell command execution
#   unified_exec            — Unified execution mode
#   shell_snapshot          — Shell state snapshots
#   undo                    — Enable undo/rollback
#   js_repl                 — JavaScript REPL (experimental)
#   memory_tool             — Persistent memory (experimental)
#   web_search_request      — Web search in exec mode
#   dx_serializer           — DX compact serialization (DX exclusive)
#   rlm                     — Recursive Language Model processing (DX exclusive)
#   token_tracker           — Real-time token savings dashboard (DX exclusive)
#   smart_routing            — Intelligent model routing per task (DX exclusive)


# ╔═══════════════════════════════════════════════════════════════════════════╗
# ║                   11. MCP — Model Context Protocol Management            ║
# ║  Inherited from Codex CLI                                                ║
# ╚═══════════════════════════════════════════════════════════════════════════╝

dx mcp list                                 # List configured MCP servers
dx mcp list --json                          # JSON output
dx mcp add <NAME> -- <COMMAND> [ARGS]       # Add stdio MCP server
dx mcp add <NAME> --url <URL>               # Add streamable HTTP MCP server
dx mcp add <NAME> -- cmd --env KEY=VALUE    # Add with environment vars
dx mcp show <NAME>                          # Show server config
dx mcp show <NAME> --json                   # JSON output
dx mcp remove <NAME>                        # Remove an MCP server
dx mcp login <NAME>                         # OAuth login for HTTP server
dx mcp logout <NAME>                        # Remove OAuth credentials
dx mcp-server                               # Run DX itself as an MCP server


# ╔═══════════════════════════════════════════════════════════════════════════╗
# ║                   12. SKILLS — Skill Management                          ║
# ║  Inherited from Codex + DX marketplace additions                         ║
# ╚═══════════════════════════════════════════════════════════════════════════╝

dx skills list                              # List installed skills
dx skills list --json                       # JSON output
dx skills info <SKILL>                      # Show skill details
dx skills check                             # Verify skill integrity
dx skills install <SKILL>                   # Install from marketplace (DX)
dx skills install <PATH>                    # Install from local path (DX)
dx skills uninstall <SKILL>                 # Remove a skill (DX)
dx skills search <QUERY>                    # Search marketplace (DX)
dx skills publish <PATH>                    # Publish to marketplace (DX)
dx skills browse                            # Open visual skill browser in TUI (DX)


# ╔═══════════════════════════════════════════════════════════════════════════╗
# ║                   13. MODELS — Model Management                          ║
# ║  DX universal provider layer + OpenClaw-inspired model management        ║
# ╚═══════════════════════════════════════════════════════════════════════════╝

dx models list                              # List available models across all providers
dx models list --provider <PROVIDER>        # List models for specific provider
dx models list --local                      # List local/offline models only
dx models list --json                       # JSON output
dx models status                            # Check model availability + auth status
dx models status --probe                    # Live probe each provider
dx models set <MODEL>                       # Set default model
dx models set-image <MODEL>                 # Set default vision model
dx models scan                              # Auto-discover available models (DX)
dx models pull <MODEL>                      # Pull/download local model via Ollama (DX)
dx models benchmark <MODEL>                 # Benchmark a model (speed, cost, quality) (DX)

# MODEL ALIASES (from OpenClaw)
dx models aliases list                      # List model aliases
dx models aliases add <ALIAS> <MODEL>       # Add alias (e.g., "fast" -> gpt-5.4-fast)
dx models aliases remove <ALIAS>            # Remove alias

# MODEL FALLBACKS (from OpenClaw)
dx models fallbacks list                    # List fallback chain
dx models fallbacks add <MODEL>             # Add to fallback chain
dx models fallbacks remove <MODEL>          # Remove from fallback chain
dx models fallbacks clear                   # Clear all fallbacks

# MODEL AUTH (from OpenClaw)
dx models auth add --provider <P> --key <K> # Add provider credentials
dx models auth setup-token <PROVIDER>       # Interactive token setup
dx models auth paste-token <PROVIDER>       # Paste token from clipboard
dx models auth order get                    # Get provider priority order
dx models auth order set <P1,P2,P3>         # Set provider priority


# ╔═══════════════════════════════════════════════════════════════════════════╗
# ║                   14. GATEWAY — AI Agent Gateway                         ║
# ║  Inspired by OpenClaw — DX's always-on agent runtime in Rust             ║
# ╚═══════════════════════════════════════════════════════════════════════════╝

dx gateway                                  # Show gateway status
dx gateway start                            # Start the DX gateway daemon
dx gateway stop                             # Stop the gateway
dx gateway restart                          # Restart (apply config changes)
dx gateway install                          # Install as system service
dx gateway uninstall                        # Remove system service
dx gateway status                           # Probe gateway health (RPC)
dx gateway status --deep                    # Deep health probe
dx gateway run                              # Run gateway in foreground (debug)
dx gateway logs                             # Show gateway logs
dx gateway logs --follow                    # Tail logs
dx gateway logs --json                      # JSON log output
dx gateway logs --local-time                # Timestamps in local timezone


# ╔═══════════════════════════════════════════════════════════════════════════╗
# ║                   15. CHANNELS — Messaging Platform Integration          ║
# ║  Inspired by OpenClaw — reach DX from any messaging app                  ║
# ╚═══════════════════════════════════════════════════════════════════════════╝

dx channels list                            # List configured channels
dx channels status                          # Check channel health
dx channels status --probe                  # Deep health check
dx channels logs                            # Show channel logs
dx channels add                             # Interactive channel setup wizard
dx channels add --type <TYPE>               # Non-interactive add
dx channels remove <CHANNEL>                # Remove a channel
dx channels remove <CHANNEL> --delete       # Remove + delete config
dx channels login                           # Auth/pair a channel

# SUPPORTED CHANNEL TYPES:
#   whatsapp, telegram, discord, slack, google-chat, signal,
#   imessage, irc, microsoft-teams, matrix, webchat,
#   line, mattermost, nostr, twitch


# ╔═══════════════════════════════════════════════════════════════════════════╗
# ║                   16. MESSAGE — Outbound Messaging                       ║
# ║  Inspired by OpenClaw — send messages from CLI                           ║
# ╚═══════════════════════════════════════════════════════════════════════════╝

dx message send --channel <CH> --target <T> --message <M>
dx message edit --channel <CH> --target <T> --message-id <ID> --message <M>
dx message delete --channel <CH> --message-id <ID>
dx message search --channel <CH> --query <Q>
dx message react --channel <CH> --message-id <ID> --emoji <E>
dx message pin --channel <CH> --message-id <ID>
dx message unpin --channel <CH> --message-id <ID>
dx message poll --channel <CH> --target <T> --poll-question <Q> --poll-option <O>


# ╔═══════════════════════════════════════════════════════════════════════════╗
# ║                   17. AGENTS — Multi-Agent Management                    ║
# ║  Codex multi-agent + OpenClaw isolated agent architecture                ║
# ╚═══════════════════════════════════════════════════════════════════════════╝

dx agents list                              # List configured agents
dx agents list --json                       # JSON output
dx agents add                               # Interactive agent wizard
dx agents add --name <NAME> --workspace <DIR> --model <MODEL>  # Non-interactive
dx agents delete <NAME>                     # Delete an agent
dx agents modify <NAME> [--model M] [--sandbox-mode M] [--sandbox-scope S]
dx agents status                            # Show agent health + activity
dx agents bind <AGENT> --channel <CH>       # Bind agent to channel (DX)
dx agents unbind <AGENT> --channel <CH>     # Unbind agent from channel (DX)
dx agents route list                        # Show routing bindings (DX)
dx agent --deliver <PROMPT>                 # Run one agent turn via gateway
dx agent --deliver --session-id <ID> <PROMPT>  # Deliver to specific session


# ╔═══════════════════════════════════════════════════════════════════════════╗
# ║                   18. SESSIONS — Session Management                      ║
# ║  Codex sessions + OpenClaw session architecture                          ║
# ╚═══════════════════════════════════════════════════════════════════════════╝

dx sessions list                            # List all stored sessions
dx sessions list --json                     # JSON output
dx sessions list --agent <NAME>             # Filter by agent
dx sessions show <SESSION_ID>               # Show session details
dx sessions delete <SESSION_ID>             # Delete a session
dx sessions export <SESSION_ID>             # Export session transcript (DX)
dx sessions share <SESSION_ID>              # Generate shareable link (DX)


# ╔═══════════════════════════════════════════════════════════════════════════╗
# ║                   19. CRON — Scheduled Task Automation                   ║
# ║  Inspired by OpenClaw's cron system                                      ║
# ╚═══════════════════════════════════════════════════════════════════════════╝

dx cron list                                # List cron jobs
dx cron list --all                          # Include disabled jobs
dx cron status                              # Show cron system status
dx cron add --name <NAME> --cron <EXPR> --message <PROMPT>     # Cron expression
dx cron add --name <NAME> --every <MS> --message <PROMPT>      # Interval (ms)
dx cron add --name <NAME> --at <ISO_TIME> --message <PROMPT>   # One-time
dx cron edit --id <NAME>                    # Edit existing job
dx cron rm --id <NAME>                      # Remove a job
dx cron enable --id <NAME>                  # Enable a job
dx cron disable --id <NAME>                 # Disable a job
dx cron run --id <NAME>                     # Run immediately (ignore schedule)
dx cron runs --id <NAME>                    # View run history
dx cron runs --id <NAME> --limit <N>        # Limit history results


# ╔═══════════════════════════════════════════════════════════════════════════╗
# ║                   20. BROWSER — Browser Automation                       ║
# ║  Inspired by OpenClaw's CDP browser control                              ║
# ╚═══════════════════════════════════════════════════════════════════════════╝

dx browser status                           # Check browser status
dx browser start                            # Start managed browser instance
dx browser stop                             # Stop browser
dx browser reset-profile                    # Reset browser profile
dx browser tabs                             # List open tabs
dx browser tabs open <URL>                  # Open URL in new tab
dx browser tabs focus <TAB_ID>              # Focus a tab
dx browser tabs close <TAB_ID>              # Close a tab
dx browser screenshot                       # Capture current page
dx browser screenshot --url <URL>           # Capture specific URL
dx browser snapshot                         # Full page snapshot (DOM + screenshot)
dx browser navigate <URL>                   # Navigate current tab
dx browser profiles create-profile <NAME>   # Create browser profile
dx browser profiles delete-profile <NAME>   # Delete browser profile


# ╔═══════════════════════════════════════════════════════════════════════════╗
# ║                   21. NODES — Device Network Management                  ║
# ║  Inspired by OpenClaw's node system                                      ║
# ╚═══════════════════════════════════════════════════════════════════════════╝

dx nodes list                               # List paired devices
dx nodes status                             # Show node health
dx nodes add                                # Pair a new device
dx nodes remove <NODE_ID>                   # Unpair a device

# DEVICE-SPECIFIC NODE
dx node run                                 # Run DX as a node (iOS/Android/remote)
dx node status                              # Node status
dx node install                             # Install node service
dx node uninstall                           # Uninstall node service
dx node start                               # Start node service
dx node stop                                # Stop node service
dx node restart                             # Restart node service

dx devices list                             # List paired devices (alias)


# ╔═══════════════════════════════════════════════════════════════════════════╗
# ║                   22. MEMORY — Persistent Memory System                  ║
# ║  Codex memories + OpenClaw memory architecture                           ║
# ╚═══════════════════════════════════════════════════════════════════════════╝

dx memory status                            # Show memory system status
dx memory index                             # Rebuild memory index
dx memory search <QUERY>                    # Search memories
dx memory clear                             # Clear all memories
dx memory export                            # Export memory to file (DX)
dx memory import <FILE>                     # Import memory from file (DX)


# ╔═══════════════════════════════════════════════════════════════════════════╗
# ║                   23. PLUGINS — Plugin Management                        ║
# ║  Codex plugins + OpenClaw plugin architecture                            ║
# ╚═══════════════════════════════════════════════════════════════════════════╝

dx plugins list                             # List installed plugins
dx plugins info <PLUGIN>                    # Show plugin details
dx plugins install <PATH|URL|NPM_SPEC>     # Install a plugin
dx plugins uninstall <PLUGIN>               # Uninstall a plugin
dx plugins enable <PLUGIN>                  # Enable a plugin
dx plugins disable <PLUGIN>                 # Disable a plugin
dx plugins doctor                           # Check plugin health


# ╔═══════════════════════════════════════════════════════════════════════════╗
# ║                   24. CONFIG — Configuration Management                  ║
# ║  Codex config.toml + OpenClaw config system, unified                     ║
# ╚═══════════════════════════════════════════════════════════════════════════╝

dx config                                   # Launch config wizard (DX visual TUI)
dx config get <PATH>                        # Get a config value (dot-path)
dx config set <PATH> <VALUE>                # Set a config value
dx config unset <PATH>                      # Remove a config entry
dx config file                              # Show config file path
dx config validate                          # Validate config file
dx config reset                             # Reset to defaults
dx config export                            # Export full config (DX)
dx config import <FILE>                     # Import config (DX)


# ╔═══════════════════════════════════════════════════════════════════════════╗
# ║                   25. SECURITY — Security & Policy                       ║
# ║  Codex execpolicy + OpenClaw security model                              ║
# ╚═══════════════════════════════════════════════════════════════════════════╝

dx security audit                           # Audit config + state for security issues
dx security audit --deep                    # Live gateway security probe
dx security audit --fix                     # Auto-fix security issues
dx execpolicy check --rules <FILE> <CMD>    # Check command against policy rules
dx execpolicy check --pretty --rules <F> <CMD>  # Pretty-printed output

dx approvals get                            # Get current approval settings
dx approvals set <POLICY>                   # Set approval policy
dx approvals allowlist add <PATTERN>        # Add to command allowlist
dx approvals allowlist remove <PATTERN>     # Remove from allowlist


# ╔═══════════════════════════════════════════════════════════════════════════╗
# ║                   26. SANDBOX — Sandbox Management                       ║
# ║  Codex sandbox + OpenClaw Docker sandbox                                 ║
# ╚═══════════════════════════════════════════════════════════════════════════╝

dx sandbox list                             # List sandbox environments
dx sandbox recreate                         # Recreate sandbox
dx sandbox explain                          # Show current sandbox policy
dx sandbox run <COMMAND>                    # Run command in sandbox


# ╔═══════════════════════════════════════════════════════════════════════════╗
# ║                   27. SETUP & ONBOARDING                                 ║
# ║  OpenClaw-inspired wizard + DX zero-config philosophy                    ║
# ╚═══════════════════════════════════════════════════════════════════════════╝

dx setup                                    # Full interactive setup wizard
dx setup --wizard                           # Explicit wizard mode
dx onboard                                  # Quick onboarding (gateway + workspace + skills)
dx onboard --install-daemon                 # Install gateway as daemon during onboard
dx onboard --non-interactive                # Headless onboard for CI
dx configure                                # Interactive configure wizard
dx doctor                                   # Health checks + quick fixes
dx doctor --fix                             # Auto-fix common issues
dx doctor --deep                            # Deep system scan
dx doctor --yes                             # Accept all defaults
dx doctor --non-interactive                 # Headless mode


# ╔═══════════════════════════════════════════════════════════════════════════╗
# ║                   28. SYSTEM & MAINTENANCE                               ║
# ║  Combined from Codex + OpenClaw                                          ║
# ╚═══════════════════════════════════════════════════════════════════════════╝

dx update                                   # Update DX to latest version
dx reset                                    # Reset local config/state
dx reset --scope <SCOPE>                    # config | config+creds+sessions | full
dx uninstall                                # Uninstall gateway + data (CLI remains)
dx completion bash                          # Generate bash completions
dx completion zsh                           # Generate zsh completions
dx completion fish                          # Generate fish completions
dx completion powershell                    # Generate PowerShell completions
dx status                                   # Overall DX system status
dx status --deep                            # Deep probe all services
dx status --usage                           # Show provider usage breakdown
dx health                                   # Gateway + channel health check
dx logs                                     # Show DX logs
dx logs --follow                            # Tail logs
dx logs --json                              # JSON logs
dx logs --local-time                        # Local timezone
dx dashboard                                # Open web control UI in browser


# ╔═══════════════════════════════════════════════════════════════════════════╗
# ║                   29. DEBUG                                              ║
# ║  Inherited from Codex CLI                                                ║
# ╚═══════════════════════════════════════════════════════════════════════════╝

dx debug app-server                         # Debug app server
dx debug app-server send-message-v2         # Test app-server client
dx debug clear-memories                     # Reset all saved memory state
dx debug sessions                           # Debug session storage
dx debug providers                          # Debug provider connections (DX)
dx debug tokens                             # Debug token counting/savings (DX)


# ╔═══════════════════════════════════════════════════════════════════════════╗
# ║                   30. GENERATE — Universal Generation                    ║
# ║  DX EXCLUSIVE — media generation beyond code                             ║
# ╚═══════════════════════════════════════════════════════════════════════════╝

dx generate code <PROMPT>                   # Generate code (default behavior)
dx generate chart <PROMPT>                  # Generate chart/visualization
dx generate image <PROMPT>                  # Generate image via AI
dx generate video <PROMPT>                  # Generate video via AI
dx generate audio <PROMPT>                  # Generate audio/music
dx generate 3d <PROMPT>                     # Generate 3D asset/scene
dx generate research <PROMPT>               # Deep research synthesis

# GENERATE FLAGS
--output, -o <PATH>         # Output file/directory
--format <FORMAT>           # Output format (png, svg, mp4, mp3, gltf, etc.)
--model <MODEL>             # Override model for this generation
--provider <PROVIDER>       # Override provider for this generation
--quality <QUALITY>         # Quality level (draft | standard | high | ultra)


# ╔═══════════════════════════════════════════════════════════════════════════╗
# ║                   31. TOKENS — Token Savings Dashboard                   ║
# ║  DX EXCLUSIVE — real-time token economics                                ║
# ╚═══════════════════════════════════════════════════════════════════════════╝

dx tokens status                            # Show token savings summary
dx tokens status --json                     # JSON output
dx tokens history                           # Token usage history
dx tokens history --period <PERIOD>         # Filter: today | week | month | all
dx tokens compare                           # Compare DX savings vs raw API costs
dx tokens compare --json                    # JSON output
dx tokens budget set <AMOUNT>               # Set daily/monthly token budget
dx tokens budget status                     # Show budget utilization


# ╔═══════════════════════════════════════════════════════════════════════════╗
# ║                   32. VOICE — Voice Interaction                          ║
# ║  Codex voice + OpenClaw push-to-talk + DX enhancements                   ║
# ╚═══════════════════════════════════════════════════════════════════════════╝

dx voice start                              # Start voice session
dx voice stop                               # Stop voice session
dx voice devices                            # List audio devices
dx voice devices set-mic <DEVICE>           # Set microphone
dx voice devices set-speaker <DEVICE>       # Set speaker
dx voice transcribe <FILE>                  # Transcribe audio file


# ╔═══════════════════════════════════════════════════════════════════════════╗
# ║                   33. WORKSPACE — Workspace Files Management             ║
# ║  Inspired by OpenClaw workspace architecture                             ║
# ╚═══════════════════════════════════════════════════════════════════════════╝

dx workspace init                           # Initialize DX workspace in current dir
dx workspace wizard                         # Interactive workspace file generator
dx workspace status                         # Show workspace config status
dx workspace validate                       # Validate workspace files

# WORKSPACE FILES (auto-managed in .dx/ or ~/.dx/workspace/):
#   SOUL.md       — Agent identity and purpose
#   AGENTS.md     — Project instructions (Codex-compatible)
#   USER.md       — User preferences and context
#   TOOLS.md      — Tool configuration and permissions
#   HEARTBEAT.md  — Scheduled check-in behavior
#   IDENTITY.md   — Agent personality/voice


# ╔═══════════════════════════════════════════════════════════════════════════╗
# ║                   34. ACP — Agent Client Protocol Bridge                 ║
# ║  Inspired by OpenClaw ACP for IDE integration                            ║
# ╚═══════════════════════════════════════════════════════════════════════════╝

dx acp                                      # Start ACP bridge (stdio)
dx acp --session-label <LABEL>              # ACP with named session
dx acp --remote-url <URL>                   # Connect to remote gateway
dx acp --remote-token <TOKEN>               # Auth for remote gateway
dx acp status                               # ACP bridge status
dx acp health                               # ACP health check


# ╔═══════════════════════════════════════════════════════════════════════════╗
# ║                   35. HEARTBEAT — Scheduled Check-ins                    ║
# ║  Inspired by OpenClaw heartbeat system                                   ║
# ╚═══════════════════════════════════════════════════════════════════════════╝

dx heartbeat last                           # Show last heartbeat
dx heartbeat enable                         # Enable heartbeat
dx heartbeat disable                        # Disable heartbeat
dx heartbeat status                         # Heartbeat status


# ╔═══════════════════════════════════════════════════════════════════════════╗
# ║                   36. PRESENCE — Online/Offline Status                   ║
# ║  Inspired by OpenClaw presence system                                    ║
# ╚═══════════════════════════════════════════════════════════════════════════╝

dx presence status                          # Show presence across channels
dx presence set <STATUS>                    # Set status (online/away/busy/offline)


# ╔═══════════════════════════════════════════════════════════════════════════╗
# ║                        SLASH COMMANDS (IN-TUI)                           ║
# ║  Available during interactive sessions — type / to open popup            ║
# ╚═══════════════════════════════════════════════════════════════════════════╝

# INHERITED FROM CODEX CLI:
/model              # Switch model mid-session
/model <MODEL>      # Switch to specific model
/plan               # Enter plan mode
/plan <PROMPT>      # Plan mode with prompt
/review             # Launch code review
/fork               # Fork current session
/fork <PROMPT>      # Fork with new direction
/resume             # Resume previous session picker
/clear              # Clear terminal, fresh chat
/compact            # Summarize history to reclaim tokens
/copy               # Copy latest output to clipboard
/status             # Session status (model, tokens, git branch)
/statusline         # Configure status line items
/config             # Inspect effective config
/permissions        # View/edit permissions
/personality        # Set agent personality
/experimental       # Toggle experimental features
/agent              # Agent management
/mcp                # List connected MCP tools
/apps               # Browse available connectors
/theme              # Change TUI theme
/quit               # Exit DX
/exit               # Exit DX (alias)
/feedback           # Send logs to DX maintainers
/sandbox-add-read-dir <PATH>   # (Windows) Add read access to directory

# DX EXCLUSIVE SLASH COMMANDS:
/provider           # Switch provider mid-session
/provider <P>       # Switch to specific provider
/tokens             # Show token savings for current session
/undo               # Undo last agent action (visual button equivalent)
/skills             # Open skill browser
/skills install <S> # Quick-install a skill
/generate <TYPE>    # Quick-generate (chart, image, video, audio, 3d)
/voice              # Toggle voice input
/offline            # Toggle offline mode
/route              # Show/change model routing strategy
/budget             # Show token budget status
/share              # Share current session
/export             # Export session transcript


# ╔═══════════════════════════════════════════════════════════════════════════╗
# ║                      KEYBOARD SHORTCUTS (IN-TUI)                         ║
# ║  All surfaced with visible hints in DX's beautiful TUI                   ║
# ╚═══════════════════════════════════════════════════════════════════════════╝

# INHERITED FROM CODEX:
Hold Spacebar       # Voice transcription (record + transcribe)
Ctrl+G              # Open prompt in external editor ($VISUAL)
Ctrl+L              # Clear screen (keep conversation)
Ctrl+C              # Cancel current operation
Esc (2x)            # Edit previous user message
Esc (repeated)      # Walk back through transcript
Enter               # Confirm / fork from walked-back point
@                   # Fuzzy file search in composer
Tab                 # Autocomplete

# DX ADDITIONS:
Ctrl+P              # Open command palette (search all features)
Ctrl+T              # Toggle token savings panel
Ctrl+U              # Undo last action
Ctrl+S              # Quick save / export session
Ctrl+M              # Toggle model selector
Ctrl+O              # Toggle offline mode
Ctrl+/              # Show all keyboard shortcuts


# ╔═══════════════════════════════════════════════════════════════════════════╗
# ║                    CONFIGURATION FILE REFERENCE                          ║
# ║  ~/.dx/config.toml — all CLI flags have config equivalents               ║
# ╚═══════════════════════════════════════════════════════════════════════════╝

# CORE (inherited from Codex)
# model = "gpt-5.4"
# model_reasoning_effort = "high"
# approval_policy = "on-request"
# sandbox_mode = "workspace-write"
# web_search = "cached"
#
# [features]
# multi_agent = true
# shell_tool = true
# voice_transcription = true
# undo = true
#
# [profiles.ci]
# approval_policy = "never"
# sandbox_mode = "workspace-write"
#
# [mcp_servers.my-db]
# command = "/usr/local/bin/my-mcp-server"
# env = { DB_URL = "postgres://..." }
# timeout_secs = 30

# DX ADDITIONS
# [dx]
# default_provider = "auto"          # auto | openai | anthropic | google | ollama | ...
# token_savings = true               # Enable DX Serializer + RLM
# smart_routing = true               # Auto-route tasks to best model
# offline_fallback = true            # Fall back to local model when offline
# theme = "catppuccin"               # TUI theme
#
# [dx.providers.openai]
# auth_mode = "subscription"         # subscription | api-key
# subscription_type = "plus"         # plus | pro | team | enterprise
#
# [dx.providers.anthropic]
# auth_mode = "subscription"
# subscription_type = "max"
#
# [dx.providers.github-copilot]
# auth_mode = "subscription"
# subscription_type = "pro-plus"
#
# [dx.providers.ollama]
# base_url = "http://localhost:11434"
# default_model = "llama3.3"
#
# [dx.gateway]
# mode = "local"
# bind = "loopback"
# port = 18789
# auth = { mode = "token", token = "your-secret-token" }
#
# [dx.tokens]
# budget_daily = 1000000             # Daily token budget
# budget_monthly = 25000000          # Monthly token budget
# alert_threshold = 0.8              # Alert at 80% usage
#
# [dx.channels.whatsapp]
# enabled = true
# dm_policy = "pairing"
#
# [dx.channels.telegram]
# enabled = true
# bot_token = "..."
#
# [dx.channels.discord]
# enabled = true
# bot_token = "..."
# guild_id = "..."


# ╔═══════════════════════════════════════════════════════════════════════════╗
# ║                    ENVIRONMENT VARIABLES                                 ║
# ╚═══════════════════════════════════════════════════════════════════════════╝

# INHERITED FROM CODEX:
# OPENAI_API_KEY          — OpenAI API key
# VISUAL                  — External editor for Ctrl+G

# DX ADDITIONS:
# DX_CONFIG_PATH          — Custom config file path
# DX_PROVIDER             — Default provider override
# DX_MODEL                — Default model override
# DX_OFFLINE              — Force offline mode (1/true)
# DX_THEME                — TUI theme override
# ANTHROPIC_API_KEY       — Anthropic API key
# GOOGLE_API_KEY          — Google AI API key
# GROQ_API_KEY            — Groq API key
# OPENROUTER_API_KEY      — OpenRouter API key
# OLLAMA_BASE_URL         — Ollama server URL
# GITHUB_COPILOT_TOKEN    — GitHub Copilot auth token
# DX_GATEWAY_TOKEN        — Gateway auth token
# DX_GATEWAY_PORT         — Gateway port override
# DX_TOKEN_BUDGET         — Token budget override
# DX_LOG_LEVEL            — Log level (trace/debug/info/warn/error)


# ╔═══════════════════════════════════════════════════════════════════════════╗
# ║                    QUICK START EXAMPLES                                  ║
# ╚═══════════════════════════════════════════════════════════════════════════╝

# ZERO CONFIG — Just type dx
dx

# FIRST TIME SETUP — Full wizard
dx setup --wizard

# USE WITH EXISTING SUBSCRIPTION
dx login --provider chatgpt-plus
dx login --provider claude-max
dx login --provider github-copilot

# OFFLINE CODING — No internet needed
dx --offline "Explain this function"

# FULL AUTO CI/CD
dx exec --full-auto -m gpt-5.4 "Fix failing tests and make npm test pass"

# CODE REVIEW BEFORE COMMIT
dx review --uncommitted "Focus on security and performance"

# MULTI-MODEL WORKFLOW
dx -m claude-4-sonnet "Write the architecture"
# then in session: /model gpt-5.4 for implementation

# SCHEDULED AUTOMATION
dx cron add --name "morning-report" --cron "0 9 * * *" \
  --message "Review open PRs and summarize blockers"

# SEND RESULTS TO WHATSAPP
dx message send --channel whatsapp --target +1234567890 \
  --message "Deploy complete ✅"

# GENERATE A CHART
dx generate chart "Monthly revenue Q1 2026 from sales.csv"

# CHECK TOKEN SAVINGS
dx tokens compare
```

---

That's **350+ commands, flags, and arguments** — the most comprehensive AI agent CLI ever designed. It unifies:

- 🔧 **Codex CLI's** entire command surface (exec, resume, fork, review, apply, cloud, mcp, features, sandbox, execpolicy, debug, app, login/logout, completion)
- 🦞 **OpenClaw's** gateway/messaging/automation layer (gateway, channels, message, cron, browser, nodes, agents, memory, plugins, heartbeat, presence, ACP, security audit, workspace files)
- 🚀 **DX exclusives** (generate, tokens, voice, models with routing/fallbacks/aliases, skills marketplace, universal auth, smart routing, offline mode, command palette, token budgets, session sharing)

**One CLI. Every feature. Zero config. The throne is yours.** 👑
