# Context Menu Dismiss Control State

## Source Plan
- `plans/player/context-menu-dismiss-control-work-plan.md`

## Gap Analysis
The automation dismiss node is a fixed top-left rectangle, not a point computed outside the currently rendered context menu. If the menu is opened near the top-left, activating `player.context.dismiss` can land inside the menu instead of outside it. See `apps/player/src-tauri/src/ui/app.rs:2263`.

The current E2E asserts the dismiss selector disappears, but it does not assert that no context item command ran while dismissing. State changes to play/pause, aspect, repeat, shuffle, file loading, or toast text would not be caught. See `apps/player/src-tauri/tests/plan_ui_e2e.rs:578`.

Clicks inside the menu rectangle but outside an item row leave the menu open because the pointer handler returns without dispatching or dismissing. This leaves top/header and bottom-padding boundary behavior ambiguous. See `apps/player/src-tauri/src/ui/app.rs:1680`.

## Plan Requirements Not Met
- The dismiss control must target a point outside the actual rendered menu geometry.
- Tests must verify dismiss closes the menu without running any item command.
- Tests must verify repeated dismiss after reopening the menu is deterministic.
- Tests must cover top/header and bottom-padding boundary clicks with documented dismiss or no-op behavior.

## Required Test Shape
- Open the context menu at several positions, activate `player.context.dismiss`, and assert `context_menu == None`.
- Snapshot playback, aspect, repeat, shuffle, media path, playlist, and toast state before dismiss and assert they are unchanged.
- Reopen and dismiss again after changing play/aspect/repeat/shuffle state.
- Click the menu header and bottom padding and assert the documented result without accidental item dispatch.

## Required Changes
- Compute the automation dismiss node from the current menu bounds and viewport so its center is outside the menu.
- Update context-menu pointer handling for non-item menu areas to dismiss or no-op explicitly and consistently.
- Extend context-menu dismiss coverage with state invariants around no command dispatch.

## Verification
- `cargo test -p tench-player --test plan_ui_e2e context_menu_dismiss`
- `cargo test -p tench-player`
