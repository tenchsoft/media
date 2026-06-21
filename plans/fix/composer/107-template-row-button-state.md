# Template Row Button State

## Source Plan

- `plans/composer/template-row-button-work-plan.md`

## Gap Analysis

Template rows register `SelectTemplate(index)` and `select_template` updates timeline and export settings, but the current E2E coverage clicks only `composer.template.0` and asserts a capture change. It does not assert `selected_template_idx`, timeline FPS/dimensions, export FPS/resolution, selected highlight, or notice text. See `apps/composer/src-tauri/src/ui/left_panel.rs:267`, `apps/composer/src-tauri/src/ui/state.rs:561`, and `apps/composer/src-tauri/tests/plan_ui_e2e.rs:204`.

Selecting another template afterward is untested, so overwrite behavior and highlight movement are not verified.

The stale-index and existing-clips scenarios are untested.

## Plan Requirements Not Met

- Selecting a 1920x1080 template must be tested to update both timeline and export resolution/FPS.
- Selecting a second template must be tested to overwrite settings and move the selected highlight.
- Clicking a stale template row index must be tested to avoid panic and preserve existing settings.
- Selecting a template while clips exist must be tested to keep clip timing valid in frame units.
- Tests must assert the template-applied notice names the selected template.

## Required Test Shape

- Add a Composer UI automation test that clicks `composer.template.0` and asserts `selected_template_idx`, timeline dimensions/FPS, export dimensions/FPS, and notice.
- Click another template such as `composer.template.2` and assert settings overwrite to that template and the highlight moves.
- Simulate a stale `SelectTemplate` action after removing or shrinking the template list and assert no mutation or panic.
- Create a clip, select a template, and assert clip `timeline_in`, duration, and media frame ranges remain valid.

## Required Changes

- Add the missing template-row scenario tests.
- Expose selected template state, timeline settings, export settings, and notice through automation if current state access is insufficient.
- Add stale-index test hooks only if existing action dispatch cannot simulate the case.

## Verification

- `cargo test -p tench-composer --test plan_ui_e2e template_row`
- `cargo test -p tench-composer`
