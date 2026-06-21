# Clip Context Menu Duplicate State Fix Plan

## Source Plan

- `plans/composer/clip-context-menu-duplicate-button-work-plan.md`

## Gap Analysis

Duplicate menu dispatch is wired, but the duplicate is created through the
generic add-clip path. That path resets clip properties instead of copying the
source clip's media range, speed, reversed state, effects, transitions, and
enabled state.

## Plan Requirements Not Met

- Duplicate does not create a full copy of the source clip's editable
  properties.
- Duplicate reuses the generic `Clip added` notice instead of a
  duplicate-specific result.
- There is no test that Duplicate inserts a clip immediately after the original.
- There is no test that the duplicate receives a unique id and copy name.
- There is no test that duplicate preserves source clip properties that should
  carry over.
- There is no test that Duplicate pushes undo and selects or exposes the new
  duplicate deterministically.
- There is no test that changing selection after menu open still duplicates the
  clip captured by the menu item.
- A stale Duplicate target can silently no-op without an actionable notice.

## Code Review

- `apps/composer/src-tauri/src/ui/mod.rs:762` stores
  `ClickAction::DuplicateClip(clip_id)` when the context menu opens.
- `apps/composer/src-tauri/src/ui/mod.rs:793` clears `context_menu` before
  dispatching enabled item actions.
- `apps/composer/src-tauri/src/ui/mod.rs:360` dispatches Duplicate to
  `state.duplicate_clip`.
- `apps/composer/src-tauri/src/ui/state.rs:1022` looks up the captured clip id.
- `apps/composer/src-tauri/src/ui/state.rs:1034` duplicates by calling
  `add_clip_to_track`.
- `apps/composer/src-tauri/src/ui/state.rs:845` creates a new clip with default
  media, speed, reversed, enabled, effects, and transition fields.

## Test Review

- `apps/composer/src-tauri/tests/plan_ui_e2e.rs:343` asserts the Duplicate menu
  item is present.
- Existing E2E clicks Copy only; it does not click
  `composer.clip.context.duplicate`.
- Existing tests do not assert duplicate id, name, position, copied properties,
  undo stack, notice, or menu-target stability.

## Required Test Shape

- Open the context menu on clip A, click Duplicate, and assert a new clip exists
  on the same track with a unique id and ` (copy)` name.
- Assert the duplicate starts at the original clip's `timeline_out`.
- Assert media range, duration, speed, reversed, enabled, effects, and
  transitions are preserved according to the intended copy rule.
- Assert undo stack grows and the menu is absent after dispatch.
- Open a menu on clip A, select clip B through a controlled state change, click
  Duplicate, and assert clip A is duplicated.
- Force a stale menu target and assert an actionable no-op notice.

## Required Changes

- Implement a duplicate-specific helper that clones the source clip, assigns a
  new id and copy name, moves it after the original, and preserves intended clip
  properties.
- Return a duplicate result so missing-target and insert-failed paths can show
  accurate notices.
- Expose menu target id, clip ids, clip names, and clip property summaries
  through automation or focused test helpers.

## Verification

- `cargo test -p tench-composer clip_context_menu_duplicate`
- `cargo test -p tench-composer composer_plan_inspector_render_queue_ai_and_context_menu_use_real_events_ui_e2e`
- `cargo test -p tench-ui-test ui_automation`
- `git diff --check`
