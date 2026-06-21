# Canvas Crop Drag Control State

## Source Plan
- `plans/pixel-design/canvas-crop-drag-control-work-plan.md`

## Gap Analysis
Pointer events map viewport coordinates through the current document rect, and Crop applies when the normalized rectangle is larger than two document pixels. However, history is pushed after `document.crop(...)`, so the history snapshot can capture the post-crop state instead of the pre-crop state needed for undo restoration. See `apps/pixel-design/src-tauri/src/ui/mod.rs:637` and `apps/pixel-design/src-tauri/src/ui/state.rs:980`.

Crop mutates every layer through `Document::crop` without a product-defined rule for locked or hidden layers, even though the plan requires the action to respect lock, visibility, history, and flattened-canvas refresh rules. See `crates/pixel-core/src/document.rs:231`.

The current E2E coverage performs one normal crop and asserts document width decreased and status text. It does not verify exact dimensions, tiny-rectangle no-op behavior, undo restoration, zoom/pan coordinate accuracy, flattened canvas pixels, selection cleanup, or lock/visibility semantics. See `apps/pixel-design/src-tauri/tests/pixel_design_e2e.rs:487`.

## Plan Requirements Not Met
- Crop must push history before mutating document dimensions so undo restores the previous state.
- Tiny crop rectangles must have a defined no-op status/history behavior.
- Crop lock and visibility semantics must be defined and enforced.
- Tests must verify exact crop dimensions and flattened-canvas refresh.
- Tests must verify crop after zoom and pan maps pointer coordinates correctly.
- Tests must verify undo restores previous dimensions, pixels, and history state.

## Required Test Shape
- Add a Pixel Design UI automation test that performs a known crop and asserts exact document dimensions, layer buffer sizes, status text, dirty state, and flattened capture.
- Drag a tiny crop rectangle and assert dimensions, pixels, history, and status follow the product-defined no-op behavior.
- Zoom and pan, crop at known canvas fractions, and assert mapped document dimensions are correct.
- Undo after crop and assert document dimensions, active layer pixels, flattened capture, and history index restore.
- Lock and hide layers before crop and assert the product-defined behavior for those layers.

## Required Changes
- Move crop history snapshot before `document.crop(...)` or otherwise store the pre-crop document state.
- Define and implement crop behavior for locked and hidden layers.
- Add crop drag E2E and state tests for normal, tiny, zoom/pan, undo, and lock/visibility scenarios.

## Verification
- `cargo test -p tench-pixel-design --test pixel_design_e2e canvas_crop_drag`
- `cargo test -p tench-pixel-design`
