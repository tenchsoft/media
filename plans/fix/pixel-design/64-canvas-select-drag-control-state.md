# Canvas Select Drag Control State

## Source Plan
- `plans/pixel-design/canvas-select-drag-control-work-plan.md`

## Gap Analysis
Select drag creates a selection on pointer down and updates it on move, but there is no minimum-size policy for Select. Tiny or zero-area drags are retained and reported as `Selection updated`, so the required minimum-size behavior is undefined. See `apps/pixel-design/src-tauri/src/ui/state.rs:863`, `apps/pixel-design/src-tauri/src/ui/state.rs:1030`, and `apps/pixel-design/src-tauri/src/ui/state.rs:1110`.

The automation node for `pd.auto.selection_overlay` is exposed with the full document bounds and no selection rectangle value. Tests can detect that an overlay exists, but they cannot verify normalized selection coordinates, tiny-selection behavior, or zoom/pan mapping through stable automation metadata. See `apps/pixel-design/src-tauri/src/ui/mod.rs:1413`.

Finishing Select drag calls `refresh_flattened()` through the generic canvas pointer-up path even though selection changes do not modify document pixels. This violates the flattened-canvas refresh rule for a selection-only interaction. See `apps/pixel-design/src-tauri/src/ui/mod.rs:672`.

The current E2E coverage performs one left-to-right Select drag and only asserts overlay presence and status text. It does not verify right-to-left normalization, tiny selections, selection rectangle coordinates, zoom/pan coordinate accuracy, unchanged document pixels, unchanged history, unchanged dirty state, or unchanged flattened content. See `apps/pixel-design/src-tauri/tests/pixel_design_e2e.rs:430`.

## Plan Requirements Not Met
- Select drag must define and enforce minimum-size behavior.
- Selection overlay automation must expose the actual normalized selection rectangle.
- Selection-only drags must repaint without rebuilding flattened document pixels.
- Tests must verify left-to-right and right-to-left normalized selection bounds.
- Tests must verify tiny-selection behavior.
- Tests must verify Select drag after zooming and panning.
- Tests must verify Select drag leaves document pixels, history, dirty state, and flattened content unchanged.

## Required Test Shape
- Add a Pixel Design UI automation test that creates a left-to-right selection and asserts normalized document-space bounds.
- Drag right-to-left and bottom-to-top, then assert the normalized selection bounds match the expected rectangle.
- Perform a tiny drag and assert the product-defined minimum-size outcome.
- Zoom and pan before Select drag, then assert pointer-to-document mapping through the reported selection bounds.
- Capture history length, dirty state, representative pixels, and flattened capture before/after Select drag and assert they do not change.

## Required Changes
- Define the Select minimum-size rule and apply it on release.
- Expose normalized selection bounds through `pd.auto.selection_overlay` metadata or a dedicated automation node value.
- Skip `refresh_flattened()` for pure selection completion while still repainting the overlay.
- Add Select normalization, tiny-selection, zoom/pan, no-pixel-change, and no-history tests.

## Verification
- `cargo test -p tench-pixel-design --test pixel_design_e2e canvas_select_drag`
- `cargo test -p tench-pixel-design`
