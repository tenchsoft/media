# Context Menu Repeat Item State

## Source Plan
- `plans/player/context-menu-repeat-item-work-plan.md`

## Gap Analysis
The current E2E clicks `player.context.repeat` but does not assert the repeat mode, toast, menu closure, label update, or one-command behavior. See `apps/player/src-tauri/tests/plan_ui_e2e.rs:573`.

The context item uses the shared repeat-cycle state path, whose full cycle and control-label gaps are already captured in `plans/fix/player/44-bottom-repeat-mode-button-state.md`. This plan still needs coverage proving the context-menu entry point drives that path correctly. See `apps/player/src-tauri/src/ui/app.rs:1498`.

Dynamic Repeat labels are not value-tested. The menu label is computed from `state.repeat_mode`, but tests only assert selector presence and would not catch a stale `Repeat: ...` label after cycling. See `apps/player/src-tauri/src/ui/app.rs:1734`.

## Plan Requirements Not Met
- Tests must verify context-menu Repeat cycles repeat mode exactly once per activation.
- Tests must verify the context menu closes after Repeat activation.
- Tests must verify the Repeat menu label matches current repeat mode before and after cycling.
- Tests must verify the repeat toast/control label updates through the context entry point.
- Tests must verify no neighboring context item command changes play/pause, aspect, shuffle, media path, or playlist.

## Required Test Shape
- Open the menu, assert the Repeat label matches the current mode, click `player.context.repeat`, and assert the next mode and toast.
- Reopen the menu after each cycle and assert the label matches `state.repeat_mode.label()`.
- Repeat through the full cycle and assert one state transition per click.
- Assert context-menu selectors are absent after each activation.
- Snapshot unrelated context-controlled state and assert it remains unchanged.

## Required Changes
- Add context-menu Repeat coverage to `plan_ui_e2e` or a focused context-menu automation test.
- Reuse the repeat-label automation exposure from `plans/fix/player/44-bottom-repeat-mode-button-state.md`.
- Add menu closure and no-neighbor-command assertions for the Repeat item.

## Verification
- `cargo test -p tench-player --test plan_ui_e2e context_menu_repeat`
- `cargo test -p tench-player`
