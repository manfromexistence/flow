# Vercel Theme Color Reference for dx

This document maps the Vercel design system colors from `theme.css` to the dx theme configuration.

## Dark Mode Color Palette

Based on `theme.css` `.dark` section with OKLCH color space:

### Core Colors

| Color Name | OKLCH Value | Hex Value | Usage in dx |
|------------|-------------|-----------|---------------|
| Background | oklch(0 0 0) | #000000 | App background, status bar |
| Foreground | oklch(1 0 0) | #ffffff | Text, CWD, active items |
| Border | oklch(0.2600 0 0) | #424242 | All borders (pick, confirm, spot, etc.) |
| Muted | oklch(0.2300 0 0) | #3a3a3a | Inactive tabs, alt backgrounds |
| Muted Foreground | oklch(0.7200 0 0) | #b8b8b8 | Secondary text, directories |
| Accent | oklch(0.3200 0 0) | #525252 | Permission separators |

### Semantic Colors

| Purpose | Color | Hex Value | Usage |
|---------|-------|-----------|-------|
| Success/Green | - | #22c55e | Copied markers, exec permissions, executable files |
| Error/Red | - | #ef4444 | Cut markers, write permissions, orphan files |
| Warning/Orange | - | #f59e0b | Selected markers, read permissions, images |
| Info/Purple | - | #a855f7 | Media files, run commands |

## Color Mapping

### Application (`[app]`)
- **overall.bg**: #000000 (pure black background)

### Manager (`[mgr]`)
- **cwd**: #ffffff (white, bold) - Current working directory
- **find_keyword**: #ffffff (white, bold, italic, underline)
- **find_position**: #a3a3a3 (light gray)
- **symlink_target**: #737373 (medium gray, italic)
- **marker_copied**: #22c55e (green)
- **marker_cut**: #ef4444 (red)
- **marker_marked**: #ffffff (white)
- **marker_selected**: #f59e0b (orange)
- **count_copied**: fg=#ffffff, bg=#22c55e
- **count_cut**: fg=#ffffff, bg=#ef4444
- **count_selected**: fg=#000000, bg=#f59e0b
- **border_style**: #424242 (dark gray)

### Tabs (`[tabs]`)
- **active**: fg=#000000, bg=#ffffff (inverted - black on white)
- **inactive**: fg=#ffffff, bg=#3a3a3a (white on muted)

### Mode (`[mode]`)
- **normal_main**: fg=#000000, bg=#ffffff (inverted)
- **normal_alt**: fg=#ffffff, bg=#3a3a3a
- **select_main**: fg=#000000, bg=#ffffff (inverted)
- **select_alt**: fg=#ffffff, bg=#3a3a3a
- **unset_main**: fg=#ffffff, bg=#ef4444 (white on red)
- **unset_alt**: fg=#ef4444, bg=#3a3a3a

### Status (`[status]`)
- **overall.bg**: #000000
- **perm_sep**: #525252 (accent gray)
- **perm_type**: #22c55e (green)
- **perm_read**: #f59e0b (orange)
- **perm_write**: #ef4444 (red)
- **perm_exec**: #22c55e (green)
- **progress_normal**: fg=#22c55e, bg=#000000
- **progress_error**: fg=#f59e0b, bg=#ef4444

### Which (`[which]`)
- **mask.bg**: #000000
- **cand**: #ffffff (white)
- **rest**: #737373 (medium gray)
- **desc**: #a3a3a3 (light gray)
- **separator_style**: #525252

### Confirmation (`[confirm]`)
- **border**: #424242
- **title**: #ffffff (white, bold)
- **body**: #ffffff
- **list**: #ffffff
- **btn_yes**: fg=#000000, bg=#ffffff (inverted)
- **btn_no**: fg=#ffffff, bg=#3a3a3a

### Picker (`[pick]`)
- **border**: #424242
- **active**: fg=#ffffff, bg=#3a3a3a (bold)
- **inactive**: #a3a3a3

### Input (`[input]`)
- **border**: #424242
- **title**: #ffffff (bold)
- **value**: #ffffff
- **selected**: reversed

### Completion (`[cmp]`)
- **border**: #424242
- **active**: fg=#000000, bg=#ffffff (inverted, bold)
- **inactive**: #a3a3a3

### Tasks (`[tasks]`)
- **border**: #424242
- **title**: #ffffff (bold)
- **hovered**: fg=#ffffff, bg=#3a3a3a (bold)

### Help (`[help]`)
- **on**: #ffffff (white)
- **run**: #a855f7 (purple)
- **desc**: #b8b8b8 (muted foreground)
- **hovered**: fg=#000000, bg=#ffffff (inverted, bold)
- **footer**: fg=#000000, bg=#ffffff (inverted)

### File Types (`[filetype]`)
- **Images**: #f59e0b (orange)
- **Media**: #a855f7 (purple)
- **Archives**: #ef4444 (red)
- **Documents**: #b8b8b8 (muted foreground)
- **VFS absent/stale**: #737373 (medium gray)
- **Orphan files**: bg=#ef4444 (red background)
- **Executable**: #22c55e (green)
- **Directories**: #b8b8b8 (muted foreground)

### Icons (`[icon]`)
- **Orphan**: #ef4444 (red)
- **Link**: #737373 (medium gray)
- **Block/Char/FIFO/Sock/Sticky**: #f59e0b (orange)
- **Dummy**: #ef4444 (red)
- **Directory (hovered)**: #ffffff (white)
- **Directory**: #b8b8b8 (muted foreground)
- **Executable**: #22c55e (green)
- **Regular files**: #ffffff (white)

## Design Principles

1. **High Contrast**: Pure black (#000000) background with pure white (#ffffff) foreground for maximum readability
2. **Consistent Borders**: All borders use #424242 (oklch(0.2600 0 0)) for visual consistency
3. **Semantic Colors**: 
   - Green (#22c55e) for success/executable
   - Red (#ef4444) for errors/destructive
   - Orange (#f59e0b) for warnings/selection
   - Purple (#a855f7) for special actions
4. **Muted Elements**: Use #3a3a3a for backgrounds and #b8b8b8 for secondary text
5. **Inverted Active States**: Active items use black-on-white (#000000 on #ffffff) for clear indication

## No Blue Colors

The theme intentionally avoids blue colors (#3b82f6) to maintain the Vercel design system's minimalist black, white, and gray aesthetic with semantic accent colors only where needed.

## Comparison with Original

### Removed Blue Usage:
- ❌ Select mode background (was #3b82f6)
- ❌ Directory icons (was #3b82f6)
- ❌ Document files (was #3b82f6)
- ❌ Help "on" state (was #3b82f6)
- ❌ Permission exec (was #3b82f6)

### Replaced With:
- ✅ Select mode: White on black (inverted)
- ✅ Directories: Muted foreground (#b8b8b8)
- ✅ Documents: Muted foreground (#b8b8b8)
- ✅ Help "on": White (#ffffff)
- ✅ Permission exec: Green (#22c55e)

This creates a cohesive, professional appearance that matches the Vercel design system's emphasis on clarity and minimalism.
