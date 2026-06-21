# Export Bitrate Field State

## Source Plan

- `plans/composer/export-bitrate-field-control-work-plan.md`

## Gap Analysis

The export bitrate field is currently a direct increment action, not a confirmed option edit. The Deliver panel maps the bitrate row to `SetExportBitrate(settings.bitrate_kbps.saturating_add(1000))`, so clicking the field mutates the setting immediately and provides no editor, option choice, or cancel path. See `apps/composer/src-tauri/src/ui/right_panel.rs:461`.

The action handler writes the incoming value directly into `project.export_settings.bitrate_kbps` without validating it against export format, codec, resolution, or FPS constraints. See `apps/composer/src-tauri/src/ui/mod.rs:424` and `crates/composer-core/src/project.rs:32`.

The current E2E coverage only asserts that `composer.export.bitrate` exists. It changes the format field and queues an export, but does not interact with bitrate, verify cancel preservation, verify incompatible-value handling, or assert that the queued render job uses the exact displayed bitrate. See `apps/composer/src-tauri/tests/plan_ui_e2e.rs:298`.

## Plan Requirements Not Met

- Activating `composer.export.bitrate` must open an option editor or cycle through a defined valid bitrate set instead of blindly adding `1000`.
- A confirmed bitrate choice must be validated against the export contract before it is written to `project.export_settings`.
- Canceling the bitrate edit must preserve the previous bitrate.
- Incompatible bitrate choices must either show an actionable constraint message or adjust dependent export settings through the same validated path.
- Queueing an export after changing bitrate must use the exact bitrate displayed in the Deliver panel.

## Required Test Shape

- Add a Composer UI automation test that enters Deliver mode, activates `composer.export.bitrate`, chooses a valid bitrate, and asserts the visible field or automation node value changed.
- In the same flow, open the bitrate editor again, cancel it, and assert the previous bitrate value is preserved.
- Cover one incompatible bitrate combination and assert either the constraint notice or the dependent setting adjustment.
- Queue an export and assert the render job settings contain the displayed bitrate value.

## Required Changes

- Replace the direct `SetExportBitrate(current + 1000)` click path with a bounded bitrate editor or a deterministic cycle over valid bitrate options.
- Add shared export-setting validation that includes bitrate compatibility with format, codec, resolution, and FPS.
- Commit bitrate changes to `project.export_settings` only after confirmation or a validated cycle step.
- Expose enough automation state for tests to compare the displayed bitrate with the queued render job bitrate.

## Verification

- `cargo test -p tench-composer --test plan_ui_e2e export_bitrate`
- `cargo test -p tench-composer`
