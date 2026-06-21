# Clip Context Menu Copy State Fix Plan

## Source Plan

- `plans/composer/clip-context-menu-copy-button-work-plan.md`

## Gap Analysis

Copy menu dispatch is wired and the menu closes after the enabled action, but
the behavior is not verified beyond selector disappearance. Tests do not prove
that the clipboard contains the menu clip, the timeline is unchanged, undo is
not pushed, or the notice is shown.

## Plan Requirements Not Met

- There is no test that Copy stores the clip associated with the opened context
  menu in `clipboard.clip`.
- There is no test that Copy leaves timeline tracks, clip ranges, effects, and
  selection unchanged except for intentional selection from the right-click.
- There is no test that Copy shows the `Clip copied` notice.
- There is no test that Copy does not push undo because it is non-mutating.
- There is no test that opening a menu on one clip, then changing selection
  before dispatch, still copies the clip id captured by the menu item.
- Copy silently no-ops if the stored menu target clip no longer exists.

## Code Review

- `apps/composer/src-tauri/src/ui/mod.rs:741` builds context menu state when a
  clip is right-clicked.
- `apps/composer/src-tauri/src/ui/mod.rs:750` stores
  `ClickAction::CopyClip(clip_id)` in the Copy menu item.
- `apps/composer/src-tauri/src/ui/mod.rs:793` clears `context_menu` before
  dispatching enabled item actions.
- `apps/composer/src-tauri/src/ui/mod.rs:354` dispatches Copy to
  `state.copy_clip`.
- `apps/composer/src-tauri/src/ui/state.rs:994` copies the target clip into the
  clipboard and shows `Clip copied` only when the clip is found.
- `apps/composer/src-tauri/src/ui/mod.rs:1318` exposes context menu items by
  label, but not the target clip id captured by each item.

## Test Review

- `apps/composer/src-tauri/tests/plan_ui_e2e.rs:338` opens the clip context
  menu.
- `apps/composer/src-tauri/tests/plan_ui_e2e.rs:348` clicks Copy.
- `apps/composer/src-tauri/tests/plan_ui_e2e.rs:349` only asserts the Copy menu
  item is absent after dispatch.

## Required Test Shape

- Open the context menu on a known clip, click Copy, and assert
  `clipboard.clip` matches that clip id and content.
- Snapshot timeline tracks before Copy and assert they are unchanged after Copy.
- Assert undo stack length is unchanged and notice text is `Clip copied`.
- Open a menu on clip A, change selection to clip B through state injection or a
  controlled event, click Copy, and assert clip A is copied.
- Remove or invalidate the menu target before dispatch and assert an actionable
  no-op notice instead of a silent failure.

## Required Changes

- Expose context menu item target clip ids and clipboard state through
  automation or targeted test helpers.
- Make `copy_clip` return a result so the UI can distinguish success from a
  stale missing-target no-op.
- Keep Copy non-mutating: do not push undo or alter timeline data.

## Verification

- `cargo test -p tench-composer clip_context_menu_copy`
- `cargo test -p tench-composer composer_plan_inspector_render_queue_ai_and_context_menu_use_real_events_ui_e2e`
- `cargo test -p tench-ui-test ui_automation`
- `git diff --check`
