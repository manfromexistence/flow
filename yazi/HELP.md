# Help Needed

> This file is auto-generated when a task fails 3 consecutive attempts.
> A more capable AI or a human should review and resolve these blockers.

---

## Blocker: Left Panel Mouse Wheel Scrolling in Yazi File Picker

**Date:** 2026-03-22

**Task Description:**
Add mouse wheel scroll support to the left panel (parent folder view) in yazi file picker, matching the behavior of the center and right panels.

**Attempt 1:**
- Approach: Added `Parent:scroll()` function in `parent.lua` that emits "parent-arrow" event
- Created `parent_arrow.rs` actor to handle the event and call `arrow()` on parent folder
- Registered the actor in `mgr/mod.rs`
- Result: No scrolling occurred in left panel
- Error: Events not reaching the Parent component

**Attempt 2:**
- Approach: Modified `Tab:scroll()` in `tab.lua` to route scroll events to child components (Parent, Current, Preview)
- Used `ya.child_at()` to find which component is under the mouse cursor
- Result: Still no scrolling in left panel
- Error: Parent folder not updating visually even though arrow() was called

**Attempt 3:**
- Approach: Added debug logging to verify if parent-arrow event was being triggered
- Simplified parent_arrow implementation to just call `parent.arrow()` and render
- Result: Unable to verify if events were reaching the Rust code
- Error: Left panel remains unresponsive to mouse wheel

**Root Cause Analysis:**
The left panel (parent folder) in yazi has a fundamentally different design than the center and right panels:

1. **Window Calculation**: The parent folder's window is calculated to show files around the current directory's position in the parent folder, not based on an independent scrollable offset.

2. **Event Routing**: The `Tab` component's scroll/touch/click methods are empty stubs, so events from the Root component don't get routed to child components (Parent, Current, Preview).

3. **Parent Folder Purpose**: The parent folder is designed to provide context (show surrounding files), not to be independently scrollable. It's meant to show where you are in the parent directory, not to browse the parent directory.

4. **Lua-Rust Boundary**: Even when the parent folder's offset is modified via `arrow()`, the visual update may not occur because:
   - The parent folder window might be recalculated on each render
   - The parent folder's offset might be reset based on the current directory's position
   - The Lives/UserData caching might not reflect the updated offset

**Suggested Solutions:**

1. **Modify Yazi Core Design** (Complex):
   - Change how the parent folder window is calculated to use an independent offset
   - Ensure the parent folder's offset persists across renders
   - Update the Tab component to properly route mouse events to children
   - This requires deep understanding of yazi's architecture and may break existing behavior

2. **Alternative UI Pattern** (Simpler):
   - Keep the parent panel as context-only (non-scrollable)
   - Add keyboard shortcuts to navigate the parent folder (e.g., Shift+Up/Down)
   - Document that the parent panel is for context, not browsing

3. **Hybrid Approach** (Medium):
   - Make the parent panel scrollable only when explicitly focused (e.g., via Tab key)
   - When focused, temporarily change its behavior to be independently scrollable
   - When unfocused, revert to context-showing behavior

**Environment Info:**
- Language/Runtime: Rust 1.82 (edition 2024), Lua (via mlua)
- OS: Windows (bash shell)
- Key Dependencies: 
  - yazi-core (file manager core logic)
  - yazi-plugin (Lua plugin system)
  - yazi-actor (event handling)
  - ratatui (TUI rendering)

**Additional Context:**
- The center panel (Current) and right panel (Preview) both have working mouse wheel scroll
- The center panel uses `ya.emit("arrow", { step })` which works correctly
- The right panel uses `ya.emit("seek", { step })` for preview scrolling
- The parent panel's `Parent:scroll()` function is currently an empty stub
- Mouse events are enabled in config: `mouse_events = [ "click", "scroll" ]`

**Files Modified (Need Revert):**
- `yazi/yazi-plugin/preset/components/parent.lua` - Added scroll function (reverted)
- `yazi/yazi-plugin/preset/components/tab.lua` - Added event routing (reverted)
- `yazi/yazi-actor/src/mgr/parent_arrow.rs` - Created new actor (deleted)
- `yazi/yazi-actor/src/mgr/mod.rs` - Registered parent_arrow (reverted)

**Recommendation:**
This issue requires either:
1. A yazi maintainer who understands the parent folder's intended design
2. Significant refactoring of yazi's core folder management system
3. Acceptance that the parent panel is context-only and not meant to scroll

For now, the left panel remains click-only (clicking on items navigates to them) without mouse wheel scroll support.
