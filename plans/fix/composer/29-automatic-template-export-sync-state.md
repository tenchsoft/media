# Automatic Template Export Sync State Fix Plan

## Source Plan

- `plans/composer/automatic-template-export-sync-behavior-work-plan.md`

## Gap Analysis

Template selection updates timeline and export settings, but preview geometry is
still fixed at `16:9`. Built-in vertical and square templates therefore cannot
make the preview reflect the selected resolution. Deliver-field and invalid
index behavior also lack value-based verification.

## Plan Requirements Not Met

- Preview dimensions do not reflect selected template resolution for non-16:9
  templates.
- There is no test asserting that selecting a different-fps template updates
  both timeline fps and export fps.
- There is no test asserting that selecting a different-resolution template
  updates timeline dimensions, export dimensions, and preview aspect geometry.
- There is no test asserting that Deliver fields display the selected
  template's export resolution and fps after switching to Deliver.
- There is no test asserting that an invalid template index leaves timeline,
  export settings, selected template, and notice state unchanged.
- Automation does not expose template metadata or Deliver field values needed
  for selector-based assertions.

## Code Review

- `crates/composer-core/src/project.rs:172` defines a vertical `1080x1920`
  built-in template.
- `crates/composer-core/src/project.rs:180` defines a square `1080x1080`
  built-in template.
- `apps/composer/src-tauri/src/ui/preview.rs:5` hardcodes preview aspect ratio
  to `16:9`.
- `apps/composer/src-tauri/src/ui/preview_panel.rs:27` computes the monitor rect
  without using `state.project.timeline.width` or `height`.
- `apps/composer/src-tauri/src/ui/state.rs:561` validates the template index
  before mutating state.
- `apps/composer/src-tauri/src/ui/right_panel.rs:419` renders Deliver settings
  from `project.export_settings`, but automation does not expose the rendered
  field values.

## Test Review

- `apps/composer/src-tauri/tests/plan_ui_e2e.rs:204` opens Templates and clicks
  `composer.template.0`.
- `apps/composer/src-tauri/tests/plan_ui_e2e.rs:207` only checks capture
  changes after template selection.
- `apps/composer/src-tauri/tests/plan_ui_e2e.rs:298` opens Deliver and checks
  export control selector presence, not displayed resolution or fps values.

## Required Test Shape

- Add state tests for selecting fps, resolution, and invalid-index templates.
- Add an E2E test that selects the vertical template, switches to Deliver, and
  asserts export resolution and fps values through automation.
- Add an E2E or render helper test that verifies preview monitor aspect geometry
  changes when selecting vertical and square templates.
- Assert the success notice after a valid template and unchanged state after an
  invalid template index.

## Required Changes

- Compute preview monitor aspect ratio from timeline width and height instead
  of a fixed constant.
- Expose automation values for template rows and Deliver fields, including
  width, height, fps, and selected state.
- Keep `select_template` as the single place that mutates timeline and export
  template-derived settings.

## Verification

- `cargo test -p tench-composer automatic_template_export_sync`
- `cargo test -p tench-composer composer_plan_left_panel_templates_effects_transitions_and_subtitles_use_real_events_ui_e2e`
- `cargo test -p tench-ui-test ui_automation`
- `git diff --check`
