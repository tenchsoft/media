# Bottom Seek Backward 10 Seconds Button State

## Source Plan
- `plans/player/bottom-seek-backward-10s-button-work-plan.md`

## Gap Analysis
The existing E2E clicks `player.controls.seek_back_10` only as part of a selector-presence loop. It does not verify `current_time` moved back by 10 seconds, clamped at zero, updated subtitles, moved the seekbar handle, or called `backend.seek`. See `apps/player/src-tauri/tests/plan_ui_e2e.rs:230`.

State unit coverage verifies broad seek clamping with a large negative delta, but it does not cover the bottom button dispatch, the exact 10-second delta, backend synchronization, playing-state preservation, or paused subtitle/frame preview behavior. See `apps/player/src-tauri/src/ui/state.rs:1473`.

Automation does not expose the post-click seek time or seekbar progress through the button, so tests need a separate deterministic progress value or state assertion to verify the immediate result. See `apps/player/src-tauri/src/ui/app.rs:2357`.

## Plan Requirements Not Met
- Tests must verify clicking Seek Backward subtracts exactly 10 seconds.
- Tests must verify seeking near the beginning clamps to 0.
- Tests must verify subtitles update after the seek.
- Tests must verify backend `seek(current_time)` is called with the clamped time.
- Tests must verify playback continues when playing and remains paused when paused.
- Automation must expose or make assertable the resulting playback position/progress.

## Required Test Shape
- Set `current_time = 42.0`, click `player.controls.seek_back_10`, and assert `current_time == 32.0`, seekbar progress changed, and backend `seek(32.0)` was called.
- Set `current_time = 4.0`, click, and assert `current_time == 0.0` plus backend `seek(0.0)`.
- Configure subtitle cues around the target time and assert `subtitle_text` updates after the click.
- Repeat in playing and paused states and assert `is_playing` is preserved.

## Required Changes
- Add fake backend seek-call assertions.
- Expose playback position/progress through automation, likely via `player.automatic.playback_progress`.
- Extend `plan_ui_e2e` Seek Backward coverage for exact delta, clamp, subtitle update, backend seek, playing, and paused states.

## Verification
- `cargo test -p tench-player --test plan_ui_e2e bottom_seek_backward_10s`
- `cargo test -p tench-player`
