# Automatic Toast Lifecycle State

## Source Plan
- `plans/player/automatic-toast-lifecycle-work-plan.md`

## Gap Analysis
Toast timeout is checked only during `paint_toast()`. There is no timer or animation-frame request while a toast is visible, so a toast can remain on screen past the timeout until some unrelated event causes another paint. See `apps/player/src-tauri/src/ui/paint_overlays.rs:112`.

The toast timer resets only when the toast string differs from `last_toast`. Re-triggering the same toast message, such as repeated screenshot failure, does not reset the timer because `show_toast()` stores only the message and no generation/timestamp. See `apps/player/src-tauri/src/ui/app.rs:1644` and `apps/player/src-tauri/src/ui/state.rs:1428`.

The existing E2E verifies that some actions create toast text or that the generic `player.automatic.toast_lifecycle` node exists. It does not wait past the timeout, assert automatic disappearance, verify repeated same-message reset, or cover repeat/shuffle/subtitle offset toast expiry. See `apps/player/src-tauri/tests/plan_ui_e2e.rs:219`.

The automatic toast lifecycle node is always emitted with no value for current toast text, remaining time, or visible/expired state. See `apps/player/src-tauri/src/ui/app.rs:2300`.

## Plan Requirements Not Met
- Toast lifecycle must schedule repaint or animation so expiry happens without unrelated input.
- Re-triggering the same toast message must reset the timeout.
- Tests must verify screenshot, repeat, shuffle, and subtitle offset toasts expire automatically.
- Tests must verify toast text remains correctly placed after resize or side-panel layout changes.
- Automation must expose current toast text or visible/expired lifecycle state.

## Required Test Shape
- Trigger a screenshot toast, advance fake time past 3000ms without user input, and assert toast state and visible selector clear.
- Trigger the same toast message twice and assert the timeout resets from the second trigger.
- Trigger repeat, shuffle, and subtitle offset toasts and assert each appears with expected text then expires.
- Resize or open a side panel while a toast is visible and assert the toast remains correctly positioned until expiry.

## Required Changes
- Add toast generation/timestamp state or reset logic independent of message equality.
- Request animation/timer repaint while a toast is visible until it expires.
- Expose toast text and lifecycle state through `player.automatic.toast_lifecycle`.
- Extend `plan_ui_e2e` or targeted UI tests for timeout, repeated same-message reset, multiple toast sources, and layout changes.

## Verification
- `cargo test -p tench-player --test plan_ui_e2e automatic_toast_lifecycle`
- `cargo test -p tench-player`
