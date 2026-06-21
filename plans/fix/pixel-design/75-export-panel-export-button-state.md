# Export Panel Export Button State

## Source Plan
- `plans/pixel-design/export-panel-export-button-work-plan.md`

## Gap Analysis
The Export button sets `pending_file_action = Some(FileAction::Export)` and status says to choose a destination, but `pending_file_action` is not routed to a platform save dialog or equivalent destination picker in the inspected UI code. See `apps/pixel-design/src-tauri/src/ui/mod.rs:575` and `apps/pixel-design/src-tauri/src/ui/state.rs:477`.

`export_document` is callable with a path, but there is no tested flow from the pending export action to a chosen path. The current E2E test only asserts the pending action, not destination selection, file creation, success status, or failure status. See `apps/pixel-design/src-tauri/src/ui/mod.rs:86` and `apps/pixel-design/src-tauri/tests/pixel_design_e2e.rs:648`.

Final export does not use `export_format` or `export_quality` to choose encoder settings. The file is saved through `image.save(path)`, so format follows the path extension and JPEG/WebP quality is ignored. See `apps/pixel-design/src-tauri/src/ui/mod.rs:101`.

Destination cancel behavior is not implemented or tested. There is no path that clears `pending_file_action` after a cancelled export without writing a file or changing document state. See `apps/pixel-design/src-tauri/src/ui/state.rs:452`.

Invalid-path export is not tested. `export_document` sets an error status on save failure, but no test verifies the message is actionable, no file is written, pending state is cleared or retained according to product rules, and dirty state remains unchanged. See `apps/pixel-design/src-tauri/src/ui/mod.rs:106`.

## Plan Requirements Not Met
- Export pending state must be routed to a platform save dialog or equivalent destination picker.
- The chosen destination path must call the export path using the selected format, quality, and scale.
- Export must use `export_format` for encoder/extension selection.
- Export must use `export_quality` for quality-capable formats.
- Destination cancel must write no file and leave document state unchanged.
- Invalid-path export must surface an actionable error without clearing dirty state.
- Tests must verify actual file output for selected format, quality, and scale.

## Required Test Shape
- Click Export and assert pending export state, status text, and destination-picker automation state.
- Complete the destination picker with a temporary path, then assert a file exists, dimensions reflect `export_scale`, format/extension reflect `export_format`, status reports success, and document dirty state is unchanged.
- Export JPEG/WebP with different quality values and assert encoder settings affect the output according to product-defined expectations.
- Cancel the destination picker and assert no file is written, pending state is resolved, and document layers, history, dirty state, and status follow the cancel rule.
- Export to an invalid path and assert actionable error status, no dirty-state clearing, and no document mutation.

## Required Changes
- Connect `FileAction::Export` pending state to the native or testable destination picker flow.
- Route the selected destination to `export_document` and clear pending state consistently on success, cancel, and error.
- Implement format-specific encoder and extension handling from `state.export_format`.
- Apply `state.export_quality` for quality-capable formats.
- Add Export button pending, success, cancel, invalid-path, format, quality, scale, and dirty-state tests.

## Verification
- `cargo test -p tench-pixel-design --test pixel_design_e2e export_panel_export`
- `cargo test -p tench-pixel-design`
