# Built-in Subtitle Track Row Button State

## Source Plan
- `plans/player/builtin-subtitle-track-row-button-work-plan.md`

## Gap Analysis
`SelectBuiltinSubtitleTrack(idx)` does not guard `idx` against `n_builtin_subtitle_tracks`. A stale or invalid row action can set `active_builtin_subtitle_track` to an out-of-range value and call the backend with that invalid index. See `apps/player/src-tauri/src/ui/app.rs:988`.

The existing E2E clicks only `player.subtitle.builtin.0` and asserts the state value becomes 0. It does not cover first/middle/last rows, backend `set_subtitle_track` calls, toast text, active row styling, invalid index guards, dynamic list changes, scroll, or unrelated playback state. See `apps/player/src-tauri/tests/plan_ui_e2e.rs:356`.

Built-in subtitle labels are fixture-only and are not refreshed from backend metadata, so dynamic row label and hit-testing behavior after media changes is not covered. See `apps/player/src-tauri/src/ui/app.rs:548` and `apps/player/src-tauri/src/ui/state.rs:624`.

Automation exposes built-in rows as generic options without selected/active value, so tests cannot assert row highlighting from the UI tree. See `apps/player/src-tauri/src/ui/app.rs:2423`.

## Plan Requirements Not Met
- Built-in subtitle track selection must guard invalid/out-of-range indices.
- Tests must verify first, middle, and last built-in rows select the correct index.
- Tests must verify backend track selection and toast text.
- Tests must verify active row highlighting.
- Tests must verify dynamic row hit testing after list changes or scroll.
- Automation must expose selected/active state for built-in subtitle rows.

## Required Test Shape
- Configure three built-in tracks, click row 0, row 1, and row 2, and assert `active_builtin_subtitle_track`, backend calls, toast text, and selected row value after each click.
- Dispatch or simulate an invalid/stale row index and assert no state or backend mutation occurs.
- Change built-in track count/labels while the drawer is open, repaint, and assert row selectors and hit targets match the displayed rows.
- Verify playback state is unchanged by row selection.

## Required Changes
- Add index bounds checks for `SelectBuiltinSubtitleTrack`.
- Populate and refresh built-in subtitle labels from backend metadata.
- Expose selected state for `player.subtitle.builtin.{idx}` rows.
- Extend `plan_ui_e2e` Built-in Subtitle Track coverage for row indices, backend calls, toast, active styling, invalid indices, dynamic rows, and no playback side effects.

## Verification
- `cargo test -p tench-player --test plan_ui_e2e builtin_subtitle_track_row`
- `cargo test -p tench-player`
