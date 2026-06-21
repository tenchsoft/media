# Automatic Clip Selection Highlight State Fix Plan

## Source Plan

- `plans/composer/automatic-clip-selection-highlight-behavior-work-plan.md`

## Gap Analysis

Clip selection highlighting is painted from `selected_clip_id`, but the behavior
is only lightly tested. The tests do not prove old highlights disappear, deleted
clips clear selection, or loaded projects start without stale selection.

## Plan Requirements Not Met

- Selecting a second clip is not tested for removing the old clip highlight.
- Deleting the selected clip is not tested for removing
  `composer.timeline.clip.selected`.
- Loading a project is not tested for clearing selection and stale highlights.
- Automation exposes one generic `composer.timeline.clip.selected` node without
  a selected clip id/value.
- Tests do not assert inspector content remains derived from `selected_clip()`
  after selection changes.

## Code Review

- `apps/composer/src-tauri/src/ui/timeline_panel.rs:338` paints the selection
  stroke when `selected_clip_id == Some(*clip_id)`.
- `apps/composer/src-tauri/src/ui/mod.rs:218` sets `selected_clip_id` from the
  clicked clip id.
- `apps/composer/src-tauri/src/ui/state.rs:805` and `:819` clear
  `selected_clip_id` after delete success.
- `apps/composer/src-tauri/src/ui/state.rs:1071` clears clip selection when a
  project loads.
- `apps/composer/src-tauri/src/ui/mod.rs:1260` exposes a generic selected-clip
  automation node, but not the selected clip id.

## Test Review

- `apps/composer/src-tauri/tests/plan_ui_e2e.rs:262` clicks one clip and asserts
  `composer.timeline.clip.selected` is present.
- There is no test for selecting another clip, deleting the selected clip, or
  loading a project.
- There is no assertion that inspector fields update from the newly selected
  clip after selection changes.

## Required Test Shape

- Select clip A, then clip B, and assert only clip B exposes selected state.
- Delete the selected clip and assert `selected_clip_id` is `None` and no
  selected-clip automation node remains.
- Load a fixture project and assert selection is cleared and no stale highlight
  appears.
- Assert inspector content tracks `selected_clip()` after each selection change.

## Required Changes

- Extend selected-clip automation with selected clip id and selected state on
  each clip node.
- Add E2E coverage for selection transfer, delete cleanup, load cleanup, and
  inspector derivation.
- Keep selection highlighting based on stable clip ids rather than row indices.

## Verification

- `cargo test -p tench-composer automatic_clip_selection_highlight`
- `cargo test -p tench-composer composer_plan_inspector_render_queue_ai_and_context_menu_use_real_events_ui_e2e`
- `cargo test -p tench-ui-test ui_automation`
- `git diff --check`
