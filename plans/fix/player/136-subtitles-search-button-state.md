# Subtitles Search Button State

## Source Plan
- `plans/player/subtitles-search-button-work-plan.md`

## Gap Analysis
The drawer Search button dispatches `ShowSubtitleSearch`, which opens the modal, focuses input, and clears text, but it does not clear `subtitle_search_results`, `subtitle_search_current`, or `subtitle_search_result_time`. Previous search results can remain stale after reopening. See `apps/player/src-tauri/src/ui/app.rs:1314`.

There is a separate `OpenSubtitleSearch` action that clears results, but the drawer button does not use that path. See `apps/player/src-tauri/src/ui/app.rs:1087` and `apps/player/src-tauri/src/ui/paint_panels.rs:521`.

The current E2E clicks `player.subtitles.search` and asserts modal controls are present, but it does not assert search focus, cleared text/results, result-time cleanup, or unrelated playback invariants. See `apps/player/src-tauri/tests/plan_ui_e2e.rs:366`.

Automation does not expose the search input value/focus or result state, so the plan's focus and clear requirements cannot be asserted through the UI tree.

## Plan Requirements Not Met
- Opening search from the Subtitles drawer must clear previous search results and current result state.
- Tests must verify search input focus after opening.
- Tests must verify search text, results, current result, and result time are cleared.
- Tests must verify repeated open after prior search state is deterministic.
- Tests must verify opening search does not change media path, playback time, paused state, playlist, or selected playlist index.
- Automation must expose input focus/value and search result state for assertions.

## Required Test Shape
- Seed stale search text/results/current/result time, click `player.subtitles.search`, and assert all documented search state is reset.
- Assert `subtitle_search_open == true` and `subtitle_search_focused == true`.
- Assert modal input value is empty through automation.
- Snapshot unrelated playback state before opening search and assert it remains unchanged.

## Required Changes
- Consolidate `OpenSubtitleSearch` and `ShowSubtitleSearch` or make both clear the same state.
- Expose subtitle search input focus/value and result state through automation.
- Extend Subtitles Search button coverage with state reset, focus, repeat open, and unrelated-state assertions.

## Verification
- `cargo test -p tench-player --test plan_ui_e2e subtitles_search_button`
- `cargo test -p tench-player`
