# Chapter Row Jump Control State

## Source Plan
- `plans/player/chapter-row-jump-control-work-plan.md`

## Gap Analysis
The current E2E clicks `player.chapter.row.1` and only asserts the capture changed. It does not assert `current_time`, toast text, selected chapter title, backend seek behavior, or unrelated playback fields. See `apps/player/src-tauri/tests/plan_ui_e2e.rs:305`.

The handler seeks the backend only when a real backend exists, but there is no test double or observable automation state proving the backend receives the same chapter time as `PlayerState`. See `apps/player/src-tauri/src/ui/app.rs:1033`.

There is no coverage for first, middle, and last chapter rows, nor for row targeting after the chapter list changes through import/add/delete. Stale row indices would not be caught. See `apps/player/src-tauri/src/ui/paint_panels.rs:425`.

Long chapter lists have no exercised scroll-aware jump coverage. The plan requires dynamic row hit testing to use the rendered row index and current scroll offset, but no test creates enough chapters to verify visible-row targeting after overflow.

## Plan Requirements Not Met
- Tests must verify jumping sets playback time to the clicked chapter time.
- Tests must verify the toast names the clicked chapter.
- Tests must prove the backend seek target matches the `PlayerState` seek target.
- Tests must cover first, middle, and last displayed rows.
- Tests must repeat jump after chapter-list mutation and verify row indices refresh.
- Tests must verify jump does not change media path, playlist, selected playlist index, or paused state.
- Tests must cover jump hit testing for a chapter list larger than the visible drawer area.

## Required Test Shape
- Seed chapters with distinct times/titles, click each target row, and assert `current_time` and toast match that row.
- Attach a test backend or backend-event spy and assert the seek request uses the same time.
- Add/import/delete chapters, click a row again, and assert the action targets the currently rendered row.
- Capture unrelated playback state before each jump and assert only expected seek fields changed.
- Create an overflowing chapter list and verify a visible row click maps to the rendered chapter under the selector.

## Required Changes
- Add value-level chapter jump coverage to `plan_ui_e2e` or a focused player UI automation test.
- Add a backend seek test hook if no existing backend spy can observe chapter jump seeks.
- Add deterministic chapter seeding for first/middle/last and overflow cases.

## Verification
- `cargo test -p tench-player --test plan_ui_e2e chapter_row_jump`
- `cargo test -p tench-player`
