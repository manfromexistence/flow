# Theme System Documentation

## Overview

The theme system has been extracted into a dedicated module (`theme.rs`) that provides unified theming across all prompts in the onboarding experience.

## Theme Module (`src/prompts/theme.rs`)

The theme module contains:

### 1. DxTheme
Core color scheme with:
- `primary` - Cyan for main UI elements
- `success` - Green for successful operations
- `warning` - Yellow for warnings
- `error` - Red for errors
- `dim` - Dimmed text for borders and secondary elements

### 2. Symbols
Unicode symbols for consistent visual appearance:
- Step indicators (active, cancel, error, submit)
- Borders (bars, corners, boxes)
- Selection indicators (radio, checkbox)
- Password masking

### 3. Rainbow Effects
Animated rainbow coloring for special symbols using the `RainbowEffect` from effects module.

## Active Prompts (21 total)

The following prompts are kept and actively used:

1. `autocomplete` - Autocomplete with suggestions
2. `confirm` - Yes/no confirmation (used in main.rs)
3. `email` - Email input with validation
4. `input` - Basic text input
5. `matrix_select` - Skills rating matrix
6. `multiselect` - Multiple selection
7. `number` - Number input
8. `password` - Masked password input
9. `phone_input` - Phone number input
10. `progress` - Progress bar
11. `range_slider` - Range selection
12. `rating` - Star rating
13. `search_filter` - Search with filtering
14. `select` - Single selection
15. `slider` - Value slider
16. `spinner` - Loading spinner
17. `tags` - Tag input
18. `text` - Multi-line text area
19. `toggle` - Toggle switches
20. `tree_select` - Tree selection
21. `url` - URL input
22. `wizard` - Multi-step wizard

## Archived Prompts (15 total)

Moved to `src/prompts/trash/`:

1. `calendar` - Calendar view
2. `code_snippet` - Code snippet picker
3. `color_picker` - Basic color picker
4. `color_picker_advanced` - Advanced color picker
5. `credit_card` - Credit card input
6. `date_picker` - Date selection
7. `emoji_picker` - Emoji selection
8. `file_browser` - File browser
9. `json_editor` - JSON editor
10. `kanban` - Kanban board
11. `list` - List editor
12. `markdown_editor` - Markdown editor
13. `table_editor` - Table editor
14. `time_picker` - Time selection

## Usage

All prompts now import theme elements from the theme module:

```rust
use crate::prompts::theme::{THEME, SYMBOLS, rainbow_symbol};
```

The theme is globally accessible and thread-safe via `RwLock`.
