# Status Bar Zoom In Button State

## Source Plan
- `plans/pixel-design/status-bar-zoom-in-button-work-plan.md`

## Gap Analysis
Zoom In updates `state.zoom`, but the displayed zoom percent reflects only that state value while `canvas_document_rect` also applies an automatic fit factor. For documents larger than the viewport, the visible canvas scale can differ from the displayed percentage, so tests must verify the actual canvas scale rather than only `state.zoom`. See `apps/pixel-design/src-tauri/src/ui/state.rs:1220`, `apps/pixel-design/src-tauri/src/ui/canvas.rs:7`, and `apps/pixel-design/src-tauri/src/ui/mod.rs:1358`.

The current E2E coverage clicks Zoom In once and only asserts `state.zoom` increased. It does not verify the exact +25 increment, displayed percent value, canvas document rect scale, 3200 percent clamp, unchanged document pixels, or selection overlay alignment. See `apps/pixel-design/src-tauri/tests/pixel_design_e2e.rs:517`.

Zoom In does not set a status message or expose disabled state at the 3200 percent boundary. The visible zoom percent may be enough acknowledgement, but boundary availability is not exposed through the `pd.status.zoom_in` automation node. See `apps/pixel-design/src-tauri/src/ui/mod.rs:607` and `apps/pixel-design/src-tauri/src/ui/mod.rs:1292`.

## Plan Requirements Not Met
- Tests must verify Zoom In increases by exactly 25 percentage points.
- Tests must verify the displayed zoom percent updates.
- Tests must verify the actual canvas scale changes consistently with the product-defined zoom/fit model.
- Tests must verify Zoom In clamps at 3200 percent and exposes boundary acknowledgement or disabled state.
- Tests must verify selection overlay coordinates remain aligned after zooming in.
- Tests must verify Zoom In does not change document pixels, history, or dirty state.

## Required Test Shape
- Click `pd.status.zoom_in` from 100 percent and assert state zoom, `pd.auto.zoom_percent`, document rect scale, visible capture change, and unchanged pixels/history/dirty state.
- Repeatedly click Zoom In to the maximum and assert zoom clamps at 3200, displayed percent stays at 3200, and button availability or no-op acknowledgement follows the product rule.
- Create a selection, zoom in, and assert selection overlay bounds still match the selected document pixels.

## Required Changes
- Expose Zoom In boundary availability through automation or status when max zoom is reached.
- Add Zoom In E2E tests for exact increment, display update, canvas scale, clamp, selection overlay alignment, and no document mutation.

## Verification
- `cargo test -p tench-pixel-design --test pixel_design_e2e status_bar_zoom_in`
- `cargo test -p tench-pixel-design`
