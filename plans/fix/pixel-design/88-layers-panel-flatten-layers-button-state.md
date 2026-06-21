# Layers Panel Flatten Layers Button State

## Source Plan
- `plans/pixel-design/layers-panel-flatten-layers-button-work-plan.md`

## Gap Analysis
Layer thumbnails are refreshed after Flatten through `refresh_flattened()`, but automation exposes only a single aggregate thumbnail node. Tests cannot verify that stale per-layer thumbnails are removed and the new flattened background layer has the correct thumbnail. See `apps/pixel-design/src-tauri/src/ui/mod.rs:53` and `apps/pixel-design/src-tauri/src/ui/mod.rs:1399`.

The current E2E coverage clicks Flatten and only asserts layer count is one. It does not verify active layer id/name, status text, history label/index, merged visible pixels, hidden-layer exclusion, canvas refresh, thumbnail updates, dirty state, or undo restoration. See `apps/pixel-design/src-tauri/tests/pixel_design_e2e.rs:302`.

Boundary behavior for Flatten and adjacent layer actions is not covered. The plan mentions invalid-state prevention around layer boundaries, but tests do not assert one-layer Flatten behavior, one-layer Delete prevention, or Move Up at the top layer around the Flatten flow. See `apps/pixel-design/src-tauri/src/ui/state.rs:755`.

## Plan Requirements Not Met
- Layer thumbnail automation must expose flattened-layer thumbnail state and stale-thumbnail removal.
- Tests must verify Flatten merges visible layers into one active background layer.
- Tests must verify hidden layers are excluded from the flattened pixels according to product rules.
- Tests must verify `Flatten` is recorded in history.
- Tests must verify Undo after Flatten restores layer order, active layer, pixels, and thumbnails.
- Tests must verify boundary actions do not create invalid layer state.

## Required Test Shape
- Create multiple layers with distinct pixels, visibility, opacity, and offsets, click Flatten, and assert one background layer, active layer id/name, merged pixels, status, dirty state, history label/index, thumbnails, and flattened capture.
- Include a hidden layer and assert its pixels are not included in the flattened output while the product-defined hidden-layer deletion behavior is verified.
- Undo after Flatten and assert layer order, active layer, pixels, thumbnails, history state, and visible capture restore.
- Exercise related boundary controls around the Flatten flow and assert invalid states are prevented.

## Required Changes
- Expose per-layer thumbnail automation metadata, including the flattened layer.
- Add Flatten Layers E2E tests for merged pixels, hidden-layer behavior, active layer, history, thumbnails, canvas refresh, undo restoration, and boundary-state protection.

## Verification
- `cargo test -p tench-pixel-design --test pixel_design_e2e layers_panel_flatten_layers`
- `cargo test -p tench-pixel-design`
