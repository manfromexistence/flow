# Yazi UI Layout Changes

## Overview

Modified the Yazi TUI to have a more compact, modern layout with all status information in the header and borders around the interface.

## Changes Made

### 1. Moved Status Bar to Header

All status bar items (mode, size, permissions, position, percent) have been moved from the bottom of the screen to the top header bar, next to the current working directory path.

**Modified Files:**
- `yazi/yazi-plugin/preset/components/header.lua` - Added status bar methods and items
- `yazi/yazi-plugin/preset/components/root.lua` - Removed status bar from layout

### 2. Added Borders Around TUI

Borders now wrap the Yazi interface using `ui.Bar` components on all four edges (top, bottom, left, right).

**Modified Files:**
- `yazi/yazi-plugin/preset/components/root.lua` - Added border bars in the build() function

## Layout Structure

### Before:
```
Header (CWD + Count)
─────────────────────
Tabs (if multiple)
─────────────────────
File Browser
─────────────────────
Status Bar (Mode, Size, etc.)
```

### After:
```
│ Header: CWD | Mode | Size | Perm | % | Pos | Count │
├─────────────────────────────────────────────────────┤
│ Tabs (if multiple)                                  │
├─────────────────────────────────────────────────────┤
│                                                      │
│ File Browser                                        │
│                                                      │
└─────────────────────────────────────────────────────┘
```

## Header Layout

The header now displays (left to right):
- **Left side:**
  - Current working directory (CWD)
  - Mode indicator (NOR/SEL/UNS)
  - File size

- **Right side:**
  - File permissions
  - Scroll percentage (Top/Bot/%)
  - Position (current/total)
  - Selection/yank count

## Technical Details

### Root Component Changes

1. **Layout adjustment** - Removed the bottom status bar constraint (now 3 sections instead of 4)

2. **Border rendering** - Added four `ui.Bar` components in `build()`:
   - Top border: `ui.Bar(ui.Edge.TOP)` with "─" symbol
   - Bottom border: `ui.Bar(ui.Edge.BOTTOM)` with "─" symbol
   - Left border: `ui.Bar(ui.Edge.LEFT)` with "│" symbol
   - Right border: `ui.Bar(ui.Edge.RIGHT)` with "│" symbol

3. **Removed Status component** - Status bar is no longer instantiated in the children array

### Header Component Changes

1. **Added status methods** - Copied all status bar rendering methods:
   - `style()` - Determines mode-based styling
   - `mode()` - Renders mode indicator (NOR/SEL/UNS)
   - `size()` - Renders file size
   - `perm()` - Renders file permissions
   - `percent()` - Renders scroll percentage
   - `position()` - Renders cursor position

2. **Extended item arrays** - Added status items to both left and right item arrays with proper ordering

## Theme Integration

The borders use the existing theme color from `th.pick.border` which is set to `#424242` (dark gray) in the Vercel theme.

## Testing

To test the changes:

```bash
cd yazi
cargo build
cargo run -p yazi-fm
```

Or from the yazi-fm directory:

```bash
cd yazi/yazi-fm
cargo run
```

The interface should now display with:
- All status information in the top header
- Borders on all four edges
- More vertical space for the file browser
- Cleaner, more compact layout

## Troubleshooting

If the UI doesn't display correctly:

1. Make sure no yazi processes are running: `Get-Process yazi | Stop-Process -Force`
2. Rebuild: `cargo clean && cargo build`
3. Check for Lua errors in the terminal output

## Reverting Changes

To revert to the original layout, restore these files from git:
- `yazi/yazi-plugin/preset/components/root.lua`
- `yazi/yazi-plugin/preset/components/header.lua`
