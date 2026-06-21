# Layers Panel Duplicate Layer Button State

## Source Plan
- `plans/pixel-design/layers-panel-duplicate-layer-button-work-plan.md`

## Gap Analysis
Duplicate clones the active layer as visible with the same opacity and blend settings. For non-opaque or blended layer content, compositing the duplicate above the original can change the visible canvas immediately, which violates the requirement that the canvas remain visually identical until the duplicate is edited. See `apps/pixel-design/src-tauri/src/ui/state.rs:719`.

Duplicate generates the new layer id from `self.document.layers.len()`. After deletions or other reorder operations, that id can collide with an existing `layer_N` id, which would break active-layer identity and per-layer metadata. See `apps/pixel-design/src-tauri/src/ui/state.rs:722`.

Layer thumbnails are refreshed after Duplicate through `refresh_flattened()`, but automation exposes only a single aggregate thumbnail node. Tests cannot verify that the duplicate has its own thumbnail and that existing thumbnails remain associated with the correct layer ids. See `apps/pixel-design/src-tauri/src/ui/mod.rs:53` and `apps/pixel-design/src-tauri/src/ui/mod.rs:1399`.

The current E2E coverage clicks Duplicate and only asserts layer count increased. It does not verify insertion position, duplicate active layer id, copied pixels/metadata, status text, history label/index, canvas visual identity, thumbnail updates, id uniqueness, boundary behavior, or undo restoration. See `apps/pixel-design/src-tauri/tests/pixel_design_e2e.rs:299`.

## Plan Requirements Not Met
- Duplicating a visible layer must preserve the composited canvas visually until the duplicate is edited.
- Duplicate layer ids must be unique after delete/reorder/duplicate sequences.
- Layer thumbnail automation must expose per-layer thumbnail presence.
- Tests must verify duplicate insertion next to the source layer and active-layer reassignment.
- Tests must verify copied pixels and metadata are correct.
- Tests must verify `Duplicate layer` is recorded in history.
- Tests must verify Undo after Duplicate restores layer order, active layer, pixels, and thumbnails.

## Required Test Shape
- Duplicate a layer with representative pixels, opacity, visibility, lock, offset, and blend metadata, then assert copied data, insertion index, active layer id, status, dirty state, history label/index, and thumbnails.
- Duplicate a semi-transparent or otherwise composite-sensitive layer and assert the visible canvas remains unchanged according to the product-defined duplicate behavior.
- Delete and duplicate layers in sequence, then assert all layer ids remain unique and active-layer identity is stable.
- Undo after Duplicate and assert layer order, active layer, pixels, thumbnails, history state, and visible capture restore.

## Required Changes
- Define and implement duplicate behavior that preserves the visible composite until the duplicate is edited.
- Generate duplicate layer ids from a collision-free layer id source.
- Expose per-layer thumbnail automation metadata.
- Add Duplicate Layer E2E tests for insertion, active layer, copied data, visual identity, unique ids, history, thumbnails, and undo restoration.

## Verification
- `cargo test -p tench-pixel-design --test pixel_design_e2e layers_panel_duplicate_layer`
- `cargo test -p tench-pixel-design`
