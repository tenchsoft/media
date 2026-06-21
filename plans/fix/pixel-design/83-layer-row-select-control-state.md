# Layer Row Select Control State

## Source Plan
- `plans/pixel-design/layer-row-select-control-work-plan.md`

## Gap Analysis
Layer row selection updates `document.active_layer_id`, but it does not set a status message. The only acknowledgement is the active row highlight, and row automation nodes do not expose active/selected state or the underlying layer id. See `apps/pixel-design/src-tauri/src/ui/mod.rs:355`, `apps/pixel-design/src-tauri/src/ui/mod.rs:1078`, and `apps/pixel-design/src-tauri/src/ui/layers.rs:89`.

The current E2E coverage adds one layer, clicks row 0, and asserts `active_layer_index() == 0`. It does not verify top, middle, and bottom selection, active row metadata, thumbnail association, hidden-layer selection, locked-layer selection, or that subsequent edits target the selected layer. See `apps/pixel-design/src-tauri/tests/pixel_design_e2e.rs:272`.

Locked layer selection is allowed by the row handler, but destructive tools do not consistently block locked-layer edits after that selection. This leaves the locked-row scenario from the plan unmet until edit tools consult `layer.locked`. See `apps/pixel-design/src-tauri/src/ui/state.rs:921` and `apps/pixel-design/src-tauri/src/ui/state.rs:954`.

## Plan Requirements Not Met
- Layer row automation must expose active/selected state and the layer id.
- Row selection must provide deterministic acknowledgement through status or active-row metadata.
- Tests must verify top, middle, and bottom layer selection.
- Tests must verify subsequent edits target the selected layer.
- Tests must verify hidden layer selection changes active layer while composite visibility remains correct.
- Tests must verify locked layer selection changes active layer while destructive edits remain blocked.

## Required Test Shape
- Create at least three layers, select top, middle, and bottom rows, and assert active layer id/index, active row automation state, and thumbnail association.
- Select a layer, perform a pixel edit, and assert only the selected layer receives the change.
- Hide a layer, select it, and assert active layer changes while the composite remains governed by visibility.
- Lock a layer, select it, attempt a destructive edit, and assert the selected locked layer is not mutated.

## Required Changes
- Expose row selected state and layer id in `pd.layer.row.{idx}` automation nodes.
- Add status or equivalent acknowledgement for row selection if active metadata is not enough.
- Ensure locked-layer edit guards apply after selecting locked rows.
- Add Layer Row Select E2E tests for multi-row selection, hidden rows, locked rows, edit targeting, active row highlight, and thumbnails.

## Verification
- `cargo test -p tench-pixel-design --test pixel_design_e2e layer_row_select`
- `cargo test -p tench-pixel-design`
