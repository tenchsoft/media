# Layers Panel Delete Layer Button State

## Source Plan
- `plans/pixel-design/layers-panel-delete-layer-button-work-plan.md`

## Gap Analysis
The Delete button is always exposed as a generic enabled automation button, even when the document has only one layer. The state guard prevents deletion and sets status, but automation does not expose disabled availability for the boundary state. See `apps/pixel-design/src-tauri/src/ui/mod.rs:1099` and `apps/pixel-design/src-tauri/src/ui/state.rs:667`.

Layer thumbnails are refreshed after Delete through `refresh_flattened()`, but automation exposes only a single aggregate thumbnail node. Tests cannot verify that deleted-layer thumbnails are removed and remaining-layer thumbnails match the new layer list. See `apps/pixel-design/src-tauri/src/ui/mod.rs:53` and `apps/pixel-design/src-tauri/src/ui/mod.rs:1399`.

The current E2E coverage duplicates, flattens, adds, deletes, and only asserts the final layer count is one. It does not verify active layer reassignment, status text, history label/index, canvas refresh, thumbnail updates, one-layer delete boundary, or undo restoration. See `apps/pixel-design/src-tauri/tests/pixel_design_e2e.rs:299`.

## Plan Requirements Not Met
- Delete Layer automation must expose enabled/disabled availability for one-layer boundary state.
- Layer thumbnail automation must expose per-layer thumbnail presence after deletion.
- Tests must verify deleting the active layer chooses a valid new active layer.
- Tests must verify `Delete layer` is recorded in history only when deletion occurs.
- Tests must verify one-layer Delete leaves state unchanged and reports the boundary status.
- Tests must verify Undo after Delete restores layer order, active layer, pixels, and thumbnails.

## Required Test Shape
- With multiple layers, delete the active layer and assert layer count, remaining order, active layer id/index, status, dirty state, history label/index, per-layer thumbnails, and flattened capture.
- On a one-layer document, click Delete and assert layer count, active layer, document pixels, dirty/history state, status, and delete-button availability follow the boundary rule.
- Undo after a committed Delete and assert layer order, active layer, pixels, thumbnails, history state, and visible capture restore.

## Required Changes
- Expose Delete button availability in automation.
- Expose per-layer thumbnail automation metadata.
- Add Delete Layer E2E assertions for active layer reassignment, history, thumbnails, one-layer boundary, canvas refresh, and undo restoration.

## Verification
- `cargo test -p tench-pixel-design --test pixel_design_e2e layers_panel_delete_layer`
- `cargo test -p tench-pixel-design`
