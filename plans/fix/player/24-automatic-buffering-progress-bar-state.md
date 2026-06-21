# Automatic Buffering Progress Bar State

## Source Plan
- `plans/player/automatic-buffering-progress-bar-work-plan.md`

## Gap Analysis
The existing E2E only asserts that the generic `player.automatic.buffering_progress` status node exists. It does not inject `MediaEvent::Buffering`, verify `buffering_percent`, assert backend pause/play coordination, or inspect the buffered seekbar bar. See `apps/player/src-tauri/tests/plan_ui_e2e.rs:128`.

The automatic buffering status node is always emitted with no value for active buffering state or percent, so automation cannot assert that the UI is derived from canonical `PlayerState.buffering_percent` and `PlayerState.is_buffering`. See `apps/player/src-tauri/src/ui/app.rs:2288`.

There is no coverage for resizing or opening side panels after buffering changes. The buffer bar width is computed from the current seekbar geometry, but tests do not verify that the bar remains correct when the media surface and controls resize. See `apps/player/src-tauri/src/ui/paint_controls.rs:151`.

## Plan Requirements Not Met
- Tests must verify `MediaEvent::Buffering` updates buffering state and the visible buffer bar.
- Tests must verify buffering below 100 pauses playback and buffering at 100 resumes playback.
- Tests must verify the buffering bar remains correct after resize or side-panel layout changes.
- Automation must expose active buffering state or percent for deterministic assertions.

## Required Test Shape
- Inject `MediaEvent::Buffering(40)` while playing and assert `buffering_percent == 40`, `is_buffering == true`, backend pause was called, and the seekbar capture shows a partial buffer bar.
- Inject `MediaEvent::Buffering(100)` after a buffering state and assert `is_buffering == false`, backend play was called, and the buffer bar is absent.
- Open a drawer or AI panel and capture again, asserting the buffering automation value and visual bar align with the resized controls.

## Required Changes
- Add test hooks or a fake backend to inject buffering events and observe pause/play calls.
- Expose buffering percent or active state in `player.automatic.buffering_progress`.
- Extend `plan_ui_e2e` or targeted UI tests for buffering event state, pause/resume coordination, visual bar rendering, and layout changes.

## Verification
- `cargo test -p tench-player --test plan_ui_e2e automatic_buffering_progress`
- `cargo test -p tench-player`
