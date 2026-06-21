# Bottom Fullscreen Button State

## Source Plan
- `plans/player/bottom-fullscreen-button-work-plan.md`

## Gap Analysis
The Fullscreen action calls the Tauri webview fullscreen API only when `app_handle` is present, but it does not store or expose fullscreen state in `PlayerState`, and it ignores API errors. In headless tests with no app handle, the button repaints without any observable fullscreen result. See `apps/player/src-tauri/src/ui/app.rs:931`.

The action requests paint immediately after calling `set_fullscreen`, not after confirming the platform fullscreen state changed. There is no follow-up layout assertion or platform-state callback to prove the video layout repainted to the new fullscreen size. See `apps/player/src-tauri/src/ui/app.rs:935`.

The existing E2E clicks `player.controls.fullscreen` only as part of a selector-presence loop and does not verify entering fullscreen, leaving fullscreen, API call count, layout repaint, or behavior while a drawer/AI panel is open. See `apps/player/src-tauri/tests/plan_ui_e2e.rs:230`.

## Plan Requirements Not Met
- Fullscreen state or API result must be observable in tests and automation.
- Fullscreen API errors must be surfaced through state, toast, or logs.
- Tests must verify entering fullscreen and returning to windowed mode.
- Tests must verify layout remains coherent when toggling fullscreen with a drawer or AI panel open.
- Tests must verify repaint happens after platform fullscreen state changes.

## Required Test Shape
- Use a fake or test fullscreen backend, click `player.controls.fullscreen`, and assert the API received `set_fullscreen(true)` and the exposed fullscreen state is true.
- Click again and assert `set_fullscreen(false)` and windowed state.
- Toggle fullscreen while a drawer or AI panel is open and assert video/control/panel bounds remain non-overlapping after the simulated resize.
- Inject a fullscreen API error and assert an actionable toast or logged state.

## Required Changes
- Add testable fullscreen state or a platform-window abstraction for Player UI tests.
- Surface fullscreen API errors.
- Expose fullscreen active state through `player.controls.fullscreen` or a dedicated automation value.
- Extend `plan_ui_e2e` or targeted tests for enter, exit, error, repaint, and side-panel layout behavior.

## Verification
- `cargo test -p tench-player --test plan_ui_e2e bottom_fullscreen`
- `cargo test -p tench-player`
