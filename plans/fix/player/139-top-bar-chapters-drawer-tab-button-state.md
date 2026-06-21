# Chapters Drawer Tab Button State

## Source Plan
- `plans/player/top-bar-chapters-drawer-tab-button-work-plan.md`

## Gap Analysis
The current E2E clicks `player.top.chapters` and asserts Chapters drawer controls are present, but it does not assert `drawer == Some(Chapters)`, active tab highlight, close-on-second-click, AI panel closure, or unrelated playback invariants. See `apps/player/src-tauri/tests/plan_ui_e2e.rs:269`.

Switching from another drawer to Chapters is only exercised incidentally and not asserted as a drawer state/content replacement with playback preserved. See `apps/player/src-tauri/tests/plan_ui_e2e.rs:246`.

The tab highlight is drawn from `state.drawer`, but automation does not expose active/selected state for `player.top.chapters`, so tests cannot assert the highlight through the UI tree. See `apps/player/src-tauri/src/ui/paint_controls.rs:43`.

## Plan Requirements Not Met
- Tests must verify Chapters opens `DrawerTab::Chapters`.
- Tests must verify clicking Chapters again closes the drawer.
- Tests must verify the Chapters tab exposes active/highlighted state while open.
- Tests must verify opening Chapters closes the AI panel.
- Tests must verify switching from another drawer to Chapters replaces drawer content without interrupting playback.
- Tests must verify Chapters toggling does not change media path, playback time, paused state, playlist, or subtitle state.

## Required Test Shape
- Click `player.top.chapters`, assert drawer state, Chapters controls, and active tab state.
- Click `player.top.chapters` again and assert drawer selectors are absent and drawer state is `None`.
- Open AI, then click Chapters and assert `ai_panel_open == false`.
- Open another drawer, then Chapters, and assert only Chapters content remains.
- Snapshot playback state before toggles and assert it remains unchanged.

## Required Changes
- Expose active/selected state for top drawer tabs through automation.
- Add Chapters tab toggle, mutual-exclusion, switch, and invariant coverage to `plan_ui_e2e` or a focused top-bar test.

## Verification
- `cargo test -p tench-player --test plan_ui_e2e top_bar_chapters_drawer_tab`
- `cargo test -p tench-player`
