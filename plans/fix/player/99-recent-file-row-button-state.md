# Recent File Row Button State

## Source Plan
- `plans/player/recent-file-row-button-work-plan.md`

## Gap Analysis
The current E2E clicks `player.recent.row.0` and only asserts the capture changed. It does not assert the selected recent file path/title, playback state, backend load/play dispatch, first/middle/last row targeting, list-mutation targeting, or unrelated state invariants. See `apps/player/src-tauri/tests/plan_ui_e2e.rs:266`.

When no backend is present, `load_media_for_action` uses `PlayerState::open_media`, which leaves `is_playing = false`. The plan expects the selected recent file to begin playback, so the headless fallback does not mirror the real backend path. See `apps/player/src-tauri/src/ui/app.rs:313` and `apps/player/src-tauri/src/ui/state.rs:860`.

There is no coverage for first, middle, and last recent rows, nor for row targeting after the recent-files list changes. Stale row indices would not be caught. See `apps/player/src-tauri/src/ui/paint_panels.rs:334`.

Long recent-file lists have no exercised scroll-aware row-selection coverage, despite the plan requiring dynamic row hit testing with current scroll offset.

## Plan Requirements Not Met
- Tests must verify the clicked recent file path/title loads.
- Tests must verify playback starts or document the no-backend loaded-only fallback.
- Tests must prove backend load/play dispatch for real playback.
- Tests must cover first, middle, and last recent file rows.
- Tests must repeat recent-file activation after the recent list changes and verify row indices refresh.
- Tests must verify recent-file activation does not change playlist order, selected playlist index, repeat/shuffle, volume, or subtitle state unless documented.
- Tests must cover recent row hit testing for a list larger than the visible drawer area.

## Required Test Shape
- Seed at least three recent files, click first/middle/last rows, and assert media path/title, playback state, and backend load/play calls.
- Replace the recent-files list, click a rendered row again, and assert the current row index is targeted.
- Assert the playlist itself is not mutated unless product rules say recent files should be added there.
- Snapshot unrelated player state before activation and assert it remains unchanged.
- Create an overflowing recent list and verify visible-row selection remains correct.

## Required Changes
- Add a testable backend load/play abstraction or command spy.
- Decide whether no-backend recent row activation should set `is_playing` or expose a documented loaded-only fallback.
- Add deterministic recent-file seeding for first/middle/last and overflow cases.
- Add value-level recent-file row coverage to `plan_ui_e2e` or a focused playlist drawer test.

## Verification
- `cargo test -p tench-player --test plan_ui_e2e recent_file_row`
- `cargo test -p tench-player`
