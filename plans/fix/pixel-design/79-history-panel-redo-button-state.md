# History Panel Redo Button State

## Source Plan
- `plans/pixel-design/history-panel-redo-button-work-plan.md`

## Gap Analysis
History rows are rendered with active styling, but automation step nodes do not expose which row is active. Tests cannot verify that Redo moves the highlighted row forward through the UI tree. See `apps/pixel-design/src-tauri/src/ui/layers.rs:253` and `apps/pixel-design/src-tauri/src/ui/mod.rs:1155`.

The History Redo button is always exposed as a generic enabled button, even when no forward snapshot exists. Clicking it in that state produces no status or disabled-state acknowledgement, and tests cannot verify Redo availability from automation metadata. See `apps/pixel-design/src-tauri/src/ui/mod.rs:1150` and `apps/pixel-design/src-tauri/src/ui/state.rs:1252`.

The current E2E coverage clicks History Undo, asserts the index decreased, clicks History Redo, then immediately clicks a history step without asserting Redo's result, status, active row, canvas refresh, or thumbnail refresh. See `apps/pixel-design/src-tauri/tests/pixel_design_e2e.rs:343`.

Redo when no forward snapshot exists is not tested. The plan requires no state changes in that case, but there is no assertion covering history index, document state, canvas cache, thumbnails, status, or dirty state. See `apps/pixel-design/src-tauri/src/ui/state.rs:1253`.

Redo after a crop operation is not tested. There is no coverage proving redone dimensions and canvas render match the cropped snapshot. See `apps/pixel-design/src-tauri/tests/pixel_design_e2e.rs:343`.

## Plan Requirements Not Met
- History row automation must expose active/selected row state.
- Redo button automation must expose enabled/disabled availability or equivalent no-forward state.
- Tests must verify History Redo moves the active row forward.
- Tests must verify Redo with no forward snapshot leaves state unchanged.
- Tests must verify Redo after crop restores cropped dimensions and rendered canvas.
- Tests must verify canvas and thumbnails refresh after Redo.

## Required Test Shape
- Create a history stack, undo once, click `pd.history.redo`, then assert history index, active row metadata, status text, canvas capture, and thumbnail capture.
- Click Redo again with no forward snapshot and assert history index, document, dirty state, status, flattened cache, thumbnails, and visible capture follow the no-op rule.
- Perform Crop, Undo, then History Redo, and assert document dimensions, flattened capture, active history row, and thumbnails match the redone snapshot.

## Required Changes
- Expose active state on history step automation nodes.
- Expose Redo availability through the `pd.history.redo` automation node.
- Add History Redo E2E tests for active row movement, no-forward no-op behavior, crop redo, canvas refresh, and thumbnail refresh.

## Verification
- `cargo test -p tench-pixel-design --test pixel_design_e2e history_panel_redo`
- `cargo test -p tench-pixel-design`
