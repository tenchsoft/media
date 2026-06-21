# Automatic Empty State Rendering Fix Plan

## Source Plan

- `plans/composer/automatic-empty-state-rendering-behavior-work-plan.md`

## Gap Analysis

Composer paints empty-state text for media and inspector panels, but automation
does not expose those empty states and tests do not cover data absence or state
transitions after import, selection, deletion, or project load.

## Plan Requirements Not Met

- Media empty state is not exposed through automation.
- Edit, Color, and Audio inspector empty states are not exposed through
  automation.
- Tests do not cover an empty media bin.
- Tests do not cover no selected clip/track inspector empty states.
- Tests do not assert that empty states disappear after import or selection.
- Tests do not assert that empty states return after deletion or project load.

## Code Review

- `apps/composer/src-tauri/src/ui/left_panel.rs:240` paints `No media imported`
  when `media_bin` is empty.
- `apps/composer/src-tauri/src/ui/inspector.rs:36` defines mode-specific
  inspector empty messages.
- `apps/composer/src-tauri/src/ui/right_panel.rs:124` branches Edit inspector
  rendering from `selected_clip()`.
- `apps/composer/src-tauri/src/ui/right_panel.rs:214` branches Audio inspector
  rendering from `selected_track()`.
- `apps/composer/src-tauri/src/ui/mod.rs:1301` exposes media asset nodes, but no
  media empty-state node.
- `apps/composer/src-tauri/src/ui/mod.rs:1348` exposes only a generic automatic
  empty-state status node.

## Test Review

- `apps/composer/src-tauri/tests/plan_ui_e2e.rs:88` asserts
  `composer.media.asset.0` exists in the default populated state.
- `apps/composer/src-tauri/tests/plan_ui_e2e.rs:264` asserts clip inspector
  controls after selecting a clip.
- There is no E2E path for empty media, empty Edit inspector, empty Audio
  inspector, deletion cleanup, or project-load cleanup.

## Required Test Shape

- Start Composer with an empty media bin and assert a stable
  `composer.media.empty` node with the expected message.
- Start with no selected clip in Edit mode and assert a stable
  `composer.inspector.edit.empty` node.
- Switch to Audio mode with no selected track and assert
  `composer.inspector.audio.empty`.
- Import media and assert `composer.media.empty` disappears.
- Select a clip and assert inspector empty nodes disappear and real controls
  appear.
- Delete the selected clip or load a project and assert the appropriate empty
  states return without stale controls.

## Required Changes

- Add stable automation nodes for media and inspector empty states with message
  values.
- Add E2E coverage for empty-to-populated and populated-to-empty transitions.
- Ensure stale inspector controls are absent whenever `selected_clip()` or
  `selected_track()` returns `None`.

## Verification

- `cargo test -p tench-composer automatic_empty_state_rendering`
- `cargo test -p tench-composer composer_plan_left_panel_templates_effects_transitions_and_subtitles_use_real_events_ui_e2e`
- `cargo test -p tench-composer composer_plan_inspector_render_queue_ai_and_context_menu_use_real_events_ui_e2e`
- `cargo test -p tench-ui-test ui_automation`
- `git diff --check`
