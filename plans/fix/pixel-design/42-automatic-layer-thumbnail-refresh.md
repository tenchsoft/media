# Automatic Layer Thumbnail Refresh

## Source Plan
- `plans/pixel-design/automatic-layer-thumbnail-refresh-work-plan.md`

## Gap Analysis
Layer thumbnails are generated from each layer buffer and refreshed through `refresh_flattened`, but there is no thumbnail dirty/invalidation model. State mutations that bypass `refresh_flattened` can leave the thumbnail cache stale, and `update_all_thumbnails` does not remove cache entries for deleted layers. See `apps/pixel-design/src-tauri/src/ui/state.rs:1477` and `apps/pixel-design/src-tauri/src/ui/mod.rs:53`.

Thumbnail generation reads raw layer pixels only and does not incorporate layer opacity or visibility, even though the plan requires refresh after opacity-visible changes. See `apps/pixel-design/src-tauri/src/ui/state.rs:1478` and `apps/pixel-design/src-tauri/src/ui/layers.rs:112`.

The automation tree exposes a single aggregate `pd.auto.layer_thumbnail` node without per-layer IDs, cache generation, thumbnail dimensions, or pixel/hash values. See `apps/pixel-design/src-tauri/src/ui/mod.rs:1399`.

The current E2E coverage only asserts the thumbnail node is present after a brush stroke and checks layer operations through state/status. It does not verify thumbnail pixels after paint, duplication, deletion, reorder, opacity/visibility changes, file load, flatten, alternate paths, persona switches, or resize. See `apps/pixel-design/src-tauri/tests/pixel_design_e2e.rs:379` and `apps/pixel-design/src-tauri/tests/pixel_design_e2e.rs:268`.

## Plan Requirements Not Met
- Thumbnail cache invalidation must cover every layer preview mutation, including paths outside `refresh_flattened`.
- Deleted layer thumbnails must be removed from the cache.
- Thumbnail output must define and test how opacity and visibility affect previews.
- Automation metadata must expose per-layer thumbnail identity and verifiable content.
- Tests must verify thumbnail refresh after paint, duplicate, delete, reorder, opacity/visibility change, file load, and flatten.
- Tests must verify thumbnails remain correct after persona switches and viewport resize.

## Required Test Shape
- Add a Pixel Design UI automation test that paints on a layer and asserts the active layer thumbnail pixel/hash changes.
- Duplicate and delete layers, then assert per-layer thumbnail nodes match current layer IDs and no deleted thumbnail remains.
- Reorder layers and assert thumbnail content follows the layer, not the row index.
- Toggle visibility and adjust opacity, then assert the product-defined thumbnail preview state updates.
- Load a fixture image and flatten layers, then assert thumbnail cache count/content is correct.
- Switch personas and resize, then assert thumbnail automation metadata and capture remain correct.

## Required Changes
- Add thumbnail cache invalidation or generation tracking tied to layer/document mutations.
- Prune thumbnail cache entries for deleted layers.
- Define opacity/visibility thumbnail semantics and implement them.
- Expose per-layer thumbnail automation nodes with layer ID and content hash/value.
- Add thumbnail refresh E2E and state tests.

## Verification
- `cargo test -p tench-pixel-design --test pixel_design_e2e automatic_layer_thumbnail_refresh`
- `cargo test -p tench-pixel-design`
