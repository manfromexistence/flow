#!/bin/bash
# Restructure script to organize codebase
# Date: March 24, 2026
# Updated: Flat src/ structure with file_browser crates inside src/

set -e  # Exit on error

echo "🚀 Starting project restructure..."
echo ""

# Colors for output
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Step 1: Create new directory structure
echo -e "${BLUE}Step 1: Creating new directory structure...${NC}"
mkdir -p src/file_browser

echo -e "${GREEN}✓ Directories created${NC}"
echo ""

# Step 2: Move dx-fm/src/tui/* directly to src/ (flat structure)
echo -e "${BLUE}Step 2: Moving TUI code to src/ (flat)...${NC}"
if [ -d "dx-fm/src/tui" ]; then
    # Move all subdirectories from tui/
    for dir in dx-fm/src/tui/*/; do
        dirname=$(basename "$dir")
        cp -r "$dir" "src/$dirname"
        echo "  Moved: tui/$dirname → src/$dirname"
    done
    
    # Move all .rs files from tui/
    for file in dx-fm/src/tui/*.rs; do
        if [ -f "$file" ]; then
            filename=$(basename "$file")
            cp "$file" "src/$filename"
            echo "  Moved: tui/$filename → src/$filename"
        fi
    done
    
    echo -e "${GREEN}✓ TUI code moved to src/ (flat)${NC}"
else
    echo -e "${YELLOW}⚠ dx-fm/src/tui not found, skipping${NC}"
fi
echo ""

# Step 3: Move dx-fm/src/* (except tui) to src/file_browser/
echo -e "${BLUE}Step 3: Moving file browser code to src/file_browser/...${NC}"
for dir in dx-fm/src/*/; do
    dirname=$(basename "$dir")
    if [ "$dirname" != "tui" ]; then
        cp -r "$dir" "src/file_browser/$dirname"
        echo "  Moved: $dirname → src/file_browser/$dirname"
    fi
done

# Move file browser .rs files (executor, router, etc.)
for file in dx-fm/src/*.rs; do
    filename=$(basename "$file")
    # Skip main app files, move only file browser related files
    if [ "$filename" = "executor.rs" ] || [ "$filename" = "router.rs" ]; then
        cp "$file" "src/file_browser/$filename"
        echo "  Moved: $filename → src/file_browser/$filename"
    fi
done

# Move top-level app files to src/
for file in main.rs dispatcher.rs root.rs panic.rs signals.rs logs.rs chat.rs chat_input.rs chat_components.rs llm.rs; do
    if [ -f "dx-fm/src/$file" ]; then
        cp "dx-fm/src/$file" "src/$file"
        echo "  Moved to src/: $file"
    fi
done

echo -e "${GREEN}✓ File browser code moved${NC}"
echo ""

# Step 4: Move dx-* crates to src/file_browser/* (nested inside src/)
echo -e "${BLUE}Step 4: Moving dx-* crates to src/file_browser/...${NC}"
for dir in dx-*/; do
    if [ "$dir" != "dx-fm/" ]; then
        # Remove "dx-" prefix
        newname=$(echo "$dir" | sed 's/dx-//' | sed 's/\/$//')
        cp -r "$dir" "src/file_browser/$newname"
        echo "  Moved: $dir → src/file_browser/$newname/"
    fi
done
echo -e "${GREEN}✓ Crates moved to src/file_browser/${NC}"
echo ""

# Step 5: Create mod.rs for file_browser module
echo -e "${BLUE}Step 5: Creating module files...${NC}"
cat > src/file_browser/mod.rs << 'EOF'
// File browser module - contains all dx file browser functionality

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

// Core functionality
pub mod executor;
pub mod router;

// Supporting crates (nested)
pub mod actor;
pub mod adapter;
pub mod boot;
pub mod config;
pub mod core;
pub mod dds;
pub mod fs;
pub mod macro_crate;  // 'macro' is a keyword, use macro_crate
pub mod parser;
pub mod plugin;
pub mod proxy;
pub mod scheduler;
pub mod shared;
pub mod term;
pub mod vfs;
pub mod watcher;
pub mod widgets;

// Re-export commonly used items
pub use executor::Executor;
pub use router::Router;
pub use config::Config;
pub use core::Core;
EOF

echo -e "${GREEN}✓ Module files created${NC}"
echo ""

# Step 6: Update Cargo.toml workspace
echo -e "${BLUE}Step 6: Creating new Cargo.toml...${NC}"
cat > Cargo.toml << 'EOF'
[workspace]
resolver = "2"
members = [
    "src/file_browser/actor",
    "src/file_browser/adapter",
    "src/file_browser/binding",
    "src/file_browser/boot",
    "src/file_browser/build",
    "src/file_browser/cli",
    "src/file_browser/codegen",
    "src/file_browser/config",
    "src/file_browser/core",
    "src/file_browser/dds",
    "src/file_browser/emulator",
    "src/file_browser/ffi",
    "src/file_browser/fs",
    "src/file_browser/macro",
    "src/file_browser/packing",
    "src/file_browser/parser",
    "src/file_browser/plugin",
    "src/file_browser/proxy",
    "src/file_browser/scheduler",
    "src/file_browser/sftp",
    "src/file_browser/shared",
    "src/file_browser/shim",
    "src/file_browser/term",
    "src/file_browser/tty",
    "src/file_browser/vfs",
    "src/file_browser/watcher",
    "src/file_browser/widgets",
]

[workspace.package]
version = "26.2.2"
edition = "2024"
license = "MIT"
authors = ["Your Name <your.email@example.com>"]
repository = "https://github.com/yourusername/dx-tui"
homepage = "https://github.com/yourusername/dx-tui"

[package]
name = "dx-tui"
version.workspace = true
edition.workspace = true
license.workspace = true
authors.workspace = true
repository.workspace = true
homepage.workspace = true
description = "Terminal UI for Codex CLI - AI-powered coding agent"

[dependencies]
# File browser dependencies (all nested in src/file_browser/)
fb-actor = { path = "src/file_browser/actor" }
fb-adapter = { path = "src/file_browser/adapter" }
fb-boot = { path = "src/file_browser/boot" }
fb-config = { path = "src/file_browser/config" }
fb-core = { path = "src/file_browser/core" }
fb-dds = { path = "src/file_browser/dds" }
fb-macro = { path = "src/file_browser/macro" }
fb-parser = { path = "src/file_browser/parser" }
fb-plugin = { path = "src/file_browser/plugin" }
fb-proxy = { path = "src/file_browser/proxy" }
fb-scheduler = { path = "src/file_browser/scheduler" }
fb-shared = { path = "src/file_browser/shared" }
fb-watcher = { path = "src/file_browser/watcher" }
fb-widgets = { path = "src/file_browser/widgets" }

# External dependencies
anyhow = "1.0"
crossterm = "0.28"
ratatui = "0.29"
tokio = { version = "1.42", features = ["full"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
toml = "0.8"
tracing = "0.1"
tracing-subscriber = "0.3"
tachyonfx = "0.8"
thiserror = "2.0"

[profile.release]
lto = "fat"
codegen-units = 1
strip = true
panic = "abort"

[[bin]]
name = "dx"
path = "src/main.rs"
EOF

echo -e "${GREEN}✓ Cargo.toml created${NC}"
echo ""

# Step 7: Update each file_browser crate's Cargo.toml
echo -e "${BLUE}Step 7: Updating crate Cargo.toml files...${NC}"
for dir in src/file_browser/*/; do
    crate_name=$(basename "$dir")
    cargo_toml="$dir/Cargo.toml"
    
    if [ -f "$cargo_toml" ]; then
        # Update package name (dx-* → fb-*)
        sed -i.bak 's/name = "dx-/name = "fb-/g' "$cargo_toml"
        
        # Update dependencies paths (dx-* → fb-*, and update paths)
        sed -i.bak 's/dx-\([a-z]*\) = { path = "..\/dx-/fb-\1 = { path = "..\/file_browser\//g' "$cargo_toml"
        sed -i.bak 's/path = "..\/dx-/path = "..\/file_browser\//g' "$cargo_toml"
        sed -i.bak 's/dx-\([a-z]*\) = { workspace/fb-\1 = { workspace/g' "$cargo_toml"
        
        # Update use statements in Rust files
        find "$dir" -name "*.rs" -type f -exec sed -i.bak 's/use dx_/use fb_/g' {} +
        find "$dir" -name "*.rs" -type f -exec sed -i.bak 's/dx_\([a-z_]*\)::/fb_\1::/g' {} +
        find "$dir" -name "*.rs" -type f -exec sed -i.bak 's/extern crate dx_/extern crate fb_/g' {} +
        
        # Remove backup files
        find "$dir" -name "*.bak" -delete
        
        echo "  Updated: $crate_name"
    fi
done
echo -e "${GREEN}✓ Crate files updated${NC}"
echo ""

# Step 8: Update imports in src/ files
echo -e "${BLUE}Step 8: Updating imports in src/ files...${NC}"
find src/ -maxdepth 1 -name "*.rs" -type f -exec sed -i.bak 's/use dx_/use fb_/g' {} +
find src/ -maxdepth 1 -name "*.rs" -type f -exec sed -i.bak 's/dx_\([a-z_]*\)::/fb_\1::/g' {} +
find src/ -maxdepth 1 -name "*.rs" -type f -exec sed -i.bak 's/use crate::tui/use crate/g' {} +

# Update menu and other subdirectories in src/
for dir in src/*/; do
    if [ "$(basename "$dir")" != "file_browser" ]; then
        find "$dir" -name "*.rs" -type f -exec sed -i.bak 's/use dx_/use fb_/g' {} +
        find "$dir" -name "*.rs" -type f -exec sed -i.bak 's/dx_\([a-z_]*\)::/fb_\1::/g' {} +
        find "$dir" -name "*.rs" -type f -exec sed -i.bak 's/use crate::tui/use crate/g' {} +
    fi
done

# Remove backup files
find src/ -name "*.bak" -delete

echo -e "${GREEN}✓ Imports updated${NC}"
echo ""

# Step 9: Create lib.rs for the main crate
echo -e "${BLUE}Step 9: Creating lib.rs...${NC}"
cat > src/lib.rs << 'EOF'
// DX TUI - Terminal UI for Codex CLI
// Main library file

// TUI modules (flat in src/)
pub mod menu;
pub mod theme;
pub mod render;
pub mod state;
pub mod input;
pub mod animations;
pub mod exit_animation;

// File browser module (nested)
pub mod file_browser;

// Re-export main types
pub use theme::{ChatTheme, ThemeVariant};
pub use state::ChatState;
pub use render::Renderer;
pub use menu::Menu;
pub use file_browser::{Executor, Router};
EOF

echo -e "${GREEN}✓ lib.rs created${NC}"
echo ""

echo -e "${GREEN}========================================${NC}"
echo -e "${GREEN}✅ Restructure complete!${NC}"
echo -e "${GREEN}========================================${NC}"
echo ""
echo -e "${YELLOW}Next steps:${NC}"
echo "1. Review the changes"
echo "2. Run: cargo check"
echo "3. Fix any remaining import issues"
echo "4. Test the application"
echo "5. Delete old dx-* folders when satisfied"
echo ""
echo -e "${BLUE}To delete old folders (CAREFUL!):${NC}"
echo "  rm -rf dx-*/"
echo ""
echo -e "${BLUE}New structure:${NC}"
echo "  src/"
echo "    ├── menu/              (your TUI menu)"
echo "    ├── theme.rs           (your TUI theme)"
echo "    ├── render.rs          (your TUI render)"
echo "    ├── state.rs           (your TUI state)"
echo "    ├── file_browser/      (all file browser code + crates)"
echo "    │   ├── app/"
echo "    │   ├── actor/         (was dx-actor)"
echo "    │   ├── config/        (was dx-config)"
echo "    │   └── ..."
echo "    ├── main.rs"
echo "    └── lib.rs"
echo ""
