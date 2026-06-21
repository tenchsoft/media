# Layers Panel Move Layer Down Button State

## Source Plan
- `plans/pixel-design/layers-panel-move-layer-down-button-work-plan.md`

## Gap Analysis
The Move Down button is always exposed as a generic enabled automation button, even when the active layer cannot move down. In the boundary state, `move_layer_down` performs no state update and sets no status, while the handler still refreshes flattened output. See `apps/pixel-design/src-tauri/src/ui/mod.rs:1109`, `apps/pixel-design/src-tauri/src/ui/state.rs:710`, and `apps/pixel-design/src-tauri/src/ui/mod.rs:383`.

Layer thumbnails are refreshed after Move Down through `refresh_flattened()`, but automation exposes only a single aggregate thumbnail node. Tests cannot verify that thumbnails remain associated with the reordered layer ids. See `apps/pixel-design/src-tauri/src/ui/mod.rs:53` and `apps/pixel-design/src-tauri/src/ui/mod.rs:1399`.

The current E2E coverage clicks Move Up and Move Down and only asserts status contains `up` or `down`. It does not verify actual layer order, active layer id, composited canvas order, history label/index, boundary behavior, thumbnail updates, or undo restoration. See `apps/pixel-design/src-tauri/tests/pixel_design_e2e.rs:293`.

## Plan Requirements Not Met
- Move Down automation must expose enabled/disabled availability for boundary state.
- Boundary no-op must provide deterministic acknowledgement or disabled state without unnecessary refresh.
- Tests must verify Move Down changes layer order in the product-defined direction.
- Tests must verify Move Down recomposites the canvas according to the new order.
- Tests must verify `Reorder layer` history is recorded only when movement occurs.
- Tests must verify Undo after Move Down restores layer order, active layer, pixels, and thumbnails.

## Required Test Shape
- Create multiple visible layers with distinguishable pixels, click Move Down, and assert layer order, active layer id/index, status, dirty state, history label/index, thumbnails, and flattened capture.
- Click Move Down at the boundary and assert order, active layer, history, dirty state, status/disabled state, thumbnails, and capture follow the no-op rule.
- Undo after Move Down and assert layer order, active layer, pixels, thumbnails, history state, and visible capture restore.

## Required Changes
- Expose Move Down availability in automation.
- Add deterministic boundary acknowledgement or disable the button when movement is unavailable.
- Expose per-layer thumbnail automation metadata.
- Add Move Layer Down E2E tests for order, canvas recomposition, boundary no-op, history, thumbnails, and undo restoration.

## Verification
- `cargo test -p tench-pixel-design --test pixel_design_e2e layers_panel_move_layer_down`
- `cargo test -p tench-pixel-design`
