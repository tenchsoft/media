# Canvas Eraser Stroke Control State

## Source Plan
- `plans/pixel-design/canvas-eraser-stroke-control-work-plan.md`

## Gap Analysis
Eraser uses the shared Brush/Eraser stroke path and `BrushStroke` reduces alpha when `is_eraser` is true. However, the selection-constrained path rasterizes into a new transparent temp buffer and only copies temp pixels where `a > 0`. For eraser strokes, the temp alpha remains `0`, so erasing inside an active selection copies nothing and does not erase allowed pixels. See `apps/pixel-design/src-tauri/src/ui/state.rs:949` and `crates/pixel-core/src/stroke.rs:142`.

Eraser commit writes directly to `active_layer_mut()` without checking whether the active layer is locked or hidden, even though the plan requires respecting layer lock and visibility. See `apps/pixel-design/src-tauri/src/ui/state.rs:954`.

The current E2E coverage performs one eraser drag and only asserts status text. It does not verify erased alpha, erasing over empty transparency, selection-limited erasing, zoom/pan coordinate accuracy, undo restoration, active-layer targeting, locked/hidden layers, or flattened-canvas refresh. See `apps/pixel-design/src-tauri/tests/pixel_design_e2e.rs:391`.

## Plan Requirements Not Met
- Eraser strokes inside selections must erase only allowed selected pixels.
- Eraser strokes must not modify locked layers.
- Eraser strokes must define and enforce behavior for hidden active layers.
- Tests must verify alpha changes when erasing over painted pixels.
- Tests must verify erasing empty transparency is stable and non-corrupting.
- Tests must verify pointer-to-document coordinates after zoom and pan.
- Tests must verify undo restores the previous pixel state after erasing.
- Tests must verify flattened canvas refresh matches the active layer pixels.

## Required Test Shape
- Add a Pixel Design UI automation test that paints pixels, erases over them, and asserts alpha decreases or becomes transparent at expected document coordinates.
- Erase over already transparent pixels and assert no unrelated pixels change.
- Create a selection, erase through it, and assert only selected pixels change.
- Zoom and pan, erase at known canvas fractions, and assert mapped document pixels are correct.
- Lock and hide the active layer before erasing and assert the product-defined behavior.
- Undo after an eraser stroke and assert active-layer pixels, flattened capture, dirty state, and history restore.

## Required Changes
- Fix selection-constrained eraser behavior so eraser effects are applied to selected active-layer pixels.
- Gate Eraser stroke begin/commit on layer lock and product-defined visibility rules.
- Add Eraser stroke pixel, selection, zoom/pan, undo, lock, and visibility E2E/state tests.

## Verification
- `cargo test -p tench-pixel-design --test pixel_design_e2e canvas_eraser_stroke`
- `cargo test -p tench-pixel-design`
