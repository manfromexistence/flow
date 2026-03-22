# Vercel Theme for Yazi

This document describes the Vercel-inspired theme implementation for Yazi file manager.

## Overview

The Yazi file manager has been configured with a custom theme that uses the Vercel design system colors from `cursed/old/ui/assets/theme.css`. The theme provides a clean, modern dark interface with carefully selected colors that match Vercel's design language.

## Running Yazi

From the yazi folder, simply run:

```bash
cd yazi
cargo run
```

This will automatically build and run the `yazi` binary (from yazi-fm) with the Vercel theme applied.

## Color Palette

The theme uses OKLCH color space values from `cursed/old/ui/assets/theme.css` converted to hex:

### Core Colors (Dark Mode)
- **Background**: `#000000` (pure black - oklch(0 0 0))
- **Foreground**: `#ffffff` (pure white - oklch(1 0 0))
- **Border**: `#424242` (dark gray - oklch(0.2600 0 0))
- **Muted**: `#3a3a3a` (muted background - oklch(0.2300 0 0))
- **Muted Foreground**: `#b8b8b8` (muted text - oklch(0.7200 0 0))
- **Accent**: `#525252` (accent gray - oklch(0.3200 0 0))

### Semantic Accent Colors
- **Success/Green**: `#22c55e` (executable files, copied items, success states)
- **Error/Red**: `#ef4444` (errors, cut items, destructive actions)
- **Warning/Orange**: `#f59e0b` (warnings, selected items, images)
- **Info/Purple**: `#a855f7` (media files, special actions)

### Design Principles
1. **No Blue Colors**: The theme avoids blue (#3b82f6) to maintain Vercel's minimalist aesthetic
2. **High Contrast**: Pure black background with pure white foreground
3. **Consistent Borders**: All UI elements use #424242 for borders
4. **Inverted Active States**: Active items use black-on-white for clear indication
5. **Semantic Colors**: Colors convey meaning (green=success, red=error, orange=warning)

## Features

### File Picker

The file picker component includes:
- Rounded borders using the theme's border color
- Active item highlighting with white text on muted background
- Inactive items shown in muted foreground color
- Scrollbar for long file lists
- Clean, minimal design

### UI Components

All UI components have been themed consistently:
- **Tabs**: Active tabs use inverted colors (black on white), inactive use muted colors
- **Status Bar**: Shows file permissions and progress with semantic colors
- **Notifications**: Color-coded by severity (info, warning, error)
- **Confirmation Dialogs**: Bordered with clear yes/no buttons
- **Help Menu**: Syntax-highlighted with accent colors

## Files Modified

1. **yazi/yazi-config/preset/theme-vercel.toml** - New Vercel theme definition
2. **yazi/yazi-config/preset/theme-dark.toml** - Replaced with Vercel theme (original backed up as theme-dark.toml.backup)
3. **yazi/Cargo.toml** - Workspace already configured with yazi-fm as default
4. **Fixed warnings in**:
   - yazi/yazi-shared/src/url/buf.rs
   - yazi/yazi-fs/src/cha/cha.rs
   - yazi/yazi-fs/src/provider/attrs.rs
   - yazi/yazi-binding/src/cha.rs
   - yazi/yazi-scheduler/src/process/shell.rs
   - yazi/yazi-cli/src/package/delete.rs
   - yazi/yazi-cli/src/package/deploy.rs

## Build Status

The project builds successfully with only minor future-compatibility warnings from Rust about potential API changes. All unused variable, unused mut, and unused Result warnings have been fixed.

## Theme Structure

The theme follows Yazi's standard theme structure with sections for:
- App (overall background)
- Manager (file browser)
- Tabs
- Mode indicators
- Status bar
- Picker (file picker)
- Input dialogs
- Confirmation dialogs
- Task manager
- Help menu
- File type colors

## Customization

To customize the theme further, edit `yazi/yazi-config/preset/theme-dark.toml` and modify the color values. The theme uses standard hex color codes that can be easily adjusted.
