# Status Bar Zoom Out Button State

## Source Plan
- `plans/pixel-design/status-bar-zoom-out-button-work-plan.md`

## Gap Analysis
Zoom Out updates `state.zoom`, but the displayed zoom percent reflects only that state value while `canvas_document_rect` also applies an automatic fit factor. For documents larger than the viewport, the visible canvas scale can differ from the displayed percentage, so tests must verify the actual canvas scale rather than only `state.zoom`. See `apps/pixel-design/src-tauri/src/ui/state.rs:1224`, `apps/pixel-design/src-tauri/src/ui/canvas.rs:7`, and `apps/pixel-design/src-tauri/src/ui/mod.rs:1358`.

`zoom_out` does not adjust or clamp `viewport_offset_x` / `viewport_offset_y`. After panning, zooming out can leave the document offset incoherent unless product rules explicitly allow that state. See `apps/pixel-design/src-tauri/src/ui/canvas.rs:16`.

The current E2E coverage clicks Zoom Out after Zoom In but does not assert the result. It does not verify the exact -25 decrement, displayed percent value, canvas document rect scale, 10 percent clamp, pan/offset coherence, unchanged document pixels, or history/dirty stability. See `apps/pixel-design/src-tauri/tests/pixel_design_e2e.rs:517`.

Zoom Out does not set a status message or expose disabled state at the 10 percent boundary. The visible zoom percent may be enough acknowledgement, but boundary availability is not exposed through the `pd.status.zoom_out` automation node. See `apps/pixel-design/src-tauri/src/ui/mod.rs:599` and `apps/pixel-design/src-tauri/src/ui/mod.rs:1277`.

## Plan Requirements Not Met
- Tests must verify Zoom Out decreases by exactly 25 percentage points.
- Tests must verify the displayed zoom percent updates.
- Tests must verify the actual canvas scale changes consistently with the product-defined zoom/fit model.
- Tests must verify Zoom Out clamps at 10 percent and exposes boundary acknowledgement or disabled state.
- Zoom Out after panning must define and enforce coherent viewport offset behavior.
- Tests must verify Zoom Out does not change document pixels, history, or dirty state.

## Required Test Shape
- Click `pd.status.zoom_out` from a known zoom and assert state zoom, `pd.auto.zoom_percent`, document rect scale, visible capture change, and unchanged pixels/history/dirty state.
- Repeatedly click Zoom Out to the minimum and assert zoom clamps at 10, displayed percent stays at 10, and button availability or no-op acknowledgement follows the product rule.
- Pan the viewport, zoom out, and assert viewport offset and document rect remain coherent according to the product-defined framing rule.

## Required Changes
- Define and implement viewport offset behavior when zooming out after pan.
- Expose Zoom Out boundary availability through automation or status when minimum zoom is reached.
- Add Zoom Out E2E tests for exact decrement, display update, canvas scale, clamp, pan coherence, and no document mutation.

## Verification
- `cargo test -p tench-pixel-design --test pixel_design_e2e status_bar_zoom_out`
- `cargo test -p tench-pixel-design`
