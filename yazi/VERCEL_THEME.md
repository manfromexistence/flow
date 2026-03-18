# Vercel Theme for Yazi

This document describes the Vercel-inspired theme implementation for Yazi file manager.

## Overview

The Yazi file manager has been configured with a custom theme that uses the Vercel design system colors from `cursed/old/ui/assets/theme.css`. The theme provides a clean, modern dark interface with carefully selected colors that match Vercel's design language.

## Color Palette

The theme uses OKLCH color space values converted to hex:

- **Background**: `#000000` (pure black - oklch(0 0 0))
- **Foreground**: `#ffffff` (pure white - oklch(1 0 0))
- **Border**: `#424242` (dark gray - oklch(0.2600 0 0))
- **Muted**: `#3a3a3a` (muted background - oklch(0.2300 0 0))
- **Muted Foreground**: `#a3a3a3` (muted text - oklch(0.7200 0 0))

### Accent Colors

- **Success/Green**: `#22c55e`
- **Error/Red**: `#ef4444`
- **Info/Blue**: `#3b82f6`
- **Warning/Orange**: `#f59e0b`
- **Purple**: `#a855f7`

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
3. **Cargo.toml** - Updated workspace to make yazi-fm the default binary

## Usage

When you run `cargo run` from the workspace root, it will now launch yazi-fm with the Vercel theme by default.

```bash
cargo run
```

The file picker will display with:
- Dark background
- Bordered interface
- Vercel-inspired color scheme
- Clean, modern aesthetics

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
