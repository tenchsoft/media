# Subtitles Drawer Tab Button State

## Source Plan
- `plans/player/top-bar-subtitles-drawer-tab-button-work-plan.md`

## Gap Analysis
The current E2E clicks `player.top.subtitles` and asserts Subtitles drawer controls are present, but it does not assert `drawer == Some(Subtitles)`, active tab highlight, close-on-second-click, AI panel closure, or unrelated playback invariants. See `apps/player/src-tauri/tests/plan_ui_e2e.rs:323`.

Switching from another drawer to Subtitles is exercised only as part of a longer flow and not asserted as a drawer state/content replacement with playback preserved. See `apps/player/src-tauri/tests/plan_ui_e2e.rs:246`.

The tab highlight is drawn from `state.drawer`, but automation does not expose active/selected state for `player.top.subtitles`, so tests cannot assert the highlight through the UI tree. See `apps/player/src-tauri/src/ui/paint_controls.rs:43`.

## Plan Requirements Not Met
- Tests must verify Subtitles opens `DrawerTab::Subtitles`.
- Tests must verify clicking Subtitles again closes the drawer.
- Tests must verify the Subtitles tab exposes active/highlighted state while open.
- Tests must verify opening Subtitles closes the AI panel.
- Tests must verify switching from another drawer to Subtitles replaces drawer content without interrupting playback.
- Tests must verify Subtitles toggling does not change media path, playback time, paused state, playlist, or subtitle selection/encoding state.

## Required Test Shape
- Click `player.top.subtitles`, assert drawer state, Subtitles controls, and active tab state.
- Click `player.top.subtitles` again and assert drawer selectors are absent and drawer state is `None`.
- Open AI, then click Subtitles and assert `ai_panel_open == false`.
- Open another drawer, then Subtitles, and assert only Subtitles content remains.
- Snapshot playback/subtitle state before toggles and assert it remains unchanged.

## Required Changes
- Expose active/selected state for top drawer tabs through automation.
- Add Subtitles tab toggle, mutual-exclusion, switch, and invariant coverage to `plan_ui_e2e` or a focused top-bar test.

## Verification
- `cargo test -p tench-player --test plan_ui_e2e top_bar_subtitles_drawer_tab`
- `cargo test -p tench-player`
