# Status Bar Zoom Slider Control State

## Source Plan
- `plans/pixel-design/status-bar-zoom-slider-control-work-plan.md`

## Gap Analysis
The zoom slider sets `state.zoom` from pointer x-position, but the displayed zoom percent reflects only that state value while `canvas_document_rect` also applies an automatic fit factor. Tests must verify the actual canvas scale under the product-defined zoom/fit model, not just the numeric state. See `apps/pixel-design/src-tauri/src/ui/mod.rs:618` and `apps/pixel-design/src-tauri/src/ui/canvas.rs:7`.

The slider maps `t * 3200` and then clamps to 10-3200. This reaches the minimum through clamping rather than a direct 10-3200 interpolation, and edge/near-edge behavior is not specified or tested. See `apps/pixel-design/src-tauri/src/ui/mod.rs:627`.

The current E2E coverage clicks the slider around 75 percent width and only asserts zoom is greater than the original value. It does not verify left-edge minimum, right-edge maximum, displayed label, canvas scale, shared state with Zoom In/Out, or unchanged document pixels/history/dirty state. See `apps/pixel-design/src-tauri/tests/pixel_design_e2e.rs:521`.

The `dragging_zoom_slider` state exists but is not used by the slider interaction. If the product expects slider dragging, that behavior is missing; if click-only is intentional, the unused state should be removed or documented. See `apps/pixel-design/src-tauri/src/ui/state.rs:467`.

## Plan Requirements Not Met
- Slider edge behavior must be defined as direct 10-3200 interpolation or documented clamped 0-3200 mapping.
- Tests must verify clicking near the left edge moves zoom near the minimum.
- Tests must verify clicking near the right edge moves zoom near the maximum.
- Tests must verify displayed zoom percent updates after slider input.
- Tests must verify Zoom In, Zoom Out, and slider share one zoom state.
- Tests must verify actual canvas scale updates and document pixels/history/dirty state remain unchanged.
- Slider drag support must be implemented or unused drag state removed/documented.

## Required Test Shape
- Click near the left edge of `pd.status.zoom_slider` and assert state zoom, `pd.auto.zoom_percent`, canvas document rect scale, and unchanged document state.
- Click near the right edge and assert maximum/near-maximum zoom according to the product-defined mapping.
- Set zoom with the slider, then click Zoom In and Zoom Out and assert all controls update the same zoom state and label.
- If slider dragging is supported, drag across the slider and assert continuous updates; otherwise remove or document `dragging_zoom_slider`.

## Required Changes
- Define and implement the intended slider x-position to zoom mapping.
- Implement slider drag behavior or remove/document unused drag state.
- Add Zoom Slider E2E tests for left edge, right edge, label update, shared state with buttons, canvas scale, and no document mutation.

## Verification
- `cargo test -p tench-pixel-design --test pixel_design_e2e status_bar_zoom_slider`
- `cargo test -p tench-pixel-design`
