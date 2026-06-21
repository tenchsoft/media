# Canvas Gradient Drag Control State

## Source Plan
- `plans/pixel-design/canvas-gradient-drag-control-work-plan.md`

## Gap Analysis
Gradient uses the shared Select/Crop/Gradient drag path, so pointer down always creates a `selection` rectangle. Because `finish_canvas_action` then sees `normalized_selection()`, the fallback "full canvas" branch is not reached through the normal drag flow. This leaves the no-selection full-canvas behavior from the plan unimplemented or unreachable. See `apps/pixel-design/src-tauri/src/ui/state.rs:863` and `apps/pixel-design/src-tauri/src/ui/state.rs:996`.

Gradient writes directly to `active_layer_mut()` without checking whether the active layer is locked or hidden, even though the plan requires respecting lock and visibility. See `apps/pixel-design/src-tauri/src/ui/state.rs:1035`.

Gradient interpolation is horizontal across the normalized rectangle and does not use the drag vector direction. If the product intends drag direction to define the gradient direction, that behavior is missing and untested. See `apps/pixel-design/src-tauri/src/ui/state.rs:1052`.

The current E2E coverage performs one Gradient drag and only asserts status text. It does not verify affected pixels, selection/full-canvas behavior, background color changes, zoom/pan coordinate accuracy, undo restoration, locked/hidden layers, or flattened-canvas refresh. See `apps/pixel-design/src-tauri/tests/pixel_design_e2e.rs:441`.

## Plan Requirements Not Met
- No-selection full-canvas gradient behavior must be reachable or explicitly redefined.
- Gradient must not mutate locked layers.
- Gradient must define and enforce behavior for hidden active layers.
- Tests must verify foreground-to-background pixels in the affected area.
- Tests must verify Gradient after changing background color.
- Tests must verify pointer-to-document coordinates after zoom and pan.
- Tests must verify undo restores the previous pixel state after Gradient.
- Tests must verify flattened canvas refresh matches active-layer pixels.

## Required Test Shape
- Add a Pixel Design UI automation test that applies Gradient with no prior selection and asserts the product-defined full-canvas or dragged-rect behavior.
- Create an active selection, apply Gradient, and assert only selected pixels change.
- Change background color, apply Gradient, and assert sampled pixels interpolate from foreground to background.
- Zoom and pan, apply Gradient at known canvas fractions, and assert mapped document pixels are correct.
- Lock and hide the active layer before Gradient and assert the product-defined behavior.
- Undo after Gradient and assert active-layer pixels, flattened capture, dirty state, and history restore.

## Required Changes
- Define whether Gradient drag applies to full canvas, an existing selection, or the dragged rectangle when no selection exists, then make the flow reachable.
- Gate Gradient on layer lock and product-defined visibility rules.
- Define and implement drag-vector direction if required.
- Add Gradient pixel, selection, color, zoom/pan, undo, lock, and visibility E2E/state tests.

## Verification
- `cargo test -p tench-pixel-design --test pixel_design_e2e canvas_gradient_drag`
- `cargo test -p tench-pixel-design`
