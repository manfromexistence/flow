# Font Selection Guide for Onboard

## Overview

A preview of all 113 fonts rendering "DX" has been generated. You can review the full output in `FONT_PREVIEW_CLEAN.txt`.

## Your Task

1. **Review the fonts** in `FONT_PREVIEW_CLEAN.txt`
2. **Choose your favorite 10 fonts** that look best for the DX logo
3. **Note the font names** (e.g., "Block", "Colossal", "Doom", etc.)
4. **Provide the list** of 10 font names

## What Happens Next

Once you provide the 10 font names, I will:

1. **Hardcode the ASCII art** for each of the 10 fonts directly in the code
2. **Remove the figlet font files** and compression system (no longer needed)
3. **Implement random selection** that picks one of the 10 hardcoded logos each time
4. **Simplify the codebase** by removing all font loading/decompression logic

## Benefits of Hardcoding

- **Smaller binary**: No 113 fonts, just 10 hardcoded ASCII strings
- **Faster startup**: No decompression needed
- **Simpler code**: No font loading logic
- **Guaranteed rendering**: No font parsing errors

## Sample Fonts from Preview

Here are some popular choices to consider:

- **Block**: Clean, blocky style
- **Colossal**: Large and impressive
- **Doom**: Classic retro gaming style
- **Slant**: Italic, dynamic look
- **Banner3**: Bold and clear
- **3d**: Three-dimensional effect
- **Shadow**: Shadowed text
- **Graffiti**: Street art style
- **Epic**: Grand and dramatic
- **Cyberlarge**: Futuristic tech style

## How to Choose

Look for fonts that:
- Are clearly readable
- Have good visual impact
- Fit the "DX" brand aesthetic
- Work well with rainbow colors
- Have consistent character spacing

## Next Steps

Reply with your 10 chosen font names in this format:

```
1. Block
2. Colossal
3. Doom
4. Slant
5. Banner3
6. 3d
7. Shadow
8. Graffiti
9. Epic
10. Cyberlarge
```

Then I'll hardcode them into the onboard project!
