# Automatic A-B Loop Playback Enforcement State

## Source Plan
- `plans/player/automatic-ab-loop-playback-enforcement-work-plan.md`

## Gap Analysis
The real backend position polling path updates `current_time` from `backend.tick()` but does not check `ab_loop` or call `backend.seek(a)` when playback reaches B. A loop-back exists only in the no-backend animation simulation path, so real media playback can pass B without enforcement. See `apps/player/src-tauri/src/ui/app.rs:535` and `apps/player/src-tauri/src/ui/app.rs:1932`.

The existing E2E only asserts that the generic `player.automatic.ab_loop` status node exists and that clicking the A-B Loop control changes `PlayerState.ab_loop`. It does not play past B, verify automatic return to A, assert backend seek, or compare the highlighted seekbar range after resize or side-panel layout changes. See `apps/player/src-tauri/tests/plan_ui_e2e.rs:128` and `apps/player/src-tauri/tests/plan_ui_e2e.rs:215`.

The automatic status node is always emitted with no value describing the active loop range or enforcement state, so automation cannot assert that the automatic behavior is derived from the current canonical A-B loop state. See `apps/player/src-tauri/src/ui/app.rs:2282`.

## Plan Requirements Not Met
- Backend position updates must enforce A-B loop by seeking to A when playback reaches B.
- Tests must verify playback past B automatically returns to A without another click.
- Tests must verify the same A-B loop state behaves identically through backend and no-backend position update paths.
- Tests must verify the seekbar A-B highlight remains correct after resize or opening a side panel.
- Automation must expose enough A-B loop state to assert the active loop range or enforcement result.

## Required Test Shape
- Configure `ab_loop = Some((a, b))`, set playback active, simulate a backend tick returning a position at or beyond B, and assert `current_time == a`, `backend.seek(a)` was called, subtitles refresh, and paint is requested.
- Run the same scenario through the no-backend animation path and assert the same `current_time` result.
- Capture the seekbar before and after opening a drawer or resizing, and assert the A-B highlight selector/value and visual capture remain consistent.

## Required Changes
- Add A-B loop enforcement to the backend position update path.
- Expose A-B loop automatic status value or a dedicated range selector for automation assertions.
- Extend `plan_ui_e2e` or targeted UI tests to cover backend loop-back, no-backend loop-back, highlight persistence, and repaint behavior.

## Verification
- `cargo test -p tench-player --test plan_ui_e2e automatic_ab_loop`
- `cargo test -p tench-player`
