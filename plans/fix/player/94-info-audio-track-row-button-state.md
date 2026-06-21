# Info Audio Track Row Button State

## Source Plan
- `plans/player/info-audio-track-row-button-work-plan.md`

## Gap Analysis
The current E2E clicks `player.info.audio_track.0` but does not assert backend selection, toast text, active row state, repaint, or unrelated playback invariants. See `apps/player/src-tauri/tests/plan_ui_e2e.rs:460`.

`SelectAudioTrack(idx)` does not guard invalid indices and always shows a success-style toast. A stale row action can send an out-of-range track index to the backend or show `Audio track N selected` without a valid track. See `apps/player/src-tauri/src/ui/app.rs:806`.

The Info drawer renders audio tracks directly from backend state when a backend exists, but there is no test double proving first, middle, and last rows target the intended stream or that row hit testing refreshes when the backend track count changes. See `apps/player/src-tauri/src/ui/paint_panels.rs:819`.

Automation exposes audio track rows as generic options and does not expose active/selected state for assertions through the UI tree. See `apps/player/src-tauri/src/ui/app.rs:2354`.

Long audio-track lists have no exercised scroll-aware row-selection coverage, despite the plan requiring dynamic row hit testing with current scroll offset.

## Plan Requirements Not Met
- Tests must verify backend audio track selection receives the selected index.
- Invalid audio track indices must not be sent to the backend or reported as success.
- Tests must verify the toast names the selected track.
- Tests must verify active row highlight/selected state changes.
- Tests must cover first, middle, and last audio track rows.
- Tests must repeat selection after the available track list changes and verify row indices refresh.
- Tests must verify selection does not change media path, playback time, paused state, playlist, or selected playlist index.
- Automation must expose selected state for audio track rows.

## Required Test Shape
- Use a backend spy with at least three audio streams, click first/middle/last rows, and assert the backend index and active UI state.
- Inject a backend track-count change, repaint, click a rendered row, and assert the current row index is targeted.
- Exercise an invalid-index action in a unit test and assert a documented no-op or error toast without backend dispatch.
- Snapshot unrelated player state before row selection and assert it remains unchanged.

## Required Changes
- Add bounds checks for `SelectAudioTrack`.
- Add a testable audio-track backend abstraction or command spy.
- Surface backend selection failures.
- Expose active state for `player.info.audio_track.{idx}` automation nodes.
- Add value-level Info drawer audio-track coverage to `plan_ui_e2e` or a focused Info drawer test.

## Verification
- `cargo test -p tench-player --test plan_ui_e2e info_audio_track_row`
- `cargo test -p tench-player`
