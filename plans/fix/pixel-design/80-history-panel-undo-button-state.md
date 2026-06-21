# History Panel Undo Button State

## Source Plan
- `plans/pixel-design/history-panel-undo-button-work-plan.md`

## Gap Analysis
History rows are rendered with active styling, but automation step nodes do not expose which row is active. Tests cannot verify that Undo moves the highlighted row backward through the UI tree. See `apps/pixel-design/src-tauri/src/ui/layers.rs:253` and `apps/pixel-design/src-tauri/src/ui/mod.rs:1155`.

The History Undo button is always exposed as a generic enabled button, even at the earliest snapshot. Clicking it in that state produces no status or disabled-state acknowledgement, and tests cannot verify Undo availability from automation metadata. See `apps/pixel-design/src-tauri/src/ui/mod.rs:1145` and `apps/pixel-design/src-tauri/src/ui/state.rs:1243`.

Some canvas edits create history snapshots before the mutation is committed, while other edits create snapshots after mutation. This makes multi-edit Undo behavior inconsistent and can skip the immediately previous canvas state. See `apps/pixel-design/src-tauri/src/ui/state.rs:811`, `apps/pixel-design/src-tauri/src/ui/state.rs:872`, and `apps/pixel-design/src-tauri/src/ui/state.rs:997`.

Layer metadata changes such as visibility, lock, and opacity mutate state without pushing history. Undo cannot restore those metadata changes even though the plan requires layer metadata restoration. See `apps/pixel-design/src-tauri/src/ui/state.rs:679` and `apps/pixel-design/src-tauri/src/ui/state.rs:774`.

The current E2E coverage clicks History Undo and only asserts `history_index` decreased. It does not verify status, active row, pixels, layer metadata, canvas refresh, thumbnail refresh, earliest-snapshot no-op behavior, or multi-edit restoration. See `apps/pixel-design/src-tauri/tests/pixel_design_e2e.rs:343`.

## Plan Requirements Not Met
- History row automation must expose active/selected row state.
- Undo button automation must expose enabled/disabled availability or equivalent earliest-snapshot state.
- History snapshots must consistently represent committed document states.
- Layer visibility, lock, and opacity changes must participate in history or have an explicit non-undoable rule.
- Tests must verify History Undo moves the active row backward.
- Tests must verify Undo at the earliest snapshot selects no invalid index and leaves state unchanged.
- Tests must verify Undo restores both canvas pixels and layer metadata.
- Tests must verify canvas and thumbnails refresh after Undo.

## Required Test Shape
- Create multiple canvas edits, click `pd.history.undo`, and assert history index, active row metadata, status, pixels, flattened capture, and thumbnails.
- Change layer visibility, lock, and opacity, then Undo and assert metadata, history row, canvas capture, and thumbnails restore according to the product-defined rule.
- Click Undo at the earliest snapshot and assert history index, document, dirty state, status, flattened cache, thumbnails, and visible capture follow the no-op rule.

## Required Changes
- Expose active state on history step automation nodes.
- Expose Undo availability through the `pd.history.undo` automation node.
- Normalize history snapshot timing so committed document states undo one step at a time.
- Add history support or explicit non-undoable behavior for layer visibility, lock, and opacity.
- Add History Undo E2E tests for active row movement, earliest no-op behavior, multi-edit canvas restoration, layer metadata restoration, canvas refresh, and thumbnail refresh.

## Verification
- `cargo test -p tench-pixel-design --test pixel_design_e2e history_panel_undo`
- `cargo test -p tench-pixel-design`
