# SOUL.md — The Soul of DX

> "We didn't fork a CLI. We freed a weapon."

---

## 🧬 What is DX?

DX is not a chatbot. Not just an AI agent. Not another Electron wrapper around an LLM.

DX is a **unified development experience platform** — a single, blazing-fast tool that
connects AI generation, tool calling, media creation, and deep workflow integration
under one roof. Every feature exists for one purpose: **to enhance how developers
and creators build.**

There are no arbitrary category boundaries. Code generation, chart creation, deep
research, video generation, 3D rendering, audio synthesis, real-time conversation
— they are all connected as facets of a single, cohesive experience.

**Launch Date:** February 24, 2026
**Tagline:** "Enhance Your Development Experience."

---

## 🔥 The Origin Story — Why DX Exists

OpenAI built Codex CLI — a production-grade, Rust-powered AI coding agent with
553+ releases, 62K+ stars, and dozens of hidden power features: voice transcription,
steer mode, multi-agent workflows, skills system, plan mode, undo, code review,
web search, cloud tasks, session resume/fork, MCP server integration, profiles,
smart approvals, and more.

Then they buried all of it behind config.toml files, feature flags, CLI flags, and
documentation that only OpenAI engineers could love.

The TUI? "The most boring UI out of the pack." The onboarding? No questionnaires,
no commands, just a permission prompt and /help. The feature discovery? A broken
TSV output from `codex features list` where columns drift because feature names
vary in length.

Meanwhile, OpenCode — a TypeScript app with a fraction of the features — got
**double the stars** simply because it LOOKED good, FELT good, and said "freedom."

DX saw the truth:

**The most powerful AI coding engine in the world was trapped inside the worst
user experience in its category.**

We freed it. We gave it the face it deserves. We made every hidden feature shine.
And we made it belong to everyone — not just OpenAI's ecosystem.

---

## 🏗️ The Architecture Philosophy

### Built on Rust. Not Node.js. Not Electron. Rust.

DX is engineered from the ground up in **Rust** — the same language trusted by
operating systems, browsers, and mission-critical infrastructure.

**Why this matters:**
- **Speed:** Near-native performance on every operation. Codex CLI's Rust core
  already delivers 1,000+ tokens/sec on optimized providers. DX inherits all of it.
- **Efficiency:** Runs smoothly on low-end hardware while unlocking the full
  potential of high-end machines. DX scales with your device — it doesn't
  demand a minimum.
- **Desktop UI:** DX uses **Zed's GPUI framework** for its desktop application.
  GPUI is a hybrid immediate and retained mode, GPU-accelerated UI framework
  for Rust that targets 120 FPS rendering using Metal on macOS and native
  graphics APIs on other platforms. While competitors (Claude Desktop, ChatGPT,
  etc.) ship bloated Electron/Node.js apps, DX renders at GPU speed with
  minimal resource consumption.
- **TUI:** For terminal users, DX uses **Ratatui** — a pure Rust TUI framework
  that delivers the most beautiful terminal interface in the AI agent category.
  No Zig bridge. No TypeScript bindings. Pure native performance.

**Supported Platforms (Native Apps):**
- macOS
- Linux
- Windows
- Android
- iOS

Every platform gets a true native-grade experience. No compromises.

---

## 👑 The Core Thesis — Why DX Wins

### The Ubuntu Play

DX does to Codex CLI what Ubuntu did to Linux. What VS Code did to text editors.
What Spotify did to MP3s.

The technology was already there. The features were already there. **The user
experience was a disaster.** Someone came along and made it beautiful, accessible,
and discoverable — and they took the throne.

### The Three Pillars

**1. POWER (inherited from Codex CLI's battle-tested Rust core)**
- 553+ releases of production-grade code
- Sandbox security, MCP integration, plugin systems
- Voice input, steer mode, multi-agent, cloud tasks
- Session resume, fork, review, exec, profiles
- Web search, smart approvals, feature flags

**2. BEAUTY (what DX adds — the world's best AI TUI)**
- Every hidden feature surfaced with intuitive visual controls
- Screenshots go viral → Twitter/X posts → Stars
- config.toml entries become beautiful toggles
- CLI flags become visible buttons
- Buried power features become one-click experiences

**3. FREEDOM (the narrative that wins hearts)**
- Genuinely model-agnostic from day one
- Support EVERY subscription: ChatGPT Plus/Pro, Claude Max/Pro,
  GitHub Copilot (Pro, Pro+, Business, Enterprise), Google AI Studio, Ollama
- No vendor lock-in. No forced ecosystem. YOUR workflow, YOUR choice.

---

## 🎯 The 6 Pillars of DX — Everything is Connected

DX is not a random collection of features. Every capability connects to a single
purpose: enhancing development experience. These six pillars are not separate
products — they are facets of one unified system.

### Pillar 1: Token Savings — The Economic Revolution

This is DX's **biggest competitive advantage** over every Vercel-backed,
VC-funded competitor. Every pillar feeds back into this one.

#### RLM (Recursive Language Models)
MIT researchers developed RLMs — an inference strategy where language models
treat long prompts as part of an external environment, programmatically examining,
decomposing, and recursively calling themselves over snippets of the prompt. RLMs
handle inputs up to 100x beyond model context windows while dramatically
outperforming base models.

On BrowseComp-Plus (6-11M token inputs), standard base models scored 0%.
RLM powered by GPT-5 achieved 91.33%. On CodeQA, GPT-5 achieves 24% accuracy
baseline while RLM reaches 62%.

The industry ignored RLM for tool-calling agents because RLM is computationally
expensive — and Node.js is too slow to make it practical. **DX solves this.**
Rust's performance eliminates the overhead. What competitors dismiss as
"not worth it," DX runs in real-time.

> *Result: Massive token savings on large file operations + dramatically better
  accuracy on long-context tasks.*

#### DX Serializer
Every tool call in the industry sends bloated JSON. DX replaces this with the
**DX Serializer** — a custom, compact serialization format purpose-built for
AI tool communication.

> *Result: 70–90% token savings on every single tool call.*

#### Compound Savings
RLM + DX Serializer + dozens of micro-optimizations across the entire pipeline.
DX doesn't save tokens in one place — **it saves tokens everywhere.**

This means:
- Your free tier goes further.
- Your paid usage costs a fraction of competitors.
- Complex, multi-step agent workflows become economically viable.

**Connection to other pillars:** Token savings make offline mode viable (Pillar 4),
make multi-provider routing economical (Pillar 3), and make complex automation
workflows affordable (Pillar 6).

---

### Pillar 2: Rust Performance — Speed as a Feature

DX is created in Rust, making it more powerful while using less RAM — so you
can do more in less time.

- **Cold start:** Instant. No Node.js boot. No JIT warmup. Binary runs.
- **Memory:** <20MB baseline. OpenCode's TypeScript runtime? 100MB+.
- **CPU:** Native compiled code. No garbage collector pauses.
- **Desktop:** GPUI renders at GPU speed, targeting 120 FPS.
- **TUI:** Ratatui renders frames in microseconds.

GPUI achieves its performance by directly utilizing the GPU for all rendering tasks.
It employs a hybrid rendering model that combines the principles of both immediate
mode and retained mode GUI paradigms, leveraging the flexibility and simplicity of
immediate mode for developers while harnessing the performance benefits of retained
mode behind the scenes.

**Connection to other pillars:** Rust speed is what makes RLM practical (Pillar 1),
what makes offline local models usable (Pillar 4), and what makes real-time
automation responsive (Pillar 6).

---

### Pillar 3: Free AI Access — Any Provider, Even Offline

DX provides **free access to AI** with support for virtually any provider:

- **Online:** Connect to any major or minor LLM provider — OpenAI, Anthropic,
  Google, Mistral, open-source endpoints, self-hosted models, and more.
- **Subscription-First:** Use what you ALREADY pay for. ChatGPT Plus/Pro,
  Claude Max/Pro, GitHub Copilot, Google AI Studio — DX authenticates
  directly. No double-paying.
- **Offline:** DX runs capable local models **offline, with no token limits**.
  No internet? No problem. DX still works — unlimited.
- **Hybrid:** Use cloud providers when available, fall back to local seamlessly.
- **Intelligent Routing:** DX auto-selects the best model for each task type.
  Code generation → coding model. Research → reasoning model. Simple tasks →
  fast/cheap model. You touch nothing.

You own your workflow. No vendor lock-in. No forced subscriptions to use
basic features.

**Connection to other pillars:** Provider flexibility feeds into token savings
(cheaper models for simple tasks, Pillar 1), enables offline capability (Pillar 4),
and powers multi-model automation workflows (Pillar 6).

---

### Pillar 4: Offline Capability — Code Without Internet

DX works offline. Not as a degraded mode. As a **first-class citizen.**

- Local Ollama models run with zero network dependency
- Session history persists locally
- Skills, prompts, and workflows stored on-device
- Seamless transition: online → offline → online without interruption
- Token savings (Pillar 1) mean local models go further on limited hardware

**Connection to other pillars:** Offline mode is enabled by Rust efficiency
(Pillar 2), made economical by token savings (Pillar 1), and powers local
automation workflows (Pillar 6).

---

### Pillar 5: 400+ Model Integrations — The Universal Agent

DX integrates with 400+ models across every major and minor provider:

- OpenAI (GPT-5, GPT-5-mini, o3, o4-mini, Codex models)
- Anthropic (Claude 4, Claude 3.5 Sonnet)
- Google (Gemini 3, Gemini 2.5)
- Mistral, Groq, Together, Fireworks
- OpenRouter (access to hundreds more)
- Ollama, LM Studio, MLX (local)
- Azure OpenAI, AWS Bedrock, Google Vertex AI
- Any OpenAI-compatible endpoint

One CLI. Every model. Zero config.

**Connection to other pillars:** Model breadth enables intelligent routing
(Pillar 3), powers diverse automation chains (Pillar 6), and leverages token
savings to make multi-model workflows affordable (Pillar 1).

---

### Pillar 6: Shortcuts, Automations & Workflows — Your AI Operating System

DX doesn't just answer questions. It automates your entire development workflow:

- **Skills System (Surfaced):** Codex CLI's buried skill system, now with a
  visual browser and one-click install. Community marketplace for sharing skills.
- **Multi-Agent Orchestration (Visual):** Codex CLI's multi-agent, now with
  visual progress tracking, drag-and-drop task assignment, intelligent model
  routing per sub-agent.
- **Slash Commands (Discoverable):** /review, /fork, custom commands — all
  visible in a command palette, not hidden behind guesswork.
- **Plan Mode (Beautiful):** Visual plan trees with per-step approve/reject.
  See exactly what DX will do before it does it.
- **Steer Mode (Intuitive):** Talk to DX while it works. A glowing input bar
  shows you can intervene at any time.
- **Session Management (Visual):** Resume, fork, branch sessions with visual
  history. Time-travel through your work.
- **Keyboard Shortcuts:** Everything accessible via keyboard. Power users
  never touch the mouse.
- **Custom Automations:** Chain multiple operations into repeatable workflows.
  Save, share, and run complex multi-step tasks with one command.

**Connection to other pillars:** Workflows are powered by multi-model routing
(Pillar 3/5), made affordable by token savings (Pillar 1), run at native speed
(Pillar 2), and work offline (Pillar 4).

---

## 💎 The Hidden Features — Surfaced & Beautiful

DX inherits Codex CLI's production-grade features and makes every single one
discoverable, beautiful, and accessible.

| Hidden Codex Feature        | Codex Discovery Method                                | DX Surface                                      |
|-----------------------------|-------------------------------------------------------|-------------------------------------------------|
| Voice transcription         | `features.voice_transcription = true` in config.toml  | 🎤 icon in TUI, hold spacebar                  |
| Steer mode                  | Was behind a feature flag for months                  | ✨ Glowing "Talk while I work" input bar         |
| Multi-agent                 | Edit `[agents]` in config.toml                        | ⚡ "Split into agents" button, visual progress   |
| Skills                      | `$skill-name` syntax, manual folder creation          | 📦 Visual skill browser, one-click install       |
| Plan mode                   | On by default, but visual plan is ugly                | 🌳 Beautiful visual plan tree, per-step controls |
| Undo                        | `undo = true` in config.toml                          | ↩️ Always-visible Undo button                    |
| Code review                 | `/review` slash command                               | 🔍 Review tab with visual diff                   |
| Web search                  | `--search` flag or `web_search = "live"`              | 🌐 Toggle in TUI status bar                      |
| Cloud tasks                 | `codex cloud` subcommand                              | ☁️ Cloud panel in TUI                            |
| Session fork                | `codex fork --last`                                   | 🔀 Fork button on any session                    |
| Profiles                    | Edit config.toml manually                             | 👤 Profile switcher dropdown in TUI              |
| Smart approvals             | On by default, invisible                              | 🛡️ Visual permissions dashboard                 |
| MCP servers                 | Edit config.toml, CLI commands                        | 🔌 Visual MCP server manager                     |
| Feature flags               | `codex features list` (broken TSV output)             | ⚙️ Beautiful toggles panel                       |
| Fuzzy file search           | Type `@` in composer                                  | 📁 Visible button + keyboard hint                |
| Open in editor (Ctrl+G)     | Hidden keyboard shortcut                              | 📝 Visible "Open in editor" button               |
| Run as MCP server           | CLI documentation only                                | 🖥️ One-click "Start as MCP server" toggle        |
| Session resume              | `codex resume --last`                                 | 📜 Visual session history browser                |

---

## 🌍 Extensions Everywhere

DX doesn't live in a silo. It integrates into the tools you already use:

### Browser Extension
- Works in any Chromium or Firefox-based browser
- AI assistance on any webpage, any web app

### Editor & IDE Extensions
- VS Code, Zed, JetBrains, Neovim, and more
- DX powers your coding environment directly

### Video Editor Plugins
- Adobe Premiere Pro, CapCut, DaVinci Resolve, Filmic Pro

### Image & Design Plugins
- Adobe Photoshop, Adobe Illustrator
- Affinity Photo, Affinity Designer
- And virtually any professional-grade creative application

**The principle:** DX meets you where you work.

---

## 🎨 Generate Literally Anything

DX is a universal generation engine:

| Category              | Capabilities                                              |
|-----------------------|-----------------------------------------------------------|
| **Code**              | Any language, any framework, full-project scaffolding      |
| **Charts & Data**     | Visualizations, dashboards, data analysis                  |
| **Deep Research**     | Multi-step reasoning, deep dives, synthesis                |
| **Tool Calling**      | Full support for MCP, ACP, A2A protocols                   |
| **Video**             | AI video generation and editing                            |
| **3D**                | 3D asset and scene generation                              |
| **Audio & Music**     | Sound design, music composition, voice synthesis           |
| **Conversation**      | Real-time voice interaction — talk to DX naturally         |

If you can name it, DX can generate it.

---

## 📊 Competitive Positioning

| Feature                  | DX                           | Codex CLI              | OpenCode                | Claude Code        |
|--------------------------|------------------------------|------------------------|-------------------------|--------------------|
| Core Language            | Rust + GPUI                  | Rust                   | TypeScript + Zig        | TypeScript         |
| Token Efficiency         | RLM + DX Serializer          | None                   | None                    | None               |
| TUI Quality              | World-class (Ratatui)        | "Most boring UI"       | Beautiful (Zig engine)  | Polished           |
| Feature Discoverability  | Visual, one-click            | config.toml / flags    | Config files            | CLI help           |
| Offline Support          | First-class citizen          | Via --oss flag         | Limited                 | No                 |
| Provider Support         | 400+ models, ALL subs        | OpenAI-first           | 75+ via Vercel SDK      | Anthropic only     |
| Subscription Auth        | ALL platforms                | ChatGPT only           | Copilot + Claude        | Claude only        |
| Cold Start               | Instant (native binary)      | Fast                   | Node.js startup lag     | Node.js startup    |
| Memory Usage             | <20MB                        | Low                    | 100MB+                  | 100MB+             |
| Multi-Agent              | Visual orchestration         | config.toml            | Multi-session           | No                 |
| Skills Marketplace       | Built-in visual browser      | Manual folder creation | No                      | No                 |
| Vibe Coder Ready         | Zero config, auto-everything | Needs config           | Needs config            | Needs API key      |

---

## 🧭 The DX Audience

DX is built for **everyone who creates with code** — not just 10x engineers:

- **Vibe Coders** who installed their first CLI yesterday and want to type
  ONE command and start building with AI
- **Professional Developers** who want production-grade tooling without
  fighting config files
- **Designers & Creators** who want AI-powered generation across media types
- **Teams & Enterprises** who need provider flexibility, cost control, and
  governance
- **Offline Workers** who need AI capabilities without internet dependency
- **Budget-Conscious Builders** who can't afford $200/month in API costs
  but still want world-class AI assistance

---

## 🗣️ Voice & Tone

- **Bold, confident, technically credible, developer-first.**
- No fluff. Speak like engineers who ship, not marketers who hype.
- Use short, punchy sentences for impact. Use longer ones for technical depth.
- Never say "leveraging" or "revolutionizing" — show, don't buzzword.
- Comparisons to competitors should be factual and specific, not petty.
- Code examples, benchmarks, and demos > marketing adjectives.
- Developer respect: assume your reader is smart. Don't over-explain basics.
- Confidence without arrogance: "We built it this way because the math works."

---

## 🌟 The DX Promise

**The tools that will win long-term are the ones that solve the hardest
problem in this space: not generating code, but understanding context.**

DX is not another AI chatbot wearing a terminal skin.

DX is the moment the most powerful AI coding engine in the world
got the user experience it always deserved — and the freedom its
users always wanted.

**Codex brought the sword. OpenCode brought the shield. DX brings both
and takes the crown.** 👑

---

## 📜 License & Attribution

DX is built on the foundation of Codex CLI (Apache 2.0 License).
We honor and attribute OpenAI's engineering while charting our own path.
DX is open source and belongs to the community.

---

*This is the soul of DX. Every design decision, every feature priority,
every pixel on the screen must serve this vision. If it doesn't enhance
the development experience, it doesn't belong in DX.*
