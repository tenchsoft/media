# Delete Toolbar Button State Fix Plan

## Source Plan

- `plans/composer/delete-toolbar-button-work-plan.md`

## Gap Analysis

The Delete toolbar button is visually disabled when no clip is selected, but it
still registers a click action with dummy `ClipId(0)`. Dispatching that action
sets `selected_clip_id` to the dummy id before delete fails.

## Plan Requirements Not Met

- Disabled Delete still registers a click region.
- Disabled Delete dispatches a dummy `ClipId(0)`.
- Clicking disabled Delete can leave `selected_clip_id = Some(ClipId(0))`.
- There is no E2E test that Delete with ripple off removes only the selected
  clip and preserves surrounding clip positions.
- There is no E2E test that Delete with ripple on closes the gap.
- There is no test that Delete with no selection does not dispatch a dummy id or
  mutate selection state.
- There is no test that deleting a multi-selected clip removes all references
  from `selected_clip_ids`.

## Code Review

- `apps/composer/src-tauri/src/ui/toolbar.rs:84` computes disabled styling for
  Delete when `selected_clip_id` is `None`.
- `apps/composer/src-tauri/src/ui/toolbar.rs:113` builds Delete as
  `ClickAction::DeleteClip(state.selected_clip_id.unwrap_or(ClipId(0)))`.
- `apps/composer/src-tauri/src/ui/toolbar.rs:117` registers the click action
  even when the button is visually disabled.
- `apps/composer/src-tauri/src/ui/mod.rs:211` handles `DeleteClip`.
- `apps/composer/src-tauri/src/ui/mod.rs:212` sets `selected_clip_id` to the
  action id before deletion.
- `apps/composer/src-tauri/src/ui/state.rs:819` clears `selected_clip_id` after
  successful deletion.
- `apps/composer/src-tauri/src/ui/state.rs:820` removes the deleted id from
  `selected_clip_ids`.

## Test Review

- `apps/composer/src-tauri/tests/plan_ui_e2e.rs:82` only asserts
  `composer.toolbar.delete` is present.
- Existing tests do not click toolbar Delete or assert ripple, no-selection, or
  multi-selection behavior.

## Required Test Shape

- Select a clip, turn ripple off, click `composer.toolbar.delete`, and assert
  only that clip is removed while later clip positions remain unchanged.
- Select a clip, turn ripple on, click Delete, and assert later clips shift to
  close the gap.
- Clear selection, click Delete, and assert no dummy id is dispatched, no
  project data changes, and `selected_clip_id` remains `None`.
- Put the selected clip id into `selected_clip_ids`, delete it, and assert every
  reference is removed.
- Assert undo stack grows on successful delete and rolls back on core errors.

## Required Changes

- Do not register Delete click regions when disabled, or register a disabled
  action that cannot mutate selection.
- Remove dummy `ClipId(0)` dispatch from toolbar Delete.
- Add a shared delete command wrapper with explicit success and no-selection
  results.
- Expose selected clip id, multi-selection ids, ripple state, and clip positions
  through automation or focused test helpers.

## Verification

- `cargo test -p tench-composer delete_toolbar_button`
- `cargo test -p tench-composer composer_plan_project_timeline_controls_use_real_events_ui_e2e`
- `git diff --check`
