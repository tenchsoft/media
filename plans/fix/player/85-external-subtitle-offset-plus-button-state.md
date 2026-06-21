# External Subtitle Offset Plus Button State

## Source Plan
- `plans/player/external-subtitle-offset-plus-button-work-plan.md`

## Gap Analysis
The current E2E clicks `player.subtitle.external.0.offset_plus` only after a prior minus click and asserts the offset returns to the original value. It does not assert the toast text, visible row label, subtitle timing refresh, first/middle/last row targeting, list-mutation targeting, or unrelated playback invariants. See `apps/player/src-tauri/tests/plan_ui_e2e.rs:347`.

`SubtitleOffsetForTrack` shows a success-style toast even when the track index is invalid. A stale row action can produce `Track N offset: 0ms` without mutating any track. See `apps/player/src-tauri/src/ui/app.rs:1359`.

External subtitle row offset controls are rendered from the current cloned track list, but no test changes the subtitle track list and then verifies the plus selector maps to the newly rendered row index. See `apps/player/src-tauri/src/ui/paint_panels.rs:555`.

Long external subtitle lists have no exercised scroll-aware offset coverage, despite the plan requiring dynamic row hit testing with current scroll offset.

## Plan Requirements Not Met
- Tests must verify the plus action toast shows the new offset.
- Tests must verify subtitle timing/overlay refreshes after the offset changes.
- Invalid track indices must not show a false success toast.
- Tests must cover first, middle, and last external subtitle rows.
- Tests must repeat plus after the subtitle track list changes and verify row indices refresh.
- Tests must verify plus does not change media path, playback time, paused state, playlist, or selected playlist index.
- Tests must cover offset-plus hit testing for a subtitle list larger than the visible drawer area.

## Required Test Shape
- Seed at least three external subtitle tracks, click each row's plus button, and assert only that row offset increases by 100ms.
- Assert the toast text includes the targeted row and new offset.
- For the active track, assert subtitle timing output changes according to the new offset.
- Remove/reload subtitle tracks, click a rendered plus selector again, and assert the current row is targeted.
- Exercise an invalid-index action in a unit test and assert an error/no-op result without false success.

## Required Changes
- Guard `SubtitleOffsetForTrack` so invalid indices produce a documented no-op or error toast.
- Add value-level external subtitle offset-plus coverage to `plan_ui_e2e` or a focused subtitles drawer test.
- Add deterministic subtitle-track seeding for first/middle/last and overflow cases.
- Expose enough subtitle timing state through automation to assert refresh after offset changes.

## Verification
- `cargo test -p tench-player --test plan_ui_e2e external_subtitle_offset_plus`
- `cargo test -p tench-player`
