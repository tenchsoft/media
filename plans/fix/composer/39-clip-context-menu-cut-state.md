# Clip Context Menu Cut State Fix Plan

## Source Plan

- `plans/composer/clip-context-menu-cut-button-work-plan.md`

## Gap Analysis

Cut menu dispatch is wired, but `cut_clip(clip_id)` copies the requested target
and then deletes `selected_clip_id`. If selection changes after the menu opens,
Cut can copy one clip and delete another. It also reports success even when the
delete step does not remove anything.

## Plan Requirements Not Met

- Cut does not reliably remove the clip associated with the opened context menu;
  deletion is based on current selection.
- Cut can show `Clip cut` even when the delete step fails or no selected clip is
  removed.
- There is no test that Cut stores the target clip in `clipboard.clip` before
  removing it from the timeline.
- There is no test that Cut removes exactly the menu target clip.
- There is no test that Cut pushes undo for the timeline mutation.
- There is no test that Cut closes the menu before dispatch and leaves no stale
  repeated activation path.
- There is no test that changing selection after menu open still cuts the clip
  associated with the menu item.

## Code Review

- `apps/composer/src-tauri/src/ui/mod.rs:747` stores
  `ClickAction::CutClip(clip_id)` when the context menu opens.
- `apps/composer/src-tauri/src/ui/mod.rs:793` clears `context_menu` before
  dispatching enabled item actions.
- `apps/composer/src-tauri/src/ui/mod.rs:351` dispatches Cut to
  `state.cut_clip`.
- `apps/composer/src-tauri/src/ui/state.rs:985` receives a `clip_id` argument.
- `apps/composer/src-tauri/src/ui/state.rs:986` copies that target clip into the
  clipboard.
- `apps/composer/src-tauri/src/ui/state.rs:988` calls `delete_selected_clip`
  instead of deleting the `clip_id` argument.
- `apps/composer/src-tauri/src/ui/state.rs:989` sets `Clip cut` without checking
  whether deletion succeeded.

## Test Review

- `apps/composer/src-tauri/tests/plan_ui_e2e.rs:340` asserts the Cut item is
  present.
- Existing E2E does not click `composer.clip.context.cut`.
- Existing tests do not assert clipboard contents, removed clip id, undo stack,
  notice text, or menu-target stability for Cut.

## Required Test Shape

- Open the context menu on clip A, click Cut, and assert clip A is in
  `clipboard.clip` and removed from the timeline.
- Assert undo stack length increases and notice text is `Clip cut`.
- Open a menu on clip A, select clip B before dispatch through a controlled
  state change, click Cut, and assert clip A is removed while clip B remains.
- Force a stale menu target and assert a clear no-op notice instead of a false
  success notice.
- Assert the menu is cleared before Cut dispatch and cannot be activated again
  from stale click regions.

## Required Changes

- Implement a target-based delete helper and have Cut remove the `clip_id`
  captured by the context menu item.
- Make `cut_clip` return a result that distinguishes copied-and-removed,
  missing-target, and delete-failed outcomes.
- Set the success notice only after the target clip has been removed.
- Expose clipboard clip id, menu item target id, and undo count through
  automation or focused test helpers.

## Verification

- `cargo test -p tench-composer clip_context_menu_cut`
- `cargo test -p tench-composer composer_plan_inspector_render_queue_ai_and_context_menu_use_real_events_ui_e2e`
- `cargo test -p tench-ui-test ui_automation`
- `git diff --check`
