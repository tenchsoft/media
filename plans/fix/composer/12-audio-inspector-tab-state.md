# Audio Inspector Tab State Fix Plan

## Source Plan

- `plans/composer/audio-inspector-tab-button-work-plan.md`

## Gap Analysis

Clicking the Audio inspector tab switches to Audio content, but the full tab
contract is not verified. Tests do not assert mode synchronization, active tab
styling, empty state, or preservation of unrelated playback and project state.

## Plan Requirements Not Met

- E2E coverage does not assert `active_inspector_tab == 2` and
  `mode == ComposerMode::Audio` after clicking Audio.
- The top Audio mode tab highlight is not verified after selecting the inspector
  tab.
- Clicking Audio while already active is not tested for no project-data changes.
- No-selected-clip empty state is not tested.
- Playback continuation is not tested while clicking the tab during playback.
- Automation does not expose active/selected state for inspector tabs or top mode
  tabs.

## Code Review

- `apps/composer/src-tauri/src/ui/right_panel.rs:70` registers each inspector tab
  with `ClickAction::SelectInspectorTab(i)`.
- `apps/composer/src-tauri/src/ui/state.rs:550` updates both
  `active_inspector_tab` and `mode` in `select_inspector_tab`.
- `apps/composer/src-tauri/src/ui/right_panel.rs:43` renders active tab styling
  from `active_inspector_tab`.
- `apps/composer/src-tauri/src/ui/mod.rs:1413` exposes inspector tab selectors,
  but not selected state or synchronized top-mode state.

## Test Review

- `apps/composer/src-tauri/tests/plan_ui_e2e.rs:285` clicks
  `composer.inspector.audio` and asserts audio controls are present.
- `apps/composer/src-tauri/src/ui/state.rs:1205` unit-tests tab/mode sync for
  Color and Deliver, but not the Audio inspector path through automation.
- There is no no-op active-click, no-selected-clip, or playback-continuation
  test.

## Required Test Shape

- Add an E2E test that clicks `composer.inspector.audio` and asserts
  `active_inspector_tab == 2`, `mode == ComposerMode::Audio`, and the top
  `composer.mode.audio` node is active.
- Click Audio again and assert project data, selection, playhead, and render
  queue state do not change.
- Clear selected clip, click Audio, and assert the expected empty state.
- Start playback, click Audio, and assert playback continues unless the product
  explicitly changes that policy.

## Required Changes

- Extend automation nodes for inspector and mode tabs with selected/active
  state.
- Add state-preservation assertions around inspector tab clicks.
- Add Audio-specific mode sync coverage in both state-level and UI E2E tests.

## Verification

- `cargo test -p tench-composer audio_inspector_tab`
- `cargo test -p tench-composer composer_plan_inspector_render_queue_ai_and_context_menu_use_real_events_ui_e2e`
- `cargo test -p tench-ui-test ui_automation`
- `git diff --check`
