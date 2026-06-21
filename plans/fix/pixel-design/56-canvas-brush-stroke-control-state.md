# Canvas Brush Stroke Control State

## Source Plan
- `plans/pixel-design/canvas-brush-stroke-control-work-plan.md`

## Gap Analysis
Pointer events map viewport coordinates through the current document rect and Brush begins a stroke with history pushed before rasterization. However, stroke commit writes directly to `active_layer_mut()` without checking whether the active layer is locked or hidden, even though the plan requires respecting layer lock and visibility. See `apps/pixel-design/src-tauri/src/ui/mod.rs:637`, `apps/pixel-design/src-tauri/src/ui/state.rs:809`, and `apps/pixel-design/src-tauri/src/ui/state.rs:949`.

Selection-constrained rasterization exists, but it is not tested. The current E2E coverage draws one Brush stroke and only asserts the capture changed and automatic nodes are present. It does not verify short vs long strokes, foreground-color pixels, selection masking, zoom/pan coordinate accuracy, undo restoration, active-layer targeting, locked/hidden layers, or flattened-canvas pixel refresh. See `apps/pixel-design/src-tauri/tests/pixel_design_e2e.rs:373`.

## Plan Requirements Not Met
- Brush strokes must not modify locked layers.
- Brush strokes must define and enforce behavior for hidden active layers.
- Tests must verify foreground-color pixels for short and long strokes.
- Tests must verify strokes inside an active selection only affect selected pixels.
- Tests must verify pointer-to-document coordinates after zoom and pan.
- Tests must verify undo restores the previous pixel state after a stroke.
- Tests must verify flattened canvas refresh matches the active layer pixels.

## Required Test Shape
- Add a Pixel Design UI automation test that draws short and long strokes and asserts active-layer pixels changed at expected document coordinates.
- Create a selection, draw through it, and assert pixels outside the selection remain unchanged.
- Zoom and pan, draw at known canvas fractions, and assert mapped document pixels are correct.
- Lock the active layer and attempt a stroke, then assert pixels and history are unchanged with a clear status.
- Hide the active layer and assert the product-defined stroke behavior is enforced.
- Undo after a brush stroke and assert active-layer pixels, flattened capture, dirty state, and history restore.

## Required Changes
- Gate Brush stroke begin/commit on layer lock and product-defined visibility rules.
- Add Brush stroke pixel, selection, zoom/pan, undo, lock, and visibility E2E/state tests.

## Verification
- `cargo test -p tench-pixel-design --test pixel_design_e2e canvas_brush_stroke`
- `cargo test -p tench-pixel-design`
