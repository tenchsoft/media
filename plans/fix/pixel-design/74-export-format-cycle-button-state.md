# Export Format Cycle Button State

## Source Plan
- `plans/pixel-design/export-format-cycle-button-work-plan.md`

## Gap Analysis
`cycle_export_format` cycles `PNG -> JPEG -> WebP -> BMP -> PNG` and updates status, but the E2E coverage only clicks once and asserts the value changed. It does not verify the full order or wraparound. See `apps/pixel-design/src-tauri/src/ui/state.rs:1210` and `apps/pixel-design/src-tauri/tests/pixel_design_e2e.rs:639`.

The export format automation node is labeled only `Format` and does not expose the displayed format value. Tests can inspect internal state, but they cannot verify through the UI tree that the displayed format row changed. See `apps/pixel-design/src-tauri/src/ui/mod.rs:1251` and `apps/pixel-design/src-tauri/src/ui/panels.rs:285`.

Cycling after changing quality and scale is not tested. Existing coverage changes quality and scale after one format click, but does not assert that cycling formats preserves prior quality/scale settings. See `apps/pixel-design/src-tauri/tests/pixel_design_e2e.rs:642`.

Final export does not use `export_format` to choose an encoder or extension. `export_document` calls `image.save(path)`, so the actual encoder is driven by the destination path rather than the displayed format state. See `apps/pixel-design/src-tauri/src/ui/mod.rs:86`.

## Plan Requirements Not Met
- Tests must verify the full PNG, JPEG, WebP, BMP, PNG cycle order.
- Export format automation must expose the currently displayed format value.
- Tests must verify format cycling preserves quality and scale settings.
- Final export must use the same format value shown in the Export panel for encoder and extension selection.
- Tests must verify Export after cycling uses the chosen encoder/extension.

## Required Test Shape
- Click the format row repeatedly and assert state value, displayed automation value, and status for PNG, JPEG, WebP, BMP, and wraparound to PNG.
- Change quality and scale, cycle through formats, and assert both settings remain unchanged.
- Export after selecting each format and assert the pending export/default path or saved file extension and encoder match `export_format`.

## Required Changes
- Expose current export format in the `pd.export.format` automation node.
- Route export destination defaults and `export_document` encoder selection through `state.export_format`.
- Add Export Format cycle order, settings preservation, display value, status, and encoder/extension tests.

## Verification
- `cargo test -p tench-pixel-design --test pixel_design_e2e export_format_cycle`
- `cargo test -p tench-pixel-design`
