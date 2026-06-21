# Export Resolution Field State

## Source Plan

- `plans/composer/export-resolution-field-control-work-plan.md`

## Gap Analysis

The resolution field does not open an option editor or cycle through supported resolutions. The click action always writes `1280x720`, so activating the field cannot choose among valid export resolutions and may become a no-op after the first click. See `apps/composer/src-tauri/src/ui/right_panel.rs:476`.

The action handler writes width and height directly into `project.export_settings` without validating the resolution against format, codec, FPS, or bitrate compatibility. See `apps/composer/src-tauri/src/ui/mod.rs:415` and `crates/composer-core/src/project.rs:32`.

The current E2E coverage only checks that `composer.export.resolution` exists. It does not activate the resolution field, assert the displayed resolution value, cover incompatible combinations, or verify that the queued render job uses the displayed resolution. See `apps/composer/src-tauri/tests/plan_ui_e2e.rs:298`.

## Plan Requirements Not Met

- Activating `composer.export.resolution` must open a resolution option editor or cycle through a defined valid resolution set.
- Resolution changes must be validated against the supported `ExportFormat`, `VideoCodec`, resolution, FPS, and bitrate contract before storage.
- Incompatible resolution choices must show an actionable constraint message or adjust dependent export settings predictably.
- Queueing an export after changing resolution must use the exact resolution displayed in the Deliver panel.
- If the resolution field is implemented as an option editor instead of a cycle, canceling the editor must preserve the previous resolution.

## Required Test Shape

- Add a Composer UI automation test that enters Deliver mode, activates `composer.export.resolution`, chooses or cycles to a valid resolution, and asserts the visible field or automation node value changed.
- Exercise one incompatible resolution combination and assert either the constraint notice or the dependent setting adjustment.
- Queue an export and assert the render job settings contain the displayed width and height.
- If an editor is used for resolution selection, open it again, cancel it, and assert the previous resolution remains unchanged.

## Required Changes

- Replace the hardcoded `SetExportResolution(1280, 720)` action with a bounded resolution editor or deterministic cycle over supported presets.
- Add shared export-setting validation for resolution compatibility with format, codec, FPS, and bitrate.
- Route resolution changes through that validator before mutating `project.export_settings`.
- Expose enough automation state for tests to compare the displayed resolution with the queued render job resolution.

## Verification

- `cargo test -p tench-composer --test plan_ui_e2e export_resolution`
- `cargo test -p tench-composer`
