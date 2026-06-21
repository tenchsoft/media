# Context Menu Shuffle Item State

## Source Plan
- `plans/player/context-menu-shuffle-item-work-plan.md`

## Gap Analysis
The current E2E clicks `player.context.shuffle` but does not assert `shuffle_enabled`, toast text, menu closure, dynamic menu label, or one-command behavior. See `apps/player/src-tauri/tests/plan_ui_e2e.rs:575`.

The context item uses the shared shuffle toggle path, whose control-label and playback-order gaps are already captured in `plans/fix/player/48-bottom-shuffle-toggle-button-state.md`. This plan still needs coverage proving the context-menu entry point drives that path correctly. See `apps/player/src-tauri/src/ui/app.rs:1502`.

Dynamic Shuffle labels are not value-tested. The menu label is computed from `state.shuffle_enabled`, but tests only assert selector presence and would not catch a stale `Shuffle: On/Off` label after toggling. See `apps/player/src-tauri/src/ui/app.rs:1738`.

## Plan Requirements Not Met
- Tests must verify context-menu Shuffle toggles shuffle exactly once per activation.
- Tests must verify the context menu closes after Shuffle activation.
- Tests must verify the Shuffle menu label matches current shuffle state before and after toggling.
- Tests must verify the shuffle toast/control label updates through the context entry point.
- Tests must verify no neighboring context item command changes play/pause, aspect, repeat, media path, playlist, screenshot, or fullscreen state.

## Required Test Shape
- Open the menu, assert the Shuffle label matches current state, click `player.context.shuffle`, and assert `shuffle_enabled` and toast.
- Reopen the menu after each toggle and assert the label matches `state.shuffle_enabled`.
- Toggle on and off through the context menu and assert one state transition per click.
- Assert context-menu selectors are absent after each activation.
- Snapshot unrelated context-controlled state and assert it remains unchanged.

## Required Changes
- Add context-menu Shuffle coverage to `plan_ui_e2e` or a focused context-menu automation test.
- Reuse shuffle label/state automation exposure from `plans/fix/player/48-bottom-shuffle-toggle-button-state.md`.
- Add menu closure and no-neighbor-command assertions for the Shuffle item.

## Verification
- `cargo test -p tench-player --test plan_ui_e2e context_menu_shuffle`
- `cargo test -p tench-player`
