# Deliver Inspector Tab State Fix Plan

## Source Plan

- `plans/composer/deliver-inspector-tab-button-work-plan.md`

## Gap Analysis

The Deliver inspector tab is clicked in E2E, but tests only assert export
controls are present. They do not verify `active_inspector_tab`, top mode sync,
no-op behavior, playback continuation, or project-data invariants.

## Plan Requirements Not Met

- There is no E2E assertion that clicking `composer.inspector.deliver` sets
  `active_inspector_tab == 3` and `mode == ComposerMode::Deliver`.
- There is no test that the top Deliver mode highlight syncs after clicking the
  inspector Deliver tab.
- There is no test that clicking Deliver while already active leaves project
  data, selection, playhead, render queue, and export settings unchanged.
- There is no test that Deliver behavior with no selected clip avoids stale
  Edit/Color/Audio inspector content.
- There is no test that playback continues after clicking Deliver during
  playback.
- Automation does not expose active inspector tab value, active top mode value,
  or Deliver tab content state.

## Code Review

- `apps/composer/src-tauri/src/ui/right_panel.rs:43` renders active tab styling
  from `active_inspector_tab`.
- `apps/composer/src-tauri/src/ui/right_panel.rs:71` registers each inspector
  tab with `ClickAction::SelectInspectorTab(i)`.
- `apps/composer/src-tauri/src/ui/right_panel.rs:99` renders Deliver content
  when `active_inspector_tab == 3`.
- `apps/composer/src-tauri/src/ui/state.rs:550` updates
  `active_inspector_tab`.
- `apps/composer/src-tauri/src/ui/state.rs:556` maps index `3` to
  `ComposerMode::Deliver`.

## Test Review

- `apps/composer/src-tauri/tests/plan_ui_e2e.rs:298` clicks
  `composer.inspector.deliver`.
- `apps/composer/src-tauri/tests/plan_ui_e2e.rs:299` through `:307` assert
  Deliver export controls are present.
- Existing coverage does not assert state sync, active top mode, no-op
  invariants, or playback state.

## Required Test Shape

- Click `composer.inspector.deliver` from another inspector tab and assert
  `active_inspector_tab`, `mode`, active top mode selector, and Deliver content.
- Click Deliver again and assert a snapshot of project data, selected clip,
  playhead, render queue state, and export settings is unchanged.
- Run the click with no selected clip and assert stale non-Deliver controls are
  absent.
- Start playback, click Deliver, and assert `is_playing` and shuttle state
  remain unchanged.

## Required Changes

- Expose active inspector tab, active mode, and Deliver tab content state
  through automation.
- Add focused E2E coverage for Deliver tab selection and invariants.
- Keep `select_inspector_tab` as the single state transition for inspector tab
  clicks.

## Verification

- `cargo test -p tench-composer deliver_inspector_tab`
- `cargo test -p tench-composer composer_plan_inspector_render_queue_ai_and_context_menu_use_real_events_ui_e2e`
- `cargo test -p tench-ui-test ui_automation`
- `git diff --check`
