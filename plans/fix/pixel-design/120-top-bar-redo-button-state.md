# Top Bar Redo Button State

## Source Plan
- `plans/pixel-design/top-bar-redo-button-work-plan.md`

## Gap Analysis
The Top Bar Redo button is always exposed as a generic enabled automation button, even when no forward snapshot exists. The plan requires disabling itself when no forward snapshot exists, but automation does not expose availability and the click path still calls `redo()` plus `refresh_flattened()`. See `apps/pixel-design/src-tauri/src/ui/mod.rs:981` and `apps/pixel-design/src-tauri/src/ui/mod.rs:232`.

The current E2E coverage uses Top Bar Redo only to restore `history_index` after Top Bar Undo. It does not verify status text, exact document state, canvas refresh, thumbnail refresh, disabled/no-forward behavior, stale redo truncation after a new edit, or layer operation restoration. See `apps/pixel-design/src-tauri/tests/pixel_design_e2e.rs:358`.

History snapshot timing is inconsistent across edit operations, so Redo cannot reliably restore the exact undone state for every canvas and layer operation until the history model is normalized. See `apps/pixel-design/src-tauri/src/ui/state.rs:811`, `apps/pixel-design/src-tauri/src/ui/state.rs:872`, and `apps/pixel-design/src-tauri/src/ui/state.rs:1261`.

## Plan Requirements Not Met
- Top Bar Redo automation must expose disabled/enabled state based on forward history availability.
- Tests must verify Redo restores the exact undone document state, not only `history_index`.
- Tests must verify stale redo history is unavailable after making a new edit.
- Tests must verify Redo restores layer order and active layer selection for layer operations.
- Tests must verify canvas and thumbnails refresh after Redo.

## Required Test Shape
- Create a pixel edit, Undo, click Top Bar Redo, and assert document pixels, status, history index, flattened capture, and thumbnails match the redone state.
- Undo, make a new edit, then assert Top Bar Redo is disabled or no-op and stale redo state is not applied.
- Perform layer add/delete/reorder, Undo, Redo, and assert layer order, active layer id, thumbnails, canvas capture, and status restore.

## Required Changes
- Expose Redo availability in `pd.top.redo` automation metadata and disable or clearly no-op at the boundary.
- Normalize history snapshot timing across edit operations.
- Add Top Bar Redo E2E tests for exact state restoration, stale redo truncation, layer operation restoration, canvas refresh, thumbnail refresh, and disabled state.

## Verification
- `cargo test -p tench-pixel-design --test pixel_design_e2e top_bar_redo`
- `cargo test -p tench-pixel-design`
