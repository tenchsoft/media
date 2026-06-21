# Canvas Fill Click Control State

## Source Plan
- `plans/pixel-design/canvas-fill-click-control-work-plan.md`

## Gap Analysis
Fill maps the click through the current document rect, pushes history, and flood-fills the active layer with foreground color. However, it writes directly to `active_layer_mut()` without checking whether the active layer is locked or hidden, even though the plan requires respecting lock and visibility. See `apps/pixel-design/src-tauri/src/ui/mod.rs:637` and `apps/pixel-design/src-tauri/src/ui/state.rs:826`.

When a selection exists, Fill pushes history before checking whether the clicked point is inside the selection. If the click is outside the selected rect, no pixels are filled, but history/dirty can still advance and status can still say "Filled region". See `apps/pixel-design/src-tauri/src/ui/state.rs:827` and `apps/pixel-design/src-tauri/src/ui/state.rs:843`.

The current E2E coverage clicks Fill once and only asserts status text. It does not verify filled pixels, bounded-region behavior, selection-limited fill, click-outside-selection no-op behavior, transparency/tolerance behavior, zoom/pan coordinate accuracy, undo restoration, locked/hidden layers, or flattened-canvas refresh. See `apps/pixel-design/src-tauri/tests/pixel_design_e2e.rs:400`.

## Plan Requirements Not Met
- Fill must not mutate locked layers.
- Fill must define and enforce behavior for hidden active layers.
- Fill with an active selection must avoid history/status changes when no selected pixels are affected.
- Tests must verify foreground-color pixels in bounded and selected regions.
- Tests must verify tolerance and transparent-pixel behavior.
- Tests must verify pointer-to-document coordinates after zoom and pan.
- Tests must verify undo restores the previous pixel state after Fill.
- Tests must verify flattened canvas refresh matches active-layer pixels.

## Required Test Shape
- Add a Pixel Design UI automation test that fills a bounded region and asserts changed pixels match foreground color while outside pixels remain unchanged.
- Create a selection, fill inside it, and assert only selected pixels change.
- Click outside an active selection and assert pixels/history/status follow the product-defined no-op behavior.
- Fill near transparent pixels and assert tolerance behavior.
- Zoom and pan, fill at known canvas fractions, and assert mapped document pixels are correct.
- Lock and hide the active layer before filling and assert the product-defined behavior.
- Undo after Fill and assert active-layer pixels, flattened capture, dirty state, and history restore.

## Required Changes
- Gate Fill on layer lock and product-defined visibility rules.
- Move or condition history/status updates so no-op selection fills do not create false history entries.
- Add Fill pixel, selection, tolerance, transparency, zoom/pan, undo, lock, and visibility E2E/state tests.

## Verification
- `cargo test -p tench-pixel-design --test pixel_design_e2e canvas_fill_click`
- `cargo test -p tench-pixel-design`
