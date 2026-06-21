# Automatic Playback Position And Progress State

## Source Plan
- `plans/player/automatic-playback-position-progress-work-plan.md`

## Gap Analysis
The existing E2E clicks Play/Pause, sends one animation frame, and only asserts that the generic `player.automatic.playback_progress` node is present. It does not verify `current_time` changes, time text changes, seekbar fill/handle moves, or repeated automatic frames advance playback without manual refresh. See `apps/player/src-tauri/tests/plan_ui_e2e.rs:146`.

Backend `Position` events and `backend.tick()` update `current_time`, but there is no fake-backend or event-injection test that verifies those paths update duration, subtitles, and visible progress consistently. See `apps/player/src-tauri/src/ui/app.rs:431` and `apps/player/src-tauri/src/ui/app.rs:535`.

The automatic playback progress status node is always emitted with no value for current time, duration, progress ratio, or update source. Automation cannot assert that the time display and seekbar are derived from canonical `PlayerState`. See `apps/player/src-tauri/src/ui/app.rs:2282`.

## Plan Requirements Not Met
- Tests must verify automatic playback advances `current_time` over multiple frames.
- Tests must verify time text and seekbar progress visually update with backend position or animation updates.
- Tests must verify backend `Position` events and `backend.tick()` produce the same canonical progress state.
- Tests must verify progress rendering remains correct after resize or side-panel layout changes.
- Automation must expose current time, duration, or progress ratio for deterministic assertions.

## Required Test Shape
- Start playback, send multiple animation frames, and assert `current_time` increases, time text changes, and capture/progress geometry changes.
- Inject backend `Position` and `Duration` events or use a fake backend tick, then assert `PlayerState.current_time`, `duration`, subtitle refresh, and seekbar progress match.
- Open a drawer or AI panel after a position update and assert the same progress ratio is rendered in the resized controls.

## Required Changes
- Expose current time/duration/progress ratio through `player.automatic.playback_progress`.
- Add fake backend or event-injection hooks for deterministic position and tick tests.
- Extend `plan_ui_e2e` or targeted UI tests for multi-frame advancement, backend position events, seekbar/time visual changes, and layout changes.

## Verification
- `cargo test -p tench-player --test plan_ui_e2e automatic_playback_progress`
- `cargo test -p tench-player`
