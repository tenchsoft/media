# Edit Mode Tab State Fix Plan

## Source Plan

- `plans/composer/edit-mode-tab-button-work-plan.md`

## Gap Analysis

The Edit top mode tab is registered, but the top-toolbar click path is not
covered. Mode labels are also hardcoded through static strings rather than a
product i18n layer.

## Plan Requirements Not Met

- There is no E2E test that clicking `composer.mode.edit` sets
  `mode == ComposerMode::Edit` and `active_inspector_tab == 0`.
- There is no test that clicking Edit while already active leaves current clip
  selection, playhead, timeline, and project data unchanged.
- There is no test that export settings remain stored after switching back to
  Edit.
- There is no narrow-window test proving `composer.mode.edit` remains
  independently clickable.
- Mode tab labels are hardcoded static strings instead of a shared UI/i18n path.
- Automation does not expose active top mode, active inspector tab, selected
  clip id, playhead frame, or export setting values as assertions.

## Code Review

- `apps/composer/src-tauri/src/ui/toolbar.rs:41` renders active top mode styling
  from `state.mode`.
- `apps/composer/src-tauri/src/ui/toolbar.rs:60` registers
  `ClickAction::SelectMode(mode)` for each top mode tab.
- `apps/composer/src-tauri/src/ui/mod.rs:215` dispatches top mode clicks to
  `select_mode`.
- `apps/composer/src-tauri/src/ui/state.rs:21` returns hardcoded mode labels.
- `apps/composer/src-tauri/src/ui/state.rs:541` sets `mode`.
- `apps/composer/src-tauri/src/ui/state.rs:543` synchronizes
  `active_inspector_tab` from the selected mode.

## Test Review

- `apps/composer/src-tauri/tests/plan_ui_e2e.rs:76` only asserts
  `composer.mode.edit` is present.
- Existing E2E does not click the Edit top mode tab or assert state sync,
  selection preservation, export setting preservation, or narrow-window hit
  behavior.

## Required Test Shape

- Switch to another mode, click `composer.mode.edit`, and assert `mode`,
  `active_inspector_tab`, active top mode, and Edit inspector content.
- Select a clip, click Edit while already active, and assert selection,
  playhead, timeline data, and undo stack remain unchanged.
- Modify export settings, switch to another mode, click Edit, and assert export
  settings remain stored.
- Run a narrow viewport capture and click `composer.mode.edit` by selector,
  asserting it remains independently clickable.

## Required Changes

- Expose active mode, active inspector tab, selected clip id, playhead frame,
  and export setting values through automation.
- Add focused E2E coverage for Edit top mode selection and invariants.
- Route mode tab labels through the shared i18n system when the Composer i18n
  layer is available.

## Verification

- `cargo test -p tench-composer edit_mode_tab`
- `cargo test -p tench-composer composer_plan_project_timeline_controls_use_real_events_ui_e2e`
- `cargo test -p tench-ui-test ui_automation`
- `git diff --check`
