# Automatic Side Panel Layout Reserve State

## Source Plan
- `plans/player/automatic-side-panel-layout-reserve-work-plan.md`

## Gap Analysis
`video_surface::video_right` subtracts a fixed 320px side-panel width without clamping or responsive fallback. At narrow window sizes the video/control region can become zero or negative width, so controls and seekbar geometry can overlap or invert instead of reserving usable space. See `apps/player/src-tauri/src/ui/video_surface.rs:3`.

The existing E2E opens the Playlist drawer at a single 1280x720 viewport and only asserts that `player.automatic.side_panel_layout` is present. It does not verify AI panel layout, every drawer tab, overlap-free controls, seekbar recomputation, hit-test geometry, resize behavior, or small viewport behavior. See `apps/player/src-tauri/tests/plan_ui_e2e.rs:246`.

The automatic side-panel layout node is always emitted with no value for video rect, panel rect, reserved width, or active side-panel type, so automation cannot assert that top bar, controls, seekbar, and video surface were recomputed from canonical panel state. See `apps/player/src-tauri/src/ui/app.rs:2309`.

## Plan Requirements Not Met
- Side-panel layout must handle narrow viewports without negative or overlapping video/control geometry.
- Tests must verify AI panel and each drawer reserve side-panel width.
- Tests must verify top bar, controls, seekbar, and video surface bounds do not overlap the side panel.
- Tests must verify resize behavior while a side panel is open.
- Automation must expose video rect, panel rect, or reserved width for deterministic layout assertions.

## Required Test Shape
- Open the AI panel and each drawer at desktop and narrow viewports, then assert video surface, controls, seekbar, top bar buttons, and panel bounds do not overlap.
- Resize while a drawer or AI panel is open and assert the same controls remain within the recomputed video/control rect.
- Assert click bounds for seekbar and top controls use the resized `video_right`, not stale full-width geometry.

## Required Changes
- Clamp or adapt side-panel reservation for narrow viewports.
- Expose side-panel layout metrics in `player.automatic.side_panel_layout`.
- Extend `plan_ui_e2e` with AI panel, all drawer tabs, resize, small viewport, and hit-test geometry assertions.

## Verification
- `cargo test -p tench-player --test plan_ui_e2e automatic_side_panel_layout`
- `cargo test -p tench-player`
