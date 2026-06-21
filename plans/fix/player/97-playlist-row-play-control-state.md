# Playlist Row Play Control State

## Source Plan
- `plans/player/playlist-row-play-control-work-plan.md`

## Gap Analysis
The current E2E clicks `player.playlist.row.1` and only asserts the capture changed. It does not assert `current_playlist_index`, media path/title, playback state, backend load/play dispatch, active row highlight, or unrelated state invariants. See `apps/player/src-tauri/tests/plan_ui_e2e.rs:257`.

When no backend is present, `load_media_for_action` uses `PlayerState::open_media`, which sets `is_playing = false`. The plan expects the clicked playlist entry to start playback, so the headless fallback does not mirror the real backend path. See `apps/player/src-tauri/src/ui/app.rs:313` and `apps/player/src-tauri/src/ui/state.rs:860`.

There is no coverage for first, middle, and last playlist rows, nor for row targeting after add/remove changes the playlist. Stale row indices would not be caught. See `apps/player/src-tauri/src/ui/paint_panels.rs:232`.

Automation does not expose active/current row state directly, so tests cannot assert the highlighted playlist row through the UI tree without reading internal state.

Long playlists have no exercised scroll-aware row-selection coverage, despite the plan requiring dynamic row hit testing with current scroll offset.

## Plan Requirements Not Met
- Tests must verify clicked playlist row becomes `current_playlist_index`.
- Tests must verify the clicked entry path/title loads.
- Tests must verify playback starts or document the no-backend fallback behavior.
- Tests must prove backend load/play dispatch for real playback.
- Tests must verify active row highlight/selected state.
- Tests must cover first, middle, and last playlist rows.
- Tests must repeat row play after add/remove and verify row indices refresh.
- Tests must cover playlist row hit testing for a list larger than the visible drawer area.

## Required Test Shape
- Seed at least three playlist entries, click first/middle/last rows, and assert current index, media path/title, row active state, and playback state.
- Use a backend spy to assert the selected path is loaded and playback starts.
- Add and remove playlist entries, click a rendered row again, and assert the current row index is targeted.
- Assert unrelated fields such as volume, repeat, shuffle, drawer tab, subtitle state, and recent files remain unchanged.
- Create an overflowing playlist and verify visible-row selection remains correct.

## Required Changes
- Add a testable backend load/play abstraction or command spy.
- Decide whether no-backend playlist row play should set `is_playing` or expose a documented loaded-only fallback.
- Expose current/active playlist row state through automation.
- Add value-level playlist row play coverage to `plan_ui_e2e` or a focused playlist drawer test.

## Verification
- `cargo test -p tench-player --test plan_ui_e2e playlist_row_play`
- `cargo test -p tench-player`
