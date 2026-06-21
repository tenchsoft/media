# Layers Panel Add Layer Button State

## Source Plan
- `plans/pixel-design/layers-panel-add-layer-button-work-plan.md`

## Gap Analysis
Layer thumbnails are generated after Add Layer through `refresh_flattened()`, but automation exposes only a single aggregate `pd.auto.layer_thumbnail` node, not per-layer thumbnail state. Tests cannot verify that the newly added layer row has its own thumbnail placeholder or thumbnail entry. See `apps/pixel-design/src-tauri/src/ui/mod.rs:53`, `apps/pixel-design/src-tauri/src/ui/mod.rs:1399`, and `apps/pixel-design/src-tauri/src/ui/state.rs:1498`.

The current E2E coverage clicks Add Layer and asserts a second row, visibility toggle, lock toggle, dirty dot, and layer count. It does not verify the new layer became active, status text, history label/index, layer order, thumbnail entry, canvas refresh, or undo restoration. See `apps/pixel-design/src-tauri/tests/pixel_design_e2e.rs:272`.

The boundary scenario mentioned in the plan is not covered. Existing tests do not verify that adjacent layer actions such as Move Up at the top layer or Delete on a one-layer document prevent invalid state before or after Add Layer interactions. See `apps/pixel-design/src-tauri/tests/pixel_design_e2e.rs:293`.

## Plan Requirements Not Met
- Layer thumbnail automation must expose per-layer thumbnail presence.
- Tests must verify the added layer becomes the active layer.
- Tests must verify `Add layer` is recorded in history.
- Tests must verify Add Layer updates thumbnails and canvas state.
- Tests must verify Undo after Add Layer restores layer order, active layer, pixels, and thumbnails.
- Tests must verify related boundary actions do not create invalid layer state.

## Required Test Shape
- Click Add Layer and assert layer count, new layer id/name/order, active layer id/index, status, dirty state, history label/index, per-layer thumbnail, and flattened capture.
- Undo after Add Layer and assert layer count, layer order, active layer, pixels, dirty/history state, thumbnails, and visible capture restore.
- Exercise related boundary controls around the Add flow, such as Move Up on the top layer and Delete on a one-layer document, and assert invalid states are prevented.

## Required Changes
- Expose per-layer thumbnail automation metadata.
- Add Add Layer E2E assertions for active layer, history, thumbnails, canvas refresh, undo restoration, and related boundary-state protection.

## Verification
- `cargo test -p tench-pixel-design --test pixel_design_e2e layers_panel_add_layer`
- `cargo test -p tench-pixel-design`
