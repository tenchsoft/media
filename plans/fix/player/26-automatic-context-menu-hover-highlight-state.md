# Automatic Context Menu Hover Highlight State

## Source Plan
- `plans/player/automatic-context-menu-hover-highlight-work-plan.md`

## Gap Analysis
The context menu renderer draws a highlighted item when `PlayerState.context_menu_hover == Some(idx)`, but pointer movement never updates `context_menu_hover` from the current menu geometry. The pointer move path handles seek hover and drag state only, so moving over menu items cannot drive the automatic highlight. See `apps/player/src-tauri/src/ui/paint_overlays.rs:224` and `apps/player/src-tauri/src/ui/app.rs:1813`.

Closing the context menu does not explicitly clear `context_menu_hover`. This is currently masked because hover is not set, but once hover updates exist the close path must clear it to satisfy the plan's clear-on-close scenario. See `apps/player/src-tauri/src/ui/app.rs:1678`.

The existing E2E only asserts that `player.automatic.context_hover` is present while the context menu is open. It does not move over items, assert hover index changes, assert visual highlight follows the pointer, or verify the highlight clears after menu close. See `apps/player/src-tauri/tests/plan_ui_e2e.rs:548`.

## Plan Requirements Not Met
- Pointer move must update `context_menu_hover` based on current context menu item geometry.
- Closing the context menu must clear `context_menu_hover`.
- Tests must verify hover follows each context menu item and clears when the menu closes.
- Tests must verify hover geometry remains correct after resize or side-panel layout changes.
- Automation must expose the current context hover index or item id.

## Required Test Shape
- Open the context menu, move the pointer over each menu item center, and assert `context_menu_hover` matches the item index and the capture changes for the highlighted row.
- Move outside the context menu and assert `context_menu_hover == None`.
- Close the menu and assert `context_menu == None`, `context_menu_hover == None`, and `player.automatic.context_hover` is absent.
- Reopen after a resize or side-panel change and verify item hover bounds still map to the correct index.

## Required Changes
- Add context-menu hover hit testing to `PointerEvent::Move`.
- Clear `context_menu_hover` whenever the context menu closes or is dismissed.
- Expose current hover index or item id in `player.automatic.context_hover`.
- Extend `plan_ui_e2e` context-menu coverage for hover movement, visual highlight, close cleanup, and layout changes.

## Verification
- `cargo test -p tench-player --test plan_ui_e2e automatic_context_menu_hover`
- `cargo test -p tench-player`
