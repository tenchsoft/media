# Top Bar Undo Button State

## Source Plan
- `plans/pixel-design/top-bar-undo-button-work-plan.md`

## Gap Analysis
The Top Bar Undo control is visually dimmed when no backward history exists, but the automation node is always emitted as a generic enabled button. The click path also still calls `state.undo()` at the boundary, so tests cannot assert the required disabled state through automation metadata. See `apps/pixel-design/src-tauri/src/ui/toolbar.rs:106`, `apps/pixel-design/src-tauri/src/ui/mod.rs:981`, and `apps/pixel-design/src-tauri/src/ui/mod.rs:228`.

The current E2E coverage only asserts that `history_index` decreases after clicking `pd.top.undo`. It does not verify restored pixels, layer list, active layer, status text, canvas capture, thumbnail refresh, boundary disabled/no-op behavior, or that redo remains available with the undone state. See `apps/pixel-design/src-tauri/tests/pixel_design_e2e.rs:358`.

History snapshot timing is inconsistent across edit operations. Brush, fill, shape, move, and gradient actions push history before the document mutation, while crop and many layer operations push after mutation, so Top Bar Undo/Redo cannot be validated against a consistent snapshot contract until the history model is normalized. See `apps/pixel-design/src-tauri/src/ui/state.rs:811`, `apps/pixel-design/src-tauri/src/ui/state.rs:827`, `apps/pixel-design/src-tauri/src/ui/state.rs:874`, `apps/pixel-design/src-tauri/src/ui/state.rs:880`, `apps/pixel-design/src-tauri/src/ui/state.rs:990`, and `apps/pixel-design/src-tauri/src/ui/state.rs:997`.

## Plan Requirements Not Met
- Top Bar Undo automation must expose disabled/enabled state based on backward history availability.
- Tests must verify Undo restores exact pixels, layer list, active layer, history index, and status text.
- Tests must verify Undo at the first history entry is disabled or produces a clearly asserted no-op.
- Tests must verify Undo after crop, brush, and layer changes refreshes the canvas and thumbnails.
- Tests must verify Redo remains available after Top Bar Undo and restores the undone state.

## Required Test Shape
- From the initial document, capture `pd.top.undo` and assert it is disabled or clicking it is a no-op with unchanged document, history index, dirty state, canvas capture, and status.
- Create a brush stroke, click `pd.top.undo`, and assert the affected pixels return to their previous values, status names the undo, canvas capture changes, and `pd.top.redo` becomes available.
- Perform crop and layer add/reorder/delete flows, click `pd.top.undo`, and assert dimensions, layer list, active layer id, thumbnails, flattened capture, and history index restore exactly.

## Required Changes
- Expose Undo availability in `pd.top.undo` automation metadata and make the boundary state testable.
- Normalize history snapshot creation so Undo/Redo around canvas and layer operations uses one consistent before/after contract.
- Add Top Bar Undo E2E coverage for exact document restoration, canvas refresh, thumbnail refresh, boundary behavior, and redo availability.

## Verification
- `cargo test -p tench-pixel-design --test pixel_design_e2e top_bar_undo`
- `cargo test -p tench-pixel-design`
