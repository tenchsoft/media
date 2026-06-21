# Context Menu Fullscreen Item State

## Source Plan
- `plans/player/context-menu-fullscreen-item-work-plan.md`

## Gap Analysis
The current E2E only asserts `player.context.fullscreen` is present. It never clicks the context-menu fullscreen item, so menu closure, command dispatch, repaint, and fullscreen-state changes are unverified on this entry point. See `apps/player/src-tauri/tests/plan_ui_e2e.rs:547`.

The context item delegates to the same `ClickAction::Fullscreen` path as the bottom control, whose fullscreen state and platform API result are not observable in headless tests. The shared fullscreen observability gap is already captured in `plans/fix/player/40-bottom-fullscreen-button-state.md`; this plan still needs context-menu-specific coverage. See `apps/player/src-tauri/src/ui/app.rs:1491`.

Context-menu boundary behavior around this item is untested, so a click near neighboring rows or padding could dispatch the wrong command or leave the menu open without being caught. See `apps/player/src-tauri/src/ui/app.rs:1680`.

## Plan Requirements Not Met
- Tests must activate fullscreen through `player.context.fullscreen`, not only the bottom button.
- Tests must verify the context menu closes after the fullscreen command.
- Tests must verify exactly one fullscreen toggle command is issued from one menu activation.
- Tests must verify no neighboring context item command runs.
- Tests must cover boundary clicks around the fullscreen row.

## Required Test Shape
- Open the context menu, click `player.context.fullscreen`, and assert context-menu selectors are absent afterward.
- Use the fullscreen test hook from the bottom fullscreen fix to assert one toggle request and the expected fullscreen state.
- Snapshot play/pause, aspect, repeat, shuffle, media path, playlist, and toast state and assert no neighboring command changed them.
- Click just above and below the fullscreen row and assert documented item or dismiss behavior.

## Required Changes
- Reuse or add the shared fullscreen platform abstraction required by `plans/fix/player/40-bottom-fullscreen-button-state.md`.
- Add context-menu-specific fullscreen coverage to `plan_ui_e2e` or a focused context-menu automation test.
- Add boundary tests for the fullscreen menu row.

## Verification
- `cargo test -p tench-player --test plan_ui_e2e context_menu_fullscreen`
- `cargo test -p tench-player`
