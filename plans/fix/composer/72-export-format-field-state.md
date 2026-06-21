# Export Format Field State

## Source Plan

- `plans/composer/export-format-field-control-work-plan.md`

## Gap Analysis

The format field cycles through every `ExportFormat` variant without checking whether the current codec, resolution, FPS, and bitrate remain valid for the selected container. The Deliver panel directly maps the format row to the next enum variant. See `apps/composer/src-tauri/src/ui/right_panel.rs:461` and `crates/composer-core/src/project.rs:60`.

The action handler writes the selected format directly into `project.export_settings.format` without a validation or normalization step for dependent export settings. See `apps/composer/src-tauri/src/ui/mod.rs:407`.

The current E2E coverage clicks `composer.export.format` and asserts the capture changed, but it does not assert the chosen format value, incompatible-combination behavior, or whether the queued render job uses the displayed format. See `apps/composer/src-tauri/tests/plan_ui_e2e.rs:298`.

## Plan Requirements Not Met

- Format changes must be validated against the supported `ExportFormat`, `VideoCodec`, resolution, FPS, and bitrate contract before storage.
- Incompatible format choices must show an actionable constraint message or adjust dependent export settings predictably.
- Queueing an export after changing format must use the exact format displayed in the Deliver panel.
- If the format field is implemented as an option editor instead of a cycle, canceling the editor must preserve the previous format.

## Required Test Shape

- Extend the Composer UI automation coverage to activate `composer.export.format`, choose or cycle to a valid format, and assert the visible field or automation node value changed to the expected format.
- Exercise one incompatible format combination and assert either the constraint notice or the dependent setting adjustment.
- Queue an export and assert the render job settings contain the displayed format.
- If an editor is used for format selection, open it again, cancel it, and assert the previous format value remains unchanged.

## Required Changes

- Add shared export-setting validation for format compatibility with codec, resolution, FPS, and bitrate.
- Route `SetExportFormat` through that validator before mutating `project.export_settings`.
- Keep the UI cycle deterministic, or replace it with a bounded option editor, but reject or normalize unsupported format combinations through one validated path.
- Expose enough automation state for tests to compare the displayed format with the queued render job format.

## Verification

- `cargo test -p tench-composer --test plan_ui_e2e export_format`
- `cargo test -p tench-composer`
