# Layer Row Visibility Toggle State

## Source Plan
- `plans/pixel-design/layer-row-visibility-toggle-work-plan.md`

## Gap Analysis
The visibility toggle flips `layer.visible` and updates status, but the automation node is labeled only `Layer N Visibility` and does not expose the current visible/hidden value. Tests cannot verify the row icon state through stable UI metadata. See `apps/pixel-design/src-tauri/src/ui/mod.rs:1083` and `apps/pixel-design/src-tauri/src/ui/layers.rs:101`.

Toggling visibility does not mark the document dirty or create history, so visibility metadata may not be persisted or undoable according to the document history model. See `apps/pixel-design/src-tauri/src/ui/state.rs:679`.

The current E2E coverage clicks one visibility toggle and only asserts the flag becomes false. It does not verify status text, visible indicator rendering, canvas recomposition, layer pixel data preservation, toggling back on, opacity/offset preservation, active layer stability, dirty state, or history behavior. See `apps/pixel-design/src-tauri/tests/pixel_design_e2e.rs:288`.

## Plan Requirements Not Met
- Layer visibility automation must expose the current visible/hidden state.
- Visibility metadata must participate in dirty/history behavior or have an explicit non-undoable rule.
- Tests must verify hidden layer pixels disappear from the composited canvas while remaining in layer data.
- Tests must verify toggling the same layer visible again restores pixels at the same opacity and offset.
- Tests must verify toggling visibility on the active layer does not move active selection.

## Required Test Shape
- Toggle visibility off and assert visible state, automation value, status, dirty/history behavior, and row indicator.
- Compare canvas/flattened capture before and after hiding a layer to assert its pixels disappear from the composite while layer buffer data remains.
- Toggle visibility back on and assert the same pixels reappear with unchanged opacity and offset.
- Toggle visibility on the active layer and assert `active_layer_id` and active row metadata remain unchanged.

## Required Changes
- Expose visible/hidden state in `pd.layer.visibility.{idx}` automation nodes.
- Define and implement dirty/history behavior for visibility metadata.
- Add Layer Visibility E2E tests for indicator state, composite refresh, data preservation, toggle-back restoration, active selection stability, history, and dirty state.

## Verification
- `cargo test -p tench-pixel-design --test pixel_design_e2e layer_row_visibility_toggle`
- `cargo test -p tench-pixel-design`
