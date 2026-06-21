# Context Menu Screenshot Item State

## Source Plan
- `plans/player/context-menu-screenshot-item-work-plan.md`

## Gap Analysis
The current E2E only asserts `player.context.screenshot` is present. It never activates the context-menu screenshot item, so menu closure, screenshot result toast, repaint, and one-command behavior are unverified on this entry point. See `apps/player/src-tauri/tests/plan_ui_e2e.rs:546`.

The context item delegates to the shared screenshot action. The success-path PNG and deterministic output gaps are already captured in `plans/fix/player/45-bottom-screenshot-button-state.md`; this plan still needs context-menu-specific coverage proving the item invokes that path correctly. See `apps/player/src-tauri/src/ui/app.rs:1490`.

Boundary clicks around the screenshot row are untested, so a near-edge click could dispatch Stop, Fullscreen, or no command without being caught. See `apps/player/src-tauri/src/ui/app.rs:1680`.

## Plan Requirements Not Met
- Tests must activate screenshot through `player.context.screenshot`.
- Tests must verify the context menu closes after Screenshot activation.
- Tests must verify exactly one screenshot command runs per activation.
- Tests must assert the user-visible screenshot result from the context entry point.
- Tests must verify no neighboring context item command changes playback, fullscreen, aspect, repeat, shuffle, media path, or playlist.
- Tests must cover boundary clicks around the screenshot row.

## Required Test Shape
- Open the menu, click `player.context.screenshot`, and assert context-menu selectors are absent afterward.
- With no video frame, assert the context entry shows the same actionable failure toast as the bottom screenshot button.
- With the deterministic screenshot writer from the bottom screenshot fix, assert one PNG write and one success toast.
- Snapshot neighboring-command state before activation and assert it remains unchanged.
- Click just above and below the screenshot row and assert documented item or dismiss behavior.

## Required Changes
- Reuse the screenshot output/test hook required by `plans/fix/player/45-bottom-screenshot-button-state.md`.
- Add context-menu screenshot coverage to `plan_ui_e2e` or a focused context-menu automation test.
- Add row-boundary tests for the screenshot menu item.

## Verification
- `cargo test -p tench-player --test plan_ui_e2e context_menu_screenshot`
- `cargo test -p tench-player`
