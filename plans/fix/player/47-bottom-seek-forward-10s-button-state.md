# Bottom Seek Forward 10 Seconds Button State

## Source Plan
- `plans/player/bottom-seek-forward-10s-button-work-plan.md`

## Gap Analysis
The existing E2E clicks `player.controls.seek_forward_10` only as part of a selector-presence loop. It does not verify `current_time` moved forward by 10 seconds, clamped at duration, updated subtitles, moved the seekbar handle, or called `backend.seek`. See `apps/player/src-tauri/tests/plan_ui_e2e.rs:230`.

State unit coverage verifies broad seek clamping with a large positive target, but it does not cover the bottom button dispatch, the exact 10-second delta, backend synchronization, playing-state preservation, or paused subtitle/frame preview behavior. See `apps/player/src-tauri/src/ui/state.rs:1473`.

Automation does not expose the post-click seek time or seekbar progress through the button, so tests need a separate deterministic progress value or state assertion to verify the immediate result. See `apps/player/src-tauri/src/ui/app.rs:2360`.

## Plan Requirements Not Met
- Tests must verify clicking Seek Forward adds exactly 10 seconds.
- Tests must verify seeking near the end clamps to duration.
- Tests must verify subtitles update after the seek.
- Tests must verify backend `seek(current_time)` is called with the clamped time.
- Tests must verify playback continues when playing and remains paused when paused.
- Automation must expose or make assertable the resulting playback position/progress.

## Required Test Shape
- Set `current_time = 42.0`, click `player.controls.seek_forward_10`, and assert `current_time == 52.0`, seekbar progress changed, and backend `seek(52.0)` was called.
- Set `current_time = duration - 4.0`, click, and assert `current_time == duration` plus backend seek with the clamped duration.
- Configure subtitle cues around the target time and assert `subtitle_text` updates after the click.
- Repeat in playing and paused states and assert `is_playing` is preserved.

## Required Changes
- Add fake backend seek-call assertions.
- Expose playback position/progress through automation, likely via `player.automatic.playback_progress`.
- Extend `plan_ui_e2e` Seek Forward coverage for exact delta, clamp, subtitle update, backend seek, playing, and paused states.

## Verification
- `cargo test -p tench-player --test plan_ui_e2e bottom_seek_forward_10s`
- `cargo test -p tench-player`
