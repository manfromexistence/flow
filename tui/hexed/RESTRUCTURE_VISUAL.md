# Visual Restructure Guide

## BEFORE (Current Structure)

```
project-root/
├── dx-fm/
│   └── src/
│       ├── tui/              ← Your custom TUI
│       ├── app/              ← File browser stuff
│       ├── cmp/              ← File browser stuff
│       ├── confirm/          ← File browser stuff
│       ├── help/             ← File browser stuff
│       ├── input/            ← File browser stuff
│       ├── mgr/              ← File browser stuff
│       ├── notify/           ← File browser stuff
│       ├── pick/             ← File browser stuff
│       ├── spot/             ← File browser stuff
│       ├── tasks/            ← File browser stuff
│       ├── which/            ← File browser stuff
│       ├── main.rs
│       ├── dispatcher.rs
│       ├── executor.rs
│       ├── router.rs
│       └── ...
│
├── dx-actor/               ← Separate crate
├── dx-adapter/             ← Separate crate
├── dx-boot/                ← Separate crate
├── dx-config/              ← Separate crate
├── dx-core/                ← Separate crate
├── dx-dds/                 ← Separate crate
├── ... (20+ dx-* crates)
└── Cargo.toml
```

**Problems:**
- ❌ TUI code mixed with file browser code
- ❌ "dx-" prefix everywhere (not your brand)
- ❌ Hard to find your custom code
- ❌ Confusing structure

---

## AFTER (New Structure)

```
project-root/
├── src/
│   ├── menu/                 ✨ YOUR TUI MENU (flat in src/)
│   │   ├── mod.rs
│   │   ├── menu_data.rs
│   │   ├── menu_render.rs
│   │   ├── menu_navigation.rs
│   │   ├── keyboard_mappings.rs
│   │   └── submenus/
│   │
│   ├── theme.rs              ✨ YOUR TUI (flat in src/)
│   ├── render.rs             ✨ YOUR TUI
│   ├── state.rs              ✨ YOUR TUI
│   ├── animations.rs         ✨ YOUR TUI
│   ├── exit_animation.rs     ✨ YOUR TUI
│   ├── input.rs              ✨ YOUR TUI
│   │
│   ├── file_browser/         📁 ALL FILE BROWSER CODE (nested)
│   │   ├── app/              (from dx-fm/src/app)
│   │   ├── cmp/              (from dx-fm/src/cmp)
│   │   ├── confirm/          (from dx-fm/src/confirm)
│   │   ├── help/             (from dx-fm/src/help)
│   │   ├── input/            (from dx-fm/src/input)
│   │   ├── mgr/              (from dx-fm/src/mgr)
│   │   ├── notify/           (from dx-fm/src/notify)
│   │   ├── pick/             (from dx-fm/src/pick)
│   │   ├── spot/             (from dx-fm/src/spot)
│   │   ├── tasks/            (from dx-fm/src/tasks)
│   │   ├── which/            (from dx-fm/src/which)
│   │   ├── executor.rs
│   │   ├── router.rs
│   │   │
│   │   ├── actor/            📦 (was dx-actor)
│   │   ├── adapter/          � (was dx-adapter)
│   │   ├── boot/             📦 (was dx-boot)
│   │   ├── config/           📦 (was dx-config)
│   │   ├── core/             📦 (was dx-core)
│   │   ├── dds/              📦 (was dx-dds)
│   │   ├── fs/               📦 (was dx-fs)
│   │   ├── macro/            📦 (was dx-macro)
│   │   ├── parser/           📦 (was dx-parser)
│   │   ├── plugin/           📦 (was dx-plugin)
│   │   ├── proxy/            📦 (was dx-proxy)
│   │   ├── scheduler/        📦 (was dx-scheduler)
│   │   ├── shared/           📦 (was dx-shared)
│   │   ├── term/             📦 (was dx-term)
│   │   ├── vfs/              📦 (was dx-vfs)
│   │   ├── watcher/          📦 (was dx-watcher)
│   │   ├── widgets/          📦 (was dx-widgets)
│   │   └── mod.rs
│   │
│   ├── lib.rs                🎯 Main library
│   ├── main.rs               🚀 Entry point
│   ├── dispatcher.rs
│   ├── root.rs
│   ├── chat.rs
│   └── ...
│
└── Cargo.toml                📝 Clean workspace config
```

**Benefits:**
- ✅ Flat structure: TUI code directly in `src/` (no extra nesting)
- ✅ Everything file browser related in one place: `src/file_browser/`
- ✅ No "dx" branding pollution
- ✅ Easy to find your custom code (src/menu/, src/theme.rs, etc.)
- ✅ File browser completely contained in one folder
- ✅ Supporting crates nested inside `src/file_browser/`
- ✅ Clean, professional structure
- ✅ Single workspace, everything in `src/`

---

## Import Changes

### Before:
```rust
use dx_config::Config;
use dx_core::Core;
use dx_shared::Data;
use crate::app::App;
use crate::tui::Renderer;
```

### After:
```rust
use fb_config::Config;
use fb_core::Core;
use fb_shared::Data;
use crate::file_browser::app::App;
use crate::menu::Menu;        // Flat in src/
use crate::theme::ChatTheme;  // Flat in src/
```

---

## Cargo.toml Changes

### Before:
```toml
[dependencies]
dx-actor = { path = "dx-actor" }
dx-config = { path = "dx-config" }
dx-core = { path = "dx-core" }
```

### After:
```toml
[dependencies]
fb-actor = { path = "src/file_browser/actor" }
fb-config = { path = "src/file_browser/config" }
fb-core = { path = "src/file_browser/core" }
```

---

## Module Structure

### Main Library (src/lib.rs):
```rust
// TUI modules (flat in src/)
pub mod menu;
pub mod theme;
pub mod render;
pub mod state;
pub mod input;
pub mod animations;

// File browser module (nested)
pub mod file_browser;

// Re-exports
pub use theme::{ChatTheme, ThemeVariant};
pub use state::ChatState;
pub use render::Renderer;
pub use menu::Menu;
pub use file_browser::{Executor, Router};
```

### File Browser Module (src/file_browser/mod.rs):
```rust
// File browser UI components
pub mod app;
pub mod cmp;
pub mod confirm;
pub mod help;
pub mod input;
pub mod mgr;
pub mod notify;
pub mod pick;
pub mod spot;
pub mod tasks;
pub mod which;
pub mod executor;
pub mod router;

// Supporting crates (nested)
pub mod actor;
pub mod adapter;
pub mod config;
pub mod core;
// ... all other crates

pub use executor::Executor;
pub use router::Router;
```

---

## How to Use the Script

1. **Backup your work:**
   ```bash
   git add -A
   git commit -m "Backup before restructure"
   ```

2. **Make script executable:**
   ```bash
   chmod +x restructure.sh
   ```

3. **Run the script:**
   ```bash
   ./restructure.sh
   ```

4. **Check the results:**
   ```bash
   cargo check
   ```

5. **Fix any remaining issues:**
   - Update imports that the script missed
   - Fix module paths
   - Test compilation

6. **Clean up old folders (when satisfied):**
   ```bash
   rm -rf dx-*/
   ```

---

## What the Script Does

1. ✅ Creates `src/tui/` and `src/file_browser/` directories
2. ✅ Moves your TUI code to `src/tui/`
3. ✅ Moves file browser code to `src/file_browser/`
4. ✅ Moves all dx-* crates to `file_browser/` (without dx- prefix)
5. ✅ Updates all Cargo.toml files (dx-* → fb-*)
6. ✅ Updates all imports in Rust files
7. ✅ Creates module files (mod.rs, lib.rs)
8. ✅ Creates new workspace Cargo.toml

---

## Expected Result

After running the script and fixing any issues:

```bash
$ tree -L 2 -d
.
├── src
│   ├── menu             ← Your TUI menu (flat)
│   ├── animations       ← Your TUI animations (flat)
│   └── file_browser     ← ALL file browser code + crates
│       ├── app
│       ├── actor
│       ├── config
│       ├── core
│       └── ... (all crates nested here)
└── target

$ cargo check
   Compiling fb-shared v26.2.2 (src/file_browser/shared)
   Compiling fb-config v26.2.2 (src/file_browser/config)
   Compiling fb-core v26.2.2 (src/file_browser/core)
   ...
   Compiling dx-tui v26.2.2
    Finished dev [unoptimized + debuginfo] target(s) in 45.2s
```

---

## Troubleshooting

### Issue: "cannot find crate `dx_*`"
**Fix:** Search and replace remaining `dx_` references:
```bash
find src/ -name "*.rs" -exec sed -i 's/dx_/fb_/g' {} +
```

### Issue: "unresolved import `crate::app`"
**Fix:** Update to `crate::file_browser::app`

### Issue: Module not found
**Fix:** Check that mod.rs files exist and export the modules

---

## Next Steps After Restructure

1. Update README.md with new structure
2. Update documentation
3. Rename binary from "dx" to "dx" or "codex-tui"
4. Update config paths (~/.dx → ~/.dx)
5. Test all functionality
6. Celebrate! 🎉
