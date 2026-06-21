# Clip Context Menu Delete State Fix Plan

## Source Plan

- `plans/composer/clip-context-menu-delete-button-work-plan.md`

## Gap Analysis

Delete menu dispatch captures a clip id and routes deletion through the shared
selected-clip delete path, but Delete itself is not covered by E2E. Ripple
behavior, selection clearing, undo, target stability, and stale-target no-op
behavior are unverified.

## Plan Requirements Not Met

- There is no test that clicking Delete removes the clip associated with the
  opened context menu.
- There is no test that Delete clears selection after the clip is removed.
- There is no test that Delete pushes undo for the timeline mutation.
- There is no test that Delete shows the correct non-ripple or ripple delete
  notice.
- There is no test that changing selection after menu open still deletes the
  clip captured by the menu item.
- There is no test that ripple deletion follows the active ripple state.
- A stale Delete target can silently no-op without an actionable notice.

## Code Review

- `apps/composer/src-tauri/src/ui/mod.rs:767` stores
  `ClickAction::DeleteClip(clip_id)` when the context menu opens.
- `apps/composer/src-tauri/src/ui/mod.rs:793` clears `context_menu` before
  dispatching enabled item actions.
- `apps/composer/src-tauri/src/ui/mod.rs:211` sets `selected_clip_id` to the
  captured id before calling `delete_selected_clip`.
- `apps/composer/src-tauri/src/ui/state.rs:789` returns `false` without a notice
  when the selected target is missing.
- `apps/composer/src-tauri/src/ui/state.rs:802` chooses ripple or normal delete
  from current ripple state.
- `apps/composer/src-tauri/src/ui/state.rs:805` and `:819` clear selection on
  successful deletion.

## Test Review

- `apps/composer/src-tauri/tests/plan_ui_e2e.rs:344` asserts the Delete menu
  item is present.
- Existing E2E clicks Copy only; it does not click
  `composer.clip.context.delete`.
- Existing unit coverage deletes selected clips directly, but does not cover
  context-menu target capture, menu clearing, ripple state, or stale targets.

## Required Test Shape

- Open the context menu on clip A, click Delete, and assert clip A is removed,
  selection is cleared, undo stack grows, and the menu is absent.
- Repeat with `ripple = true` and assert the ripple delete notice and timeline
  shift behavior.
- Open a menu on clip A, select clip B through a controlled state change, click
  Delete, and assert clip A is removed while clip B remains.
- Force a stale menu target and assert an actionable no-op notice instead of a
  silent failure.

## Required Changes

- Make target deletion return a result that distinguishes deleted,
  missing-target, and failed outcomes.
- Surface stale target failures through notices.
- Expose menu item target id, selected clip id, undo count, and clip list state
  through automation or focused test helpers.

## Verification

- `cargo test -p tench-composer clip_context_menu_delete`
- `cargo test -p tench-composer composer_plan_inspector_render_queue_ai_and_context_menu_use_real_events_ui_e2e`
- `cargo test -p tench-ui-test ui_automation`
- `git diff --check`
