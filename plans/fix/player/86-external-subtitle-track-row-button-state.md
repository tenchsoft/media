# External Subtitle Track Row Button State

## Source Plan
- `plans/player/external-subtitle-track-row-button-work-plan.md`

## Gap Analysis
The current E2E asserts `player.subtitle.external.0` is present but never activates an external subtitle row. It does not verify active-track exclusivity, subtitle overlay refresh, first/middle/last row targeting, list-mutation targeting, or unrelated playback invariants. See `apps/player/src-tauri/tests/plan_ui_e2e.rs:327`.

`SelectSubtitleTrack(idx)` does not guard invalid indices. If a stale row action carries an out-of-range index, the loop sets every external subtitle track inactive and still repaints without an error or documented no-op. See `apps/player/src-tauri/src/ui/app.rs:800`.

Selecting an external subtitle row does not call `update_subtitle_for_position`, so the overlay can remain stale until another timing update occurs. See `apps/player/src-tauri/src/ui/app.rs:800`.

Automation exposes the row action as a generic option label and does not expose selected/active state for external subtitle rows, so tests cannot assert the active row through the UI tree without reading internal state. See `apps/player/src-tauri/src/ui/app.rs:2353`.

Long external subtitle lists have no exercised scroll-aware row-selection coverage, despite the plan requiring dynamic row hit testing with current scroll offset.

## Plan Requirements Not Met
- Tests must click external subtitle rows and verify exactly one selected row is active.
- Selecting a row must refresh subtitle timing/overlay immediately.
- Invalid row indices must not deactivate all tracks silently.
- Tests must cover first, middle, and last external subtitle rows.
- Tests must repeat row selection after the subtitle track list changes and verify row indices refresh.
- Tests must verify selection does not change media path, playback time, paused state, playlist, or selected playlist index.
- Automation must expose active state for external subtitle rows.
- Tests must cover row-selection hit testing for a subtitle list larger than the visible drawer area.

## Required Test Shape
- Seed at least three external subtitle tracks, click each row, and assert only that row is active.
- For the active row, assert subtitle overlay/timing reflects the selected track immediately.
- Remove/reload subtitle tracks, click a rendered row selector again, and assert the current row is targeted.
- Exercise an invalid-index action in a unit test and assert no false deselection or success state.
- Assert the UI tree exposes selected state for `player.subtitle.external.{idx}`.

## Required Changes
- Guard `SelectSubtitleTrack` against invalid indices before mutating active flags.
- Call subtitle timing refresh after valid external row selection.
- Expose selected state for external subtitle row automation nodes.
- Add value-level external subtitle row selection coverage to `plan_ui_e2e` or a focused subtitles drawer test.
- Add deterministic subtitle-track seeding for first/middle/last and overflow cases.

## Verification
- `cargo test -p tench-player --test plan_ui_e2e external_subtitle_track_row`
- `cargo test -p tench-player`
