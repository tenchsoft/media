# Info Drawer Tab Button State

## Source Plan
- `plans/player/top-bar-info-drawer-tab-button-work-plan.md`

## Gap Analysis
The current E2E clicks `player.top.info` and asserts Info drawer controls are present, but it does not assert `drawer == Some(Info)`, active tab highlight, close-on-second-click, AI panel closure, or unrelated playback invariants. See `apps/player/src-tauri/tests/plan_ui_e2e.rs:450`.

Switching from another drawer to Info is exercised only as part of a longer flow and not asserted as a drawer state/content replacement with playback preserved. See `apps/player/src-tauri/tests/plan_ui_e2e.rs:246`.

The tab highlight is drawn from `state.drawer`, but automation does not expose active/selected state for `player.top.info`, so tests cannot assert the highlight through the UI tree. See `apps/player/src-tauri/src/ui/paint_controls.rs:43`.

## Plan Requirements Not Met
- Tests must verify Info opens `DrawerTab::Info`.
- Tests must verify clicking Info again closes the drawer.
- Tests must verify the Info tab exposes active/highlighted state while open.
- Tests must verify opening Info closes the AI panel.
- Tests must verify switching from another drawer to Info replaces drawer content without interrupting playback.
- Tests must verify Info toggling does not change media path, playback time, paused state, playlist, subtitle state, or AI draft state beyond documented panel closure.

## Required Test Shape
- Click `player.top.info`, assert drawer state, Info controls, and active tab state.
- Click `player.top.info` again and assert drawer selectors are absent and drawer state is `None`.
- Open AI, then click Info and assert `ai_panel_open == false`.
- Open another drawer, then Info, and assert only Info content remains.
- Snapshot playback state before toggles and assert it remains unchanged.

## Required Changes
- Expose active/selected state for top drawer tabs through automation.
- Add Info tab toggle, mutual-exclusion, switch, and invariant coverage to `plan_ui_e2e` or a focused top-bar test.

## Verification
- `cargo test -p tench-player --test plan_ui_e2e top_bar_info_drawer_tab`
- `cargo test -p tench-player`
