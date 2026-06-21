# Automatic Click Region Refresh State

## Source Plan
- `plans/player/automatic-click-region-refresh-work-plan.md`

## Gap Analysis
The paint path clears click regions before registering the current frame, and `PlayerState` has a unit test for clearing the vector. What is missing is E2E coverage proving that stale regions from closed menus, drawers, and modals cannot fire after the UI repaints. See `apps/player/src-tauri/src/ui/app.rs:1538` and `apps/player/src-tauri/src/ui/state.rs:1517`.

The current E2E dismisses the context menu and asserts the dismiss selector disappears, but it does not click where old context menu items used to be to prove stale actions are gone. It also does not cover stale regions after closing drawers or modals. See `apps/player/src-tauri/tests/plan_ui_e2e.rs:572`.

The generic `player.automatic.click_region_refresh` node is always emitted without a value such as registered region count or frame generation, so automation cannot assert that the click-region map was rebuilt for the current frame after layout changes. See `apps/player/src-tauri/src/ui/app.rs:2282`.

## Plan Requirements Not Met
- Tests must verify stale menu click regions do not fire after the menu closes.
- Tests must verify stale drawer and modal click regions do not fire after those surfaces close.
- Tests must verify resized or side-panel layouts rebuild click regions before the next click.
- Automation must expose enough click-region refresh state to assert the current frame map was rebuilt.

## Required Test Shape
- Open a context menu, record an item bounds, dismiss the menu, click the old item center, and assert no context action or state mutation occurs.
- Open a drawer and an Add Chapter modal, close them, click previous control bounds, and assert no drawer/modal action fires.
- Resize or open/close a side panel, then assert current control bounds work and previous moved bounds do not.
- Assert `player.automatic.click_region_refresh` reports a changed region count or generation after repaint.

## Required Changes
- Expose click-region count or refresh generation in the automatic click-region automation node.
- Extend `plan_ui_e2e` with stale menu, drawer, modal, and layout-change click-region assertions.

## Verification
- `cargo test -p tench-player --test plan_ui_e2e automatic_click_region_refresh`
- `cargo test -p tench-player`
