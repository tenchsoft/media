# Playlist Drawer Tab Button State

## Source Plan
- `plans/player/top-bar-playlist-drawer-tab-button-work-plan.md`

## Gap Analysis
The current E2E clicks `player.top.playlist` and asserts Playlist drawer controls are present, but it does not assert `drawer == Some(Playlist)`, active tab highlight, close-on-second-click, AI panel closure, or unrelated playback invariants. See `apps/player/src-tauri/tests/plan_ui_e2e.rs:249`.

Switching from another drawer to Playlist is not asserted as a drawer state/content replacement with playback preserved.

The tab highlight is drawn from `state.drawer`, but automation does not expose active/selected state for `player.top.playlist`, so tests cannot assert the highlight through the UI tree. See `apps/player/src-tauri/src/ui/paint_controls.rs:43`.

## Plan Requirements Not Met
- Tests must verify Playlist opens `DrawerTab::Playlist`.
- Tests must verify clicking Playlist again closes the drawer.
- Tests must verify the Playlist tab exposes active/highlighted state while open.
- Tests must verify opening Playlist closes the AI panel.
- Tests must verify switching from another drawer to Playlist replaces drawer content without interrupting playback.
- Tests must verify Playlist toggling does not change media path, playback time, paused state, playlist contents, or subtitle state.

## Required Test Shape
- Click `player.top.playlist`, assert drawer state, Playlist controls, and active tab state.
- Click `player.top.playlist` again and assert drawer selectors are absent and drawer state is `None`.
- Open AI, then click Playlist and assert `ai_panel_open == false`.
- Open another drawer, then Playlist, and assert only Playlist content remains.
- Snapshot playback state before toggles and assert it remains unchanged.

## Required Changes
- Expose active/selected state for top drawer tabs through automation.
- Add Playlist tab toggle, mutual-exclusion, switch, and invariant coverage to `plan_ui_e2e` or a focused top-bar test.

## Verification
- `cargo test -p tench-player --test plan_ui_e2e top_bar_playlist_drawer_tab`
- `cargo test -p tench-player`
