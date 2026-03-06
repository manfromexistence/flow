Let me research all the current features of Codex CLI thoroughly so we know exactly what you're inheriting for your **DX** fork!Alright bro, I've done a DEEP dive into every feature Codex CLI has. Here's the **complete feature inventory** of what you're inheriting — and exactly what you need to add to make **DX** the ultimate CLI. Let's go! 🔥

---

# 🧬 Complete Codex CLI Feature Inventory for Your **DX** Fork

## Everything You're Inheriting (FOR FREE) + What You Need to Add

---

## 🖼️ 1. IMAGE INPUT (Multimodal Vision) — ✅ ALREADY BUILT

Codex lets you attach screenshots or design specs so it can read image details alongside your prompt. You can paste images into the interactive composer or provide files on the command line. Codex accepts common formats such as PNG and JPEG. Use comma-separated filenames for two or more images, and combine them with text instructions to add context.

Codex supports image inputs — pass screenshots, wireframes, or diagrams alongside text prompts, e.g., `codex "Build a deployment status dashboard that looks like this screenshot" -i ./dashboard-mockup.png`

**CLI flags already built:**
- `--image` attaches images to the first message. It's repeatable and supports comma-separated lists.
- You can also attach one or more images to follow-up prompts. Separate multiple paths with commas or repeat the flag.

**🔧 What DX needs:** Your 100+ provider trait needs a `supports_vision(&self) -> bool` method so providers that don't support vision gracefully fall back to text descriptions.

---

## 🤖 2. MULTI-AGENT SYSTEM — ✅ ALREADY BUILT

This is **MASSIVE**. You get a full multi-agent orchestration engine for free:

Codex can run multi-agent workflows by spawning specialized agents in parallel and then collecting their results in one response. This can be particularly helpful for complex tasks that are highly parallel, such as codebase exploration or implementing a multi-step feature plan. With multi-agent workflows you can also define your own set of agents with different model configurations and instructions depending on the agent.

You can now fork a thread into sub-agents, making it easier to branch work without leaving the current conversation.

Multi-agent workflows are easier to run and track: `spawn_agents_on_csv` can fan out work from a CSV with built-in progress/ETA, and sub-agents are easier to follow with nicknames, a cleaner picker, and visible child-thread approval prompts.

Codex will automatically decide when to spawn a new agent or you can explicitly ask it to do so. For long-running commands or polling workflows, Codex can also use the built-in monitor role, which is tuned for waiting and repeated status checks.

**🔧 What DX needs:** With 100+ providers, each sub-agent could use a **DIFFERENT model**. Imagine: Claude for architecture review, GPT for implementation, DeepSeek for cost-efficient bulk tasks — all in one session. NO other CLI can do this.

---

## 🎤 3. VOICE INPUT — ✅ ALREADY BUILT

You can now dictate prompts by holding the spacebar to record and transcribe voice input directly in the TUI. This feature is still under development; to enable it set `features.voice_transcription = true` in your config.

Realtime voice sessions now let you pick microphone and speaker devices, persist those choices, and send audio in a format better aligned with transcription.

**🔧 What DX needs:** Route voice transcription through Whisper (OpenAI), Gemini, or local Whisper.cpp — provider-agnostic voice!

---

## 🌐 4. WEB SEARCH — ✅ ALREADY BUILT

Codex ships with a first-party web search tool. For local tasks in the Codex CLI, Codex enables web search by default and serves results from a web search cache. The cache is an OpenAI-maintained index of web results, so cached mode returns pre-indexed results instead of fetching live pages. This reduces exposure to prompt injection from arbitrary live content, but you should still treat web results as untrusted.

To fetch the most recent data, pass `--search` for a single run or set `web_search = "live"` in config.

**🔧 What DX needs:** Replace OpenAI's proprietary cache with a pluggable search backend — SearXNG, Brave Search API, Tavily, or Perplexity. Open search, not locked to OpenAI.

---

## 🛡️ 5. OS-LEVEL SANDBOXING — ✅ ALREADY BUILT

Locally, Codex uses an OS-enforced sandbox that limits what it can touch (typically to the current workspace), plus an approval policy that controls when it must stop and ask you before acting. Codex security controls come from two layers: **Sandbox mode** (what Codex can do technically) and **Approval policy** (when Codex must ask you before executing).

Codex uses Seatbelt/Landlock, not containers. Restrictions operate at the kernel level, below the application layer.

**Sandbox modes:**
- Local execution with three safety levels (Read Only/Auto/Full Access) — cross-platform sandbox security (macOS Seatbelt, Linux Landlock).

**🔧 What DX needs:** NOTHING. This is inherited directly. No TypeScript tool has kernel-level sandboxing.

---

## 📝 6. AGENTS.md (Cross-Tool Project Instructions) — ✅ ALREADY BUILT

AGENTS.md is cross-tool: Your AGENTS.md works in Cursor, Copilot, Amp, Jules, Gemini CLI, and 60,000+ open source projects. CLAUDE.md is Claude-only.

Codex discovers AGENTS.md using a cascading hierarchy: Home directory (`~/.codex/AGENTS.md`), Project root → current directory (walks down, reads one file per directory), Override files (`AGENTS.override.md` takes priority). Files are concatenated from root down. The closest file to the edited code wins for conflicting instructions.

**🔧 What DX needs:** Rename support to `DX.md` as well, while keeping backward compatibility with `AGENTS.md`.

---

## 🔗 7. MCP (Model Context Protocol) — ✅ ALREADY BUILT

Connect Codex to more tools by configuring Model Context Protocol servers. Add STDIO or streaming HTTP servers in `~/.codex/config.toml`, or manage them with the `codex mcp` CLI commands — Codex launches them automatically when a session starts and exposes their tools next to the built-ins. You can even run Codex itself as an MCP server when you need it inside another agent.

The Codex app, CLI, and IDE Extension share Model Context Protocol (MCP) settings. If you've already configured MCP servers in one, they're automatically adopted by the others.

**🔧 What DX needs:** Inherited for free. Your DX becomes the **universal MCP client** that works with any model.

---

## 💾 8. SESSION PERSISTENCE & RESUME — ✅ ALREADY BUILT

Codex stores your transcripts locally so you can pick up where you left off instead of repeating context. Use the resume subcommand when you want to reopen an earlier thread with the same repository state and instructions.

Each resumed run keeps the original transcript, plan history, and approvals, so Codex can use prior context while you supply new instructions.

**Commands:**
- 
`codex resume` launches a picker of recent interactive sessions. `codex resume --all` shows sessions beyond the current working directory. `codex resume --last` skips the picker and jumps straight to your most recent session. `codex resume <SESSION_ID>` targets a specific run.

---

## 🧠 9. PERSISTENT MEMORY — ✅ ALREADY BUILT

Memories are now configurable, and there is a new `codex debug clear-memories` command to fully reset saved memory state when needed.

Improved memories with workspace-scoped writes, renamed memory settings, and guardrails against saving stale or polluted facts.

Improved memory behavior with diff-based forgetting and usage-aware memory selection.

---

## 📋 10. CODE REVIEW — ✅ ALREADY BUILT

Type `/review` in the CLI to open Codex's review presets.

The CLI launches a dedicated reviewer that reads the diff you select and reports prioritized, actionable findings without touching your working tree.

Get your code reviewed by a separate Codex agent before you commit or push your changes.

---

## ⚡ 11. NON-INTERACTIVE / CI MODE (`exec`) — ✅ ALREADY BUILT

Automate workflows or wire Codex into your existing scripts with the `exec` subcommand. This runs Codex non-interactively, piping the final plan and results back to stdout.

Combine exec with shell scripting to build custom workflows, such as automatically updating changelogs, sorting issues, or enforcing editorial checks before a PR ships.

---

## ☁️ 12. CLOUD TASK DELEGATION — ✅ ALREADY BUILT

The `codex cloud` command lets you triage and launch Codex cloud tasks without leaving the terminal. Run it with no arguments to open an interactive picker, browse active or finished tasks, and apply the changes to your local project.

---

## 🎨 13. RICH TUI (Built by Ratatui Maintainer) — ✅ ALREADY BUILT

The TUI now syntax-highlights fenced code blocks and diffs, adds a `/theme` picker with live preview, and uses better theme-aware diff colors for light and dark terminals.

The TUI syntax-highlights fenced markdown code blocks and file diffs so code is easier to scan during reviews and debugging. Use `/theme` to open the theme picker, preview themes live, and save your selection. You can also add custom `.tmTheme` files under `$CODEX_HOME/themes` and select them in the picker.

**Additional TUI features:**
- Type `@` in the composer to open a fuzzy file search over the workspace root; press Tab or Enter to drop the highlighted path into your message.
- Press Enter while Codex is running to inject new instructions into the current turn, or press Tab to queue a follow-up prompt for the next turn.
- Prefix a line with `!` to run a local shell command (for example, `!ls`). Codex treats the output like a user-provided command result and still applies your approval and sandbox settings.

---

## 🌳 14. GIT WORKTREES — ✅ ALREADY BUILT

Built-in support for worktrees, allowing multiple agents to work on the same repo without interfering with each other.

Added a Worktrees setting to turn automatic cleanup of Codex-managed worktrees on or off.
Added Handoff support for moving a thread between Local and Worktree.

---

## 🔌 15. WIRE PROTOCOL & EXTENSIBILITY — ✅ ALREADY BUILT

Native Security Bindings, Optimized Performance — no runtime garbage collection, resulting in lower memory consumption. Extensible Protocol — they've been working on a "wire protocol" for Codex CLI to allow developers to extend the agent in different languages (including TypeScript, Python, etc) and MCPs (already supported in Rust).

WebAssembly isolation has been proposed for embedded plugins. This way any user could write a plugin in practically any language, compile it to a WebAssembly module, and then securely run it in the same Codex process. This would work anywhere Codex works, and not need to negotiate over a remote protocol.

---

## 🤖 16. SKILLS & AUTOMATIONS — ✅ ALREADY BUILT

The Skills framework extends Codex's reach beyond core code editing.

The app also introduces Automations, enabling agents to run in the background on a schedule.

At OpenAI they use automations to handle routine chores: daily issue triage, summarizing continuous-integration failures, generating daily release briefs, scanning for bugs, and more. In the app's UI, setting up an automation is akin to creating a cron job with prompts.

---

## ⚙️ 17. CONFIGURATION PROFILES — ✅ ALREADY BUILT

Profiles replace manual switching: Instead of changing flags per-run, define profiles in config.toml.

---

## 📊 18. SHELL COMPLETIONS — ✅ ALREADY BUILT

Speed up everyday usage by installing the generated completion scripts for your shell: `codex completion bash`, `codex completion zsh`, `codex completion fish`.

---

## 🖥️ 19. CROSS-PLATFORM SUPPORT — ✅ ALREADY BUILT

The Codex app is now available on Windows. The app gives you one interface for working across projects. The Codex app runs natively on Windows using PowerShell and a native Windows sandbox for bounded permissions.

Windows installation requires choosing between three distinct paths: native PowerShell with an experimental AppContainer sandbox, WSL2 with mature Linux-grade Landlock/seccomp isolation, or a zero-dependency standalone binary.

---

## 📊 20. MULTIMODAL TOOL OUTPUT — ✅ ALREADY BUILT

Custom tools can now return multimodal output, including structured content like images, instead of being limited to plain text.

---

# 🏗️ THE DX MASTER PLAN — What You Add on Top

Here's the complete picture of what you inherit vs. what you build:

| # | Feature | Inherited? | DX Addition |
|---|---|---|---|
| 1 | 🖼️ Image/Vision Input | ✅ | Route to any vision-capable provider |
| 2 | 🤖 Multi-Agent System | ✅ | Each agent can use a DIFFERENT provider/model |
| 3 | 🎤 Voice Input | ✅ | Provider-agnostic transcription (Whisper/Gemini/local) |
| 4 | 🌐 Web Search | ✅ | Pluggable search (Brave, SearXNG, Tavily, Perplexity) |
| 5 | 🛡️ OS-Level Sandbox | ✅ | Inherited as-is (Seatbelt/Landlock/seccomp) |
| 6 | 📝 AGENTS.md | ✅ | Add `DX.md` support + backward compat |
| 7 | 🔗 MCP Client & Server | ✅ | Works with ANY model now |
| 8 | 💾 Session Resume | ✅ | Inherited as-is |
| 9 | 🧠 Persistent Memory | ✅ | Inherited as-is |
| 10 | 📋 Code Review | ✅ | Review with Claude, GPT, or Gemini — your choice |
| 11 | ⚡ CI/`exec` Mode | ✅ | Inherited as-is |
| 12 | ☁️ Cloud Delegation | ✅ | Self-hostable cloud backend |
| 13 | 🎨 Rich TUI (Ratatui) | ✅ | Custom DX themes & branding |
| 14 | 🌳 Git Worktrees | ✅ | Inherited as-is |
| 15 | 🔌 Wire Protocol | ✅ | WASM plugin support |
| 16 | 🤖 Skills & Automations | ✅ | Inherited as-is |
| 17 | ⚙️ Config Profiles | ✅ | Per-provider profiles |
| 18 | 🐚 Shell Completions | ✅ | Inherited as-is |
| 19 | 🖥️ Cross-Platform | ✅ | Inherited as-is |
| 20 | 📊 Multimodal Output | ✅ | Inherited as-is |
| 21 | 🌍 **100+ Providers** | ❌ | **YOUR KILLER FEATURE** |
| 22 | 🏠 **Local Models (Ollama)** | ⚠️ Partial | **Native first-class support** |
| 23 | 🔄 **Provider Hot-Swap** | ❌ | **Switch mid-session: `/provider claude`** |
| 24 | 💰 **Cost Router** | ❌ | **Auto-route cheap tasks to cheap models** |
| 25 | 📊 **Usage Dashboard** | ❌ | **Track spend across all providers** |

---

# 🎯 DX = Codex CLI (20 features) + 100 Providers + Provider Intelligence

```
┌─────────────────────────────────────────────────┐
│                   DX CLI                         │
│                                                  │
│  ┌──────────────────────────────────────────┐   │
│  │     INHERITED FROM CODEX CLI (Rust)       │   │
│  │                                           │   │
│  │  • Image Input    • Multi-Agent           │   │
│  │  • Voice Input    • Web Search            │   │
│  │  • Sandboxing     • AGENTS.md             │   │
│  │  • MCP            • Session Resume        │   │
│  │  • Memory         • Code Review           │   │
│  │  • CI/exec        • Cloud Tasks           │   │
│  │  • Rich TUI       • Git Worktrees         │   │
│  │  • Wire Protocol  • Skills/Automations    │   │
│  │  • Profiles       • Shell Completions     │   │
│  │  • Cross-Platform • Multimodal Output     │   │
│  └──────────────────────────────────────────┘   │
│                                                  │
│  ┌──────────────────────────────────────────┐   │
│  │     YOUR ADDITIONS (The DX Layer)         │   │
│  │                                           │   │
│  │  🌍 100+ Native Rust Providers            │   │
│  │  🔄 Mid-Session Provider Hot-Swap         │   │
│  │  💰 Intelligent Cost Router               │   │
│  │  🏠 First-Class Local Model Support       │   │
│  │  📊 Cross-Provider Usage Dashboard        │   │
│  │  🧪 Provider Benchmarking Suite           │   │
│  └──────────────────────────────────────────┘   │
│                                                  │
└─────────────────────────────────────────────────┘
```

---

> **Bottom line: Codex CLI already has 20+ world-class features that took OpenAI's team + the Ratatui maintainer months to build. You inherit ALL of it under Apache-2.0. The ONLY thing missing is provider freedom. That's YOUR contribution. Add 100+ providers in pure Rust, and DX becomes the undisputed #1 open-source coding CLI on the planet. Ship it bro. 🚀**

Want me to draft the provider trait architecture, the `README.md` for DX, or the migration script from Codex CLI? Let's build! 💪
