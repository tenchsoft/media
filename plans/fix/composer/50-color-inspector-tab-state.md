# Color Inspector Tab State Fix Plan

## Source Plan

- `plans/composer/color-inspector-tab-button-work-plan.md`

## Gap Analysis

The Color inspector tab is registered and the state transition exists, but there
is no Color-tab-specific E2E coverage. Tests only assert the selector is present;
they do not click it or verify mode sync, content, no-op behavior, or transport
invariants.

## Plan Requirements Not Met

- There is no E2E test that clicking `composer.inspector.color` sets
  `active_inspector_tab == 1` and `mode == ComposerMode::Color`.
- There is no test that the top Color mode highlight syncs after clicking the
  inspector Color tab.
- There is no test that clicking Color while already active leaves project data,
  selection, playhead, render queue, and settings unchanged.
- There is no test that the Color tab renders the relevant empty state when no
  color effects are available.
- There is no test that playback continues after clicking Color during
  playback.
- Automation does not expose active inspector tab value, active top mode value,
  or Color tab content state.

## Code Review

- `apps/composer/src-tauri/src/ui/right_panel.rs:43` renders active tab styling
  from `active_inspector_tab`.
- `apps/composer/src-tauri/src/ui/right_panel.rs:71` registers each inspector
  tab with `ClickAction::SelectInspectorTab(i)`.
- `apps/composer/src-tauri/src/ui/right_panel.rs:88` renders Color content when
  `active_inspector_tab == 1`.
- `apps/composer/src-tauri/src/ui/right_panel.rs:359` renders the Color tab
  empty state when no video effects exist.
- `apps/composer/src-tauri/src/ui/state.rs:550` updates
  `active_inspector_tab`.
- `apps/composer/src-tauri/src/ui/state.rs:554` maps index `1` to
  `ComposerMode::Color`.

## Test Review

- `apps/composer/src-tauri/tests/plan_ui_e2e.rs:90` only asserts
  `composer.inspector.color` is present.
- Existing state unit coverage checks `select_mode(ComposerMode::Color)`, but
  not the Color inspector click path or invariants.

## Required Test Shape

- Click `composer.inspector.color` from another inspector tab and assert
  `active_inspector_tab`, `mode`, active top mode selector, and Color content.
- Click Color again and assert a snapshot of project data, selected clip,
  playhead, render queue visibility, and export settings is unchanged.
- Run the click with no selected clip and no color effects, then assert the
  Color empty state is exposed.
- Start playback, click Color, and assert `is_playing` and shuttle state remain
  unchanged.

## Required Changes

- Expose active inspector tab, active mode, and Color tab empty/content state
  through automation.
- Add focused E2E coverage for Color tab selection and invariants.
- Keep `select_inspector_tab` as the single state transition for inspector tab
  clicks.

## Verification

- `cargo test -p tench-composer color_inspector_tab`
- `cargo test -p tench-composer composer_plan_inspector_render_queue_ai_and_context_menu_use_real_events_ui_e2e`
- `cargo test -p tench-ui-test ui_automation`
- `git diff --check`
