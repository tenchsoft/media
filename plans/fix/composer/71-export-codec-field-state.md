# Export Codec Field State

## Source Plan

- `plans/composer/export-codec-field-control-work-plan.md`

## Gap Analysis

The codec field cycles through every `VideoCodec` variant without validating whether the selected codec is compatible with the current export format, resolution, FPS, or bitrate. The Deliver panel maps the codec row directly to the next enum variant, including combinations such as WebM with ProRes. See `apps/composer/src-tauri/src/ui/right_panel.rs:469` and `crates/composer-core/src/project.rs:92`.

The action handler writes the chosen codec straight into `project.export_settings.codec` and only sets a generic notice. There is no export-setting validation layer that can reject incompatible codecs or adjust dependent fields predictably. See `apps/composer/src-tauri/src/ui/mod.rs:411`.

The current E2E coverage only checks that `composer.export.codec` is present. It changes the format field, queues an export, and never asserts codec selection behavior or whether the queued render job uses the displayed codec. See `apps/composer/src-tauri/tests/plan_ui_e2e.rs:298`.

## Plan Requirements Not Met

- Codec changes must be validated against the supported `ExportFormat`, `VideoCodec`, resolution, FPS, and bitrate contract before storage.
- Incompatible codec choices must show an actionable constraint message or adjust dependent export settings predictably.
- Queueing an export after changing codec must use the exact codec displayed in the Deliver panel.
- If the codec field is implemented as an option editor instead of a cycle, canceling the editor must preserve the previous codec.

## Required Test Shape

- Add a Composer UI automation test that enters Deliver mode, activates `composer.export.codec`, chooses or cycles to a valid codec, and asserts the visible field or automation node value changed.
- Exercise one incompatible codec combination and assert either the constraint notice or the dependent setting adjustment.
- Queue an export and assert the render job settings contain the displayed codec.
- If an editor is used for codec selection, open it again, cancel it, and assert the previous codec value remains unchanged.

## Required Changes

- Add shared export-setting validation for codec compatibility with format, resolution, FPS, and bitrate.
- Route `SetExportCodec` through that validator before mutating `project.export_settings`.
- Keep the UI cycle deterministic, or replace it with a bounded option editor, but reject or normalize unsupported codec combinations through one validated path.
- Expose enough automation state for tests to compare the displayed codec with the queued render job codec.

## Verification

- `cargo test -p tench-composer --test plan_ui_e2e export_codec`
- `cargo test -p tench-composer`
