# Seekbar Position Control State

## Source Plan
- `plans/player/seekbar-position-control-work-plan.md`

## Gap Analysis
The current E2E clicks `player.seekbar.position` once and only asserts the capture changed. It does not assert the resulting `current_time`, backend seek target, subtitle timing refresh, handle/progress position, or paused-state invariant. See `apps/player/src-tauri/tests/plan_ui_e2e.rs:153`.

The click path computes seek position from pointer x and duration, but there are no tests for 0%, 50%, 100%, or out-of-bounds clamp behavior. See `apps/player/src-tauri/src/ui/app.rs:1770`.

The drag path updates state and backend on pointer move while `dragging_seek` is true, but no automation test performs pointer down/move/up or verifies backend seek follows movement. See `apps/player/src-tauri/src/ui/app.rs:1813`.

Automation exposes the seekbar as a generic slider action without a value for current time, ratio, handle position, or dragging state, so capture-based tests cannot assert seekbar position precisely. See `apps/player/src-tauri/src/ui/app.rs:2343`.

## Plan Requirements Not Met
- Tests must verify click positions at 0%, 50%, and 100% map to clamped playback times.
- Tests must verify backend seek receives the same target as `PlayerState`.
- Tests must verify subtitle timing updates after seek.
- Tests must verify handle/progress position moves to the new time.
- Tests must verify dragging seeks on pointer movement and clears dragging state on pointer up.
- Tests must verify seeking while paused keeps playback paused.
- Automation must expose seekbar value/ratio or handle bounds for assertions.

## Required Test Shape
- Use seekbar bounds to click near start, center, and end, then assert `current_time` and backend seek target.
- Drag from one percentage to another while playing and assert multiple backend seek calls or the final target.
- Pause playback, seek, and assert `is_playing` remains false.
- Assert active subtitle text/timing changes when seeking into and out of cues.
- Assert the automation node value or handle bounds match the expected ratio after each seek.

## Required Changes
- Add seekbar value/ratio/handle metadata to automation.
- Add a backend seek spy for seekbar tests.
- Add pointer drag support to the test flow if current helpers cannot drive down/move/up.
- Extend seekbar E2E coverage for click positions, drag, paused seek, subtitle timing, backend dispatch, and clamping.

## Verification
- `cargo test -p tench-player --test plan_ui_e2e seekbar_position`
- `cargo test -p tench-player`
