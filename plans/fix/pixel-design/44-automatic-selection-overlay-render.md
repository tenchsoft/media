# Automatic Selection Overlay Render

## Source Plan
- `plans/pixel-design/automatic-selection-overlay-render-work-plan.md`

## Gap Analysis
The renderer derives overlay coordinates from `normalized_selection()` and maps them through the current document rect. However, `pd.auto.selection_overlay` exposes only the full document rect as its bounds and no normalized selection coordinates, overlay rect, active tool, or dim-outside state. See `apps/pixel-design/src-tauri/src/ui/canvas.rs:49` and `apps/pixel-design/src-tauri/src/ui/mod.rs:1413`.

The current E2E coverage performs one Select drag and only asserts the overlay node exists and status is "Selection updated". It does not verify opposite-direction drags, normalized coordinates, dimmed outside regions, Crop overlay behavior, zoom/pan alignment, alternate paths, persona switches, or resize. See `apps/pixel-design/src-tauri/tests/pixel_design_e2e.rs:431`.

## Plan Requirements Not Met
- Automation metadata must expose normalized selection coordinates and rendered overlay rect.
- Tests must verify dragging in opposite directions produces the same normalized overlay.
- Tests must verify outside dimming for Select and Crop tools.
- Tests must verify overlay alignment after zoom and pan.
- Tests must verify equivalent state changes from alternate paths produce identical overlay output.
- Tests must verify overlay remains correct after persona switches and viewport resize.

## Required Test Shape
- Add a Pixel Design UI automation test that drags selection top-left to bottom-right and bottom-right to top-left, then asserts identical normalized selection metadata and overlay rect.
- Repeat with Crop and assert dimmed outside regions are visible in capture sampling.
- Zoom and pan after creating a selection, then assert overlay metadata and sampled border pixels remain aligned to document coordinates.
- Switch personas and resize, then assert selection overlay metadata and capture remain correct.
- Use capture assertions to verify PNG validity, size, nonblank output, and expected visual changes.

## Required Changes
- Expose normalized selection coordinates, rendered overlay rect, active selection tool, and dim-outside state through automation metadata.
- Add selection overlay render E2E and state tests.

## Verification
- `cargo test -p tench-pixel-design --test pixel_design_e2e automatic_selection_overlay_render`
- `cargo test -p tench-pixel-design`
