# Canvas Move Layer Drag Control State

## Source Plan
- `plans/pixel-design/canvas-move-layer-drag-control-work-plan.md`

## Gap Analysis
Move Layer updates `offset_x` / `offset_y` during pointer move, but the canvas renders from `composited_image`, and that flattened cache is only rebuilt on pointer up or leave. This means the layer offset can change in state while the document image does not visibly move in real time during the drag. See `apps/pixel-design/src-tauri/src/ui/state.rs:921`, `apps/pixel-design/src-tauri/src/ui/mod.rs:672`, and `apps/pixel-design/src-tauri/src/ui/canvas.rs:35`.

Move Layer writes directly to `active_layer_mut()` without checking `locked` or `visible`. A locked active layer can still have its offsets changed, and hidden-layer behavior is not explicitly defined or surfaced to the user. See `apps/pixel-design/src-tauri/src/ui/state.rs:878`, `apps/pixel-design/src-tauri/src/ui/state.rs:926`, and `crates/pixel-core/src/document.rs:10`.

The current flow pushes a history entry on pointer down before any movement is known. A click with no effective move, or a blocked move after lock/visibility rules are added, can still create a history entry unless the commit path is made conditional. See `apps/pixel-design/src-tauri/src/ui/state.rs:880`.

Move Layer ignores the current selection entirely. If selection is meant to constrain movement, move selected pixels, or remain unaffected while the whole layer moves, that rule is not defined by code or tests. See `apps/pixel-design/src-tauri/src/ui/state.rs:917` and `apps/pixel-design/src-tauri/src/ui/state.rs:921`.

The current E2E coverage performs one Move drag and only asserts that `offset_x` changed and status says `Layer moved`. It does not verify locked layers, hidden layers, selected regions, real-time visual movement, zoom/pan coordinate accuracy, no-op history behavior, undo restoration, dirty state, or flattened-canvas refresh. See `apps/pixel-design/src-tauri/tests/pixel_design_e2e.rs:460`.

## Plan Requirements Not Met
- Move Layer must visibly update the affected canvas region during drag, not only after release.
- Move Layer must not mutate locked layers.
- Move Layer must define and enforce behavior for hidden active layers.
- Move Layer must define how current selection affects a layer move.
- History must be committed only for an actual layer-offset change.
- Tests must verify Move Layer after zooming and panning.
- Tests must verify undo restores the previous active-layer offset and rendered output.

## Required Test Shape
- Add a Pixel Design UI automation test that drags a visible unlocked layer and asserts offset, status, dirty state, flattened output, and capture pixels change.
- During an in-progress drag, capture the canvas before release and assert the layer is visibly rendered at the transient offset.
- Lock the active layer, attempt Move Layer, and assert offsets, history, dirty state, status, and flattened output follow the blocked-action rule.
- Hide the active layer, attempt Move Layer, and assert the product-defined hidden-layer behavior.
- Create a selection, perform Move Layer, and assert the product-defined selection behavior.
- Move after zoom and after pan, then assert document-coordinate mapping and resulting offsets are correct.
- Undo after a committed Move Layer drag and assert offsets, history, dirty state, flattened output, and visible capture restore.

## Required Changes
- Rebuild or invalidate the flattened canvas during Move Layer drag, or render the active layer at its transient offset without relying on a stale composited cache.
- Gate Move Layer mutation on locked-layer and product-defined hidden-layer rules before starting a history entry.
- Define the selection rule for Move Layer and implement it consistently.
- Defer or collapse history creation so no-op and blocked moves do not create undo entries.
- Add Move Layer lock, visibility, selection, real-time render, zoom/pan, history, and undo tests.

## Verification
- `cargo test -p tench-pixel-design --test pixel_design_e2e canvas_move_layer_drag`
- `cargo test -p tench-pixel-design`
