# Project Restructuring Plan
**Date:** March 24, 2026  
**Goal:** Clean separation of TUI code and file browser code

## Target Structure

```
project-root/
├── src/
│   ├── tui/              ← From dx-fm/src/tui/ (your custom TUI)
│   │   ├── menu/         ← Your menu system
│   │   ├── theme.rs
│   │   ├── render.rs
│   │   ├── state.rs
│   │   └── ...
│   │
│   ├── file_browser/     ← All dx file browser code
│   │   ├── app/          ← From dx-fm/src/app/
│   │   ├── cmp/          ← From dx-fm/src/cmp/
│   │   ├── confirm/      ← From dx-fm/src/confirm/
│   │   ├── help/         ← From dx-fm/src/help/
│   │   ├── input/        ← From dx-fm/src/input/
│   │   ├── mgr/          ← From dx-fm/src/mgr/
│   │   ├── notify/       ← From dx-fm/src/notify/
│   │   ├── pick/         ← From dx-fm/src/pick/
│   │   ├── spot/         ← From dx-fm/src/spot/
│   │   ├── tasks/        ← From dx-fm/src/tasks/
│   │   ├── which/        ← From dx-fm/src/which/
│   │   ├── executor.rs   ← From dx-fm/src/executor.rs
│   │   ├── router.rs     ← From dx-fm/src/router.rs
│   │   └── mod.rs        ← New module file
│   │
│   ├── main.rs           ← From dx-fm/src/main.rs
│   ├── dispatcher.rs     ← From dx-fm/src/dispatcher.rs
│   ├── root.rs           ← From dx-fm/src/root.rs
│   ├── panic.rs          ← From dx-fm/src/panic.rs
│   ├── signals.rs        ← From dx-fm/src/signals.rs
│   ├── logs.rs           ← From dx-fm/src/logs.rs
│   ├── chat.rs           ← From dx-fm/src/chat.rs
│   ├── chat_input.rs     ← From dx-fm/src/chat_input.rs
│   ├── chat_components.rs← From dx-fm/src/chat_components.rs
│   └── llm.rs            ← From dx-fm/src/llm.rs
│
├── file_browser/         ← All dx-* crates (renamed, no dx prefix)
│   ├── actor/            ← From dx-actor/
│   ├── adapter/          ← From dx-adapter/
│   ├── binding/          ← From dx-binding/
│   ├── boot/             ← From dx-boot/
│   ├── build/            ← From dx-build/
│   ├── cli/              ← From dx-cli/
│   ├── codegen/          ← From dx-codegen/
│   ├── config/           ← From dx-config/
│   ├── core/             ← From dx-core/
│   ├── dds/              ← From dx-dds/
│   ├── emulator/         ← From dx-emulator/
│   ├── ffi/              ← From dx-ffi/
│   ├── fs/               ← From dx-fs/
│   ├── macro/            ← From dx-macro/
│   ├── packing/          ← From dx-packing/
│   ├── parser/           ← From dx-parser/
│   ├── plugin/           ← From dx-plugin/
│   ├── proxy/            ← From dx-proxy/
│   ├── scheduler/        ← From dx-scheduler/
│   ├── sftp/             ← From dx-sftp/
│   ├── shared/           ← From dx-shared/
│   ├── shim/             ← From dx-shim/
│   ├── term/             ← From dx-term/
│   ├── tty/              ← From dx-tty/
│   ├── vfs/              ← From dx-vfs/
│   ├── watcher/          ← From dx-watcher/
│   └── widgets/          ← From dx-widgets/
│
├── Cargo.toml            ← Updated workspace config
└── ...
```

## Key Changes

1. **Root `src/` folder contains:**
   - `tui/` - Your custom TUI code (menu system, themes, etc.)
   - `file_browser/` - dx file browser integration code
   - Top-level files (main.rs, dispatcher.rs, etc.)

2. **`file_browser/` crate folder contains:**
   - All dx-* crates renamed without the "dx-" prefix
   - Each becomes a sub-crate in the workspace

3. **Clean separation:**
   - TUI code is isolated and easy to maintain
   - File browser code is contained in its own namespace
   - No "dx" naming pollution in your codebase

## Migration Steps

### Step 1: Create new structure
### Step 2: Move dx-fm/src/tui/ to src/tui/
### Step 3: Move dx-fm/src/* (except tui) to src/file_browser/
### Step 4: Move all dx-* crates to file_browser/*
### Step 5: Update all imports and module paths
### Step 6: Update Cargo.toml workspace configuration
### Step 7: Test compilation

