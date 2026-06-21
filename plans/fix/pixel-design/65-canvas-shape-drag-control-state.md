# Canvas Shape Drag Control State

## Source Plan
- `plans/pixel-design/canvas-shape-drag-control-work-plan.md`

## Gap Analysis
`ShapeType` supports Rectangle, Ellipse, and Line, but `shape_type` is initialized to Rectangle and there is no control path that changes it. The tool context chips also render a fixed `Type: Rectangle` string, so users and E2E tests cannot exercise the required ellipse or line raster operations through the UI. See `apps/pixel-design/src-tauri/src/ui/state.rs:141`, `apps/pixel-design/src-tauri/src/ui/state.rs:159`, and `apps/pixel-design/src-tauri/src/ui/state.rs:533`.

Shape writes directly to `active_layer_mut()` without checking whether the active layer is locked or hidden. A locked active layer can still receive shape pixels, and hidden-layer behavior is not explicitly defined or surfaced. See `apps/pixel-design/src-tauri/src/ui/state.rs:1068` and `crates/pixel-core/src/document.rs:10`.

Shape ignores the current selection. If shapes should be clipped to the selection, blocked outside the selection, or deliberately independent of selection, that rule is not defined or verified. See `apps/pixel-design/src-tauri/src/ui/state.rs:917` and `apps/pixel-design/src-tauri/src/ui/state.rs:1068`.

History is pushed on pointer down before the shape is actually placed. No-op drags, blocked locked-layer attempts, or other failed placements can still create history entries unless history is moved to the commit path and made conditional. See `apps/pixel-design/src-tauri/src/ui/state.rs:872`.

The current E2E coverage performs one Shape drag and only asserts status text. It does not verify rectangle pixels, ellipse pixels, line pixels, foreground color, zoom/pan coordinate accuracy, locked or hidden layers, selection behavior, dirty state, flattened-canvas refresh, or undo restoration. See `apps/pixel-design/src-tauri/tests/pixel_design_e2e.rs:450`.

## Plan Requirements Not Met
- The UI must expose a way to select Rectangle, Ellipse, and Line shape types.
- Shape drag must verify each shape type maps to the intended raster operation.
- Shape drag must not mutate locked layers.
- Shape drag must define and enforce behavior for hidden active layers.
- Shape drag must define how current selection affects shape placement.
- History must be committed only when a shape is actually placed.
- Tests must verify Shape drag after zooming and panning.
- Tests must verify undo restores the previous pixel state after Shape drag.

## Required Test Shape
- Add UI automation controls for shape type selection, then test Rectangle, Ellipse, and Line through real UI events.
- For each shape type, drag known document bounds and assert representative pixels match the expected raster result and foreground color.
- Lock the active layer, attempt Shape drag, and assert pixels, history, dirty state, status, and flattened output follow the blocked-action rule.
- Hide the active layer, attempt Shape drag, and assert the product-defined hidden-layer behavior.
- Create a selection, perform Shape drag, and assert the product-defined selection behavior.
- Zoom and pan before Shape drag, then assert pointer-to-document mapping through pixel samples.
- Undo after a committed Shape drag and assert pixels, history, dirty state, flattened output, and visible capture restore.

## Required Changes
- Add a UI/state path for choosing `shape_type` and expose the selected value through automation.
- Gate Shape rasterization on locked-layer and product-defined hidden-layer rules before creating history.
- Define and implement Shape selection behavior.
- Move history creation to the successful placement path or remove no-op history entries.
- Add Shape type, pixel, foreground color, lock, visibility, selection, zoom/pan, undo, and flattened-refresh tests.

## Verification
- `cargo test -p tench-pixel-design --test pixel_design_e2e canvas_shape_drag`
- `cargo test -p tench-pixel-design`
