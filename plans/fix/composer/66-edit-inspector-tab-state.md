# Edit Inspector Tab State Fix Plan

## Source Plan

- `plans/composer/edit-inspector-tab-button-work-plan.md`

## Gap Analysis

The Edit inspector tab is registered, but the click path is not covered by E2E.
Tests only assert the selector is present in the initial UI.

## Plan Requirements Not Met

- There is no E2E test that clicking `composer.inspector.edit` sets
  `active_inspector_tab == 0` and `mode == ComposerMode::Edit`.
- There is no test that the top Edit mode highlight syncs after clicking the
  inspector Edit tab.
- There is no test that clicking Edit while already active leaves project data,
  selection, playhead, render queue, and settings unchanged.
- There is no test that Edit with no selected clip shows the expected empty
  state instead of stale inspector controls.
- There is no test that playback continues after clicking Edit during playback.
- Automation does not expose active inspector tab value, active top mode value,
  or Edit tab empty/content state.

## Code Review

- `apps/composer/src-tauri/src/ui/right_panel.rs:43` renders active tab styling
  from `active_inspector_tab`.
- `apps/composer/src-tauri/src/ui/right_panel.rs:71` registers each inspector
  tab with `ClickAction::SelectInspectorTab(i)`.
- `apps/composer/src-tauri/src/ui/right_panel.rs:77` renders Edit content when
  `active_inspector_tab == 0`.
- `apps/composer/src-tauri/src/ui/state.rs:550` updates
  `active_inspector_tab`.
- `apps/composer/src-tauri/src/ui/state.rs:553` maps index `0` to
  `ComposerMode::Edit`.

## Test Review

- `apps/composer/src-tauri/tests/plan_ui_e2e.rs:89` only asserts
  `composer.inspector.edit` is present.
- Existing coverage does not assert Edit inspector click state sync, active top
  mode, no-op invariants, empty state, or playback state.

## Required Test Shape

- Click `composer.inspector.edit` from another inspector tab and assert
  `active_inspector_tab`, `mode`, active top mode selector, and Edit content.
- Click Edit again and assert a snapshot of project data, selected clip,
  playhead, render queue state, and settings is unchanged.
- Clear selection, click Edit, and assert Edit empty state is exposed while clip
  controls are absent.
- Start playback, click Edit, and assert `is_playing` and shuttle state remain
  unchanged.

## Required Changes

- Expose active inspector tab, active mode, and Edit tab empty/content state
  through automation.
- Add focused E2E coverage for Edit tab selection and invariants.
- Keep `select_inspector_tab` as the single state transition for inspector tab
  clicks.

## Verification

- `cargo test -p tench-composer edit_inspector_tab`
- `cargo test -p tench-composer composer_plan_inspector_render_queue_ai_and_context_menu_use_real_events_ui_e2e`
- `cargo test -p tench-ui-test ui_automation`
- `git diff --check`
