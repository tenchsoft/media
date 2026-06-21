# Layers Panel Opacity Control State

## Source Plan
- `plans/pixel-design/layers-panel-opacity-control-work-plan.md`

## Gap Analysis
The opacity control updates active-layer opacity and status, but it does not mark the document dirty or create history. Opacity is document layer metadata, so changes may not be persisted or undoable according to the document history model. See `apps/pixel-design/src-tauri/src/ui/state.rs:774`.

The opacity automation node is labeled only `Layer Opacity` and does not expose the displayed percentage. Tests can inspect internal state, but they cannot verify the visible control value or layer-row percentage through the UI tree. See `apps/pixel-design/src-tauri/src/ui/mod.rs:1071` and `apps/pixel-design/src-tauri/src/ui/layers.rs:75`.

The current E2E coverage clicks the left side once and only asserts opacity decreased. It does not verify right-side increase, clamp at 0, clamp at 100, status text, displayed percentage, canvas recomposition, unchanged opacity on other layers, history behavior, or undo restoration. See `apps/pixel-design/src-tauri/tests/pixel_design_e2e.rs:268`.

## Plan Requirements Not Met
- Opacity changes must participate in dirty/history behavior or have an explicit non-undoable rule.
- Layer opacity automation must expose the displayed percentage.
- Tests must verify both decrease and increase interactions.
- Tests must verify opacity clamps at 0 percent and 100 percent.
- Tests must verify changing one selected layer leaves other layers' opacity unchanged.
- Tests must verify the canvas recomposites after opacity changes.
- Tests must verify status and visible layer-list percentages update.

## Required Test Shape
- Click the left and right halves of `pd.layer.opacity` and assert active-layer opacity, displayed automation value, row percentage, status, dirty/history behavior, and flattened capture.
- Drive repeated clicks to 0 percent and 100 percent and assert clamping without underflow or overflow.
- Create multiple layers, select one, change opacity, and assert only that layer changes while the composite updates correctly.
- Undo after an opacity change if opacity is undoable, or assert the explicit non-undoable rule.

## Required Changes
- Define and implement dirty/history behavior for layer opacity metadata.
- Expose opacity percentage in `pd.layer.opacity` and layer-row automation metadata.
- Add Layer Opacity E2E tests for decrease, increase, clamping, selected-layer targeting, status, visible display, canvas recomposition, history, and undo behavior.

## Verification
- `cargo test -p tench-pixel-design --test pixel_design_e2e layers_panel_opacity`
- `cargo test -p tench-pixel-design`
