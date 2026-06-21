# Chapter Row Delete Button State

## Source Plan
- `plans/player/chapter-row-delete-button-work-plan.md`

## Gap Analysis
The current E2E only asserts that `player.chapter.delete.0` is present. It never clicks a delete button, so deletion toast, chapter removal, row-index targeting, repaint, and no unrelated playback mutation are unverified. See `apps/player/src-tauri/tests/plan_ui_e2e.rs:277`.

The chapter list is rendered by enumerating `state.chapters` directly, but there is no test that deletes first, middle, and last rows or that repeats deletion after an import/remove changes row indices. A stale click mapping would not be caught. See `apps/player/src-tauri/src/ui/paint_panels.rs:425`.

Long chapter lists have no exercised scroll-aware delete coverage. The plan requires dynamic row hit testing to use the rendered row index and current scroll offset, but no test creates enough chapters to verify visible-row targeting after the list exceeds the drawer height.

## Plan Requirements Not Met
- Tests must click chapter delete controls and verify the targeted chapter is removed.
- Tests must cover first, middle, and last displayed rows.
- Tests must repeat deletion after chapter-list mutation so row indices are proven to refresh.
- Tests must verify delete does not change media path, playback time, paused state, playlist, or selected playlist index.
- Tests must cover delete hit testing for a chapter list larger than the visible drawer area.

## Required Test Shape
- Seed at least three chapters, click `player.chapter.delete.0`, and assert the former first chapter is gone while the remaining order is preserved.
- Delete a middle row and then the last row, asserting the toast says chapter deletion occurred after each click.
- Import or add chapters, delete again, and assert the clicked selector targets the currently rendered row.
- Capture playback state before deletion and assert unrelated playback fields remain unchanged.
- Create an overflowing chapter list and verify delete targets the visible row under the selector, not a stale pre-scroll index.

## Required Changes
- Add chapter delete flow coverage to `plan_ui_e2e` or a focused player UI automation test.
- Add any missing test-only chapter seeding helper needed to create deterministic first/middle/last and overflowing lists.
- If chapter scrolling is implemented, route delete hit testing through the rendered row plus current scroll offset and clamp invalid indices.

## Verification
- `cargo test -p tench-player --test plan_ui_e2e chapter_row_delete`
- `cargo test -p tench-player`
