# Deliver Mode Tab State Fix Plan

## Source Plan

- `plans/composer/deliver-mode-tab-button-work-plan.md`

## Gap Analysis

The Deliver top mode tab is registered, but the top-toolbar click path is not
covered. Existing E2E clicks the Deliver inspector tab, not `composer.mode.deliver`.

## Plan Requirements Not Met

- There is no E2E test that clicking `composer.mode.deliver` sets
  `mode == ComposerMode::Deliver` and `active_inspector_tab == 3`.
- There is no test that export settings become visible after clicking the top
  Deliver mode tab.
- There is no test that template-derived export resolution and fps are displayed
  after selecting a template and then clicking Deliver mode.
- There is no test that switching to Deliver while a render job is queued keeps
  the queue available.
- There is no test that repeated Deliver mode clicks do not enqueue render jobs
  or mutate render queue state.
- There is no small-width test proving `composer.mode.deliver` remains
  independently clickable.
- Automation does not expose active top mode, active inspector tab, displayed
  export values, or render queue visibility as values.

## Code Review

- `apps/composer/src-tauri/src/ui/toolbar.rs:41` renders active top mode styling
  from `state.mode`.
- `apps/composer/src-tauri/src/ui/toolbar.rs:60` registers
  `ClickAction::SelectMode(mode)` for each top mode tab.
- `apps/composer/src-tauri/src/ui/mod.rs:215` dispatches top mode clicks to
  `select_mode`.
- `apps/composer/src-tauri/src/ui/state.rs:541` sets `mode`.
- `apps/composer/src-tauri/src/ui/state.rs:543` synchronizes
  `active_inspector_tab` from the selected mode.
- `apps/composer/src-tauri/src/ui/right_panel.rs:395` renders Deliver export
  settings from `project.export_settings`.

## Test Review

- `apps/composer/src-tauri/tests/plan_ui_e2e.rs:79` only asserts
  `composer.mode.deliver` is present.
- `apps/composer/src-tauri/tests/plan_ui_e2e.rs:298` clicks the inspector
  Deliver tab, not the top mode tab.
- Existing coverage does not assert top Deliver mode state sync, template value
  display, render queue invariants, or small-width hit behavior.

## Required Test Shape

- Click `composer.mode.deliver` from another mode and assert `mode`,
  `active_inspector_tab`, active top mode, and export controls.
- Select a template with distinct export settings, click Deliver mode, and
  assert displayed resolution and fps match `project.export_settings`.
- Queue a render job, click Deliver mode, and assert queue visibility/jobs
  remain unchanged.
- Click Deliver repeatedly and assert render queue length does not change.
- Run a narrow viewport capture and click `composer.mode.deliver` by selector,
  asserting it remains independently clickable.

## Required Changes

- Expose active mode, active inspector tab, displayed export setting values, and
  render queue state through automation.
- Add focused E2E coverage for Deliver top mode selection and invariants.
- Keep mode selection free of render enqueueing and project-data mutation.

## Verification

- `cargo test -p tench-composer deliver_mode_tab`
- `cargo test -p tench-composer composer_plan_project_timeline_controls_use_real_events_ui_e2e`
- `cargo test -p tench-ui-test ui_automation`
- `git diff --check`
