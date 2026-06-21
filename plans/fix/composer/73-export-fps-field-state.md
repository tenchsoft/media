# Export FPS Field State

## Source Plan

- `plans/composer/export-fps-field-control-work-plan.md`

## Gap Analysis

The FPS field only toggles between `24.0` and `60.0` and does not validate the selected FPS against format, codec, resolution, or bitrate compatibility. See `apps/composer/src-tauri/src/ui/right_panel.rs:476`.

The action handler writes the FPS directly into `project.export_settings.fps` with no export contract check or dependent-setting adjustment. See `apps/composer/src-tauri/src/ui/mod.rs:420` and `crates/composer-core/src/project.rs:32`.

The current E2E coverage only checks that `composer.export.fps` exists. It does not activate the FPS field, assert the displayed FPS value, cover incompatible combinations, or verify that the queued render job uses the displayed FPS. See `apps/composer/src-tauri/tests/plan_ui_e2e.rs:298`.

## Plan Requirements Not Met

- FPS changes must be validated against the supported `ExportFormat`, `VideoCodec`, resolution, FPS, and bitrate contract before storage.
- Incompatible FPS choices must show an actionable constraint message or adjust dependent export settings predictably.
- Queueing an export after changing FPS must use the exact FPS displayed in the Deliver panel.
- If the FPS field is implemented as an option editor instead of a cycle, canceling the editor must preserve the previous FPS.

## Required Test Shape

- Add a Composer UI automation test that enters Deliver mode, activates `composer.export.fps`, chooses or cycles to a valid FPS, and asserts the visible field or automation node value changed.
- Exercise one incompatible FPS combination and assert either the constraint notice or the dependent setting adjustment.
- Queue an export and assert the render job settings contain the displayed FPS.
- If an editor is used for FPS selection, open it again, cancel it, and assert the previous FPS value remains unchanged.

## Required Changes

- Replace the hardcoded `24.0`/`60.0` toggle with a validated FPS option path, or make that cycle explicitly derive from supported export presets.
- Add shared export-setting validation for FPS compatibility with format, codec, resolution, and bitrate.
- Route `SetExportFps` through that validator before mutating `project.export_settings`.
- Expose enough automation state for tests to compare the displayed FPS with the queued render job FPS.

## Verification

- `cargo test -p tench-composer --test plan_ui_e2e export_fps`
- `cargo test -p tench-composer`
