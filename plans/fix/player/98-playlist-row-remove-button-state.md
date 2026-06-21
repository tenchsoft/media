# Playlist Row Remove Button State

## Source Plan
- `plans/player/playlist-row-remove-button-work-plan.md`

## Gap Analysis
The current E2E clicks `player.playlist.remove.0` and only asserts the capture changed. It does not assert the target entry was removed, the remaining order, current-index repair, first/middle/last targeting, list-mutation targeting, or unrelated playback invariants. See `apps/player/src-tauri/tests/plan_ui_e2e.rs:259`.

Removing the current row repairs `current_playlist_index`, but there is no test defining or verifying the media/backend behavior when the currently playing entry is removed. The old media can remain loaded while the current index now points at a different entry. See `apps/player/src-tauri/src/ui/state.rs:1131`.

There is no coverage for removing rows after add/remove changes the playlist. A stale remove selector could target the wrong row without being caught. See `apps/player/src-tauri/src/ui/paint_panels.rs:232`.

Long playlists have no exercised scroll-aware remove coverage, despite the plan requiring dynamic row hit testing with current scroll offset.

## Plan Requirements Not Met
- Tests must verify the clicked playlist entry is removed.
- Tests must verify remaining playlist order after removal.
- Tests must verify current index repair for removing before, at, and after the current item.
- Tests must define and verify media/backend behavior when the current item is removed.
- Tests must cover first, middle, and last row remove buttons.
- Tests must repeat removal after playlist mutation and verify row indices refresh.
- Tests must verify removal does not change unrelated playback state beyond documented current-item behavior.
- Tests must cover remove hit testing for a playlist larger than the visible drawer area.

## Required Test Shape
- Seed at least three playlist entries, remove first/middle/last rows, and assert exact remaining paths and current index.
- Remove a row before the current item and assert current index shifts down.
- Remove the current row and assert the documented media/backend behavior.
- Add files or remove a row, then remove again and assert the currently rendered row is targeted.
- Snapshot unrelated state before removal and assert only documented playlist/current-item fields change.

## Required Changes
- Add value-level playlist remove coverage to `plan_ui_e2e` or a focused playlist drawer test.
- Document and implement current-media behavior when the current playlist item is removed.
- Add deterministic playlist seeding for first/middle/last and overflow cases.
- Expose current/active playlist row state through automation if needed.

## Verification
- `cargo test -p tench-player --test plan_ui_e2e playlist_row_remove`
- `cargo test -p tench-player`
