# Automatic Grid Render

## Source Plan
- `plans/pixel-design/automatic-grid-render-work-plan.md`

## Gap Analysis
Grid painting reads `show_grid`, document width, and the current rendered document rect to choose a zoom-derived step, and Ctrl+G toggles `show_grid` with repaint. However, the automation node for `pd.auto.grid` exposes no grid step, origin, document rect, or enabled state beyond node presence. See `apps/pixel-design/src-tauri/src/ui/canvas.rs:113`, `apps/pixel-design/src-tauri/src/ui/canvas.rs:201`, and `apps/pixel-design/src-tauri/src/ui/mod.rs:1440`.

The current E2E coverage toggles grid/rulers and only asserts `pd.auto.grid` is present. It does not verify grid alignment, zoom-derived spacing, pan behavior, edit persistence, alternate toggle paths, persona switches, resize behavior, or that export output excludes the grid overlay. See `apps/pixel-design/src-tauri/tests/pixel_design_e2e.rs:509`.

## Plan Requirements Not Met
- Automation metadata must expose grid enabled state, origin, step size, and document rect.
- Tests must verify grid spacing changes or remains correct according to zoom-derived rules.
- Tests must verify grid alignment stays tied to document coordinates after pan and pixel edits.
- Tests must verify grid render remains correct after persona switches and viewport resize.
- Tests must verify equivalent state changes from alternate paths produce identical grid output.
- Tests must verify export output does not include the grid overlay unless explicitly requested.

## Required Test Shape
- Add a Pixel Design UI automation test that enables grid, reads grid metadata, and samples grid-line pixels from a capture.
- Zoom in/out and assert metadata and sampled pixels match the expected grid spacing.
- Pan the canvas and assert grid lines move with document coordinates.
- Edit pixels with grid enabled and assert document pixels change while grid remains an overlay.
- Export with grid enabled and assert exported pixels do not contain grid lines.
- Switch personas and resize, then assert grid metadata and capture remain correct.

## Required Changes
- Expose grid enabled state, origin, step, and document rect through automation metadata.
- Add grid render E2E tests with capture sampling and export verification.

## Verification
- `cargo test -p tench-pixel-design --test pixel_design_e2e automatic_grid_render`
- `cargo test -p tench-pixel-design`
