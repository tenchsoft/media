# Color Mode Tab State Fix Plan

## Source Plan

- `plans/composer/color-mode-tab-button-work-plan.md`

## Gap Analysis

The Color top mode tab is registered and `select_mode` synchronizes the
inspector index, but the top-toolbar click path is not covered. Tests only check
that the selector exists.

## Plan Requirements Not Met

- There is no E2E test that clicking `composer.mode.color` sets
  `mode == ComposerMode::Color` and `active_inspector_tab == 1`.
- There is no test that the Color mode highlight and Color inspector content
  update together after the top mode click.
- There is no test that Color mode with no selected clip shows the expected
  empty color content instead of stale Edit inspector controls.
- There is no test that Color mode with video effects present keeps those
  color-related effects visible.
- There is no test that repeated Color clicks do not push undo/history
  snapshots or mutate project data.
- There is no test that switching from Deliver to Color leaves render queue
  state unchanged.
- Automation does not expose active top mode, active inspector tab, Color
  content state, or render queue visibility as values.

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
- `apps/composer/src-tauri/src/ui/right_panel.rs:331` renders Color inspector
  content.

## Test Review

- `apps/composer/src-tauri/tests/plan_ui_e2e.rs:77` only asserts
  `composer.mode.color` is present.
- Existing state unit coverage checks `select_mode(ComposerMode::Color)`, but
  not the top toolbar click path or invariants.

## Required Test Shape

- Click `composer.mode.color` from another mode and assert `mode`,
  `active_inspector_tab`, active top mode, and Color content.
- Clear clip selection before clicking Color and assert Edit inspector controls
  are absent while the Color empty state is visible.
- Add a video effect, click Color, and assert the Color tab lists it.
- Click Color repeatedly and assert undo stack, project settings, selection,
  playhead, and render queue state remain unchanged.
- Open Deliver/render queue state, switch to Color, and assert render queue
  visibility and jobs are preserved.

## Required Changes

- Expose active mode, active inspector tab, Color content state, and render
  queue visibility through automation.
- Add focused E2E coverage for Color top mode selection and invariants.
- Keep mode selection free of project-data mutation and undo pushes.

## Verification

- `cargo test -p tench-composer color_mode_tab`
- `cargo test -p tench-composer composer_plan_project_timeline_controls_use_real_events_ui_e2e`
- `cargo test -p tench-ui-test ui_automation`
- `git diff --check`
