# Clip Context Menu Paste State Fix Plan

## Source Plan

- `plans/composer/clip-context-menu-paste-button-work-plan.md`

## Gap Analysis

Paste is wired to insert the clipboard clip at `current_frame` on the first
unlocked track, but the context-menu disabled path can prevent the documented
`Nothing to paste` message from appearing. Paste also uses the generic add-clip
path, which resets several copied clip properties.

## Plan Requirements Not Met

- Clicking Paste with an empty clipboard does not dispatch `paste_clip`, so the
  user may not see the documented `Nothing to paste` message.
- Pasted clips do not preserve all clipboard clip properties such as media
  range, speed, reversed state, effects, transitions, and enabled state.
- There is no test that Paste inserts at the playhead on the first unlocked
  track.
- There is no test that Paste pushes undo and selects or exposes the new clip
  deterministically.
- There is no test that the Paste item is disabled or enabled according to
  clipboard state.
- There is no test that all locked tracks produce the `No unlocked track`
  notice and no mutation.
- There is no test that the menu closes before Paste dispatch and cannot repeat
  from stale item regions.

## Code Review

- `apps/composer/src-tauri/src/ui/mod.rs:757` stores
  `ClickAction::PasteClip` in the context menu.
- `apps/composer/src-tauri/src/ui/mod.rs:758` disables Paste when
  `clipboard.clip` is `None`.
- `apps/composer/src-tauri/src/ui/mod.rs:793` clears `context_menu` before
  dispatching enabled item actions.
- `apps/composer/src-tauri/src/ui/mod.rs:357` dispatches Paste to
  `state.paste_clip`.
- `apps/composer/src-tauri/src/ui/state.rs:1003` reports `Nothing to paste`
  only if `paste_clip` is called.
- `apps/composer/src-tauri/src/ui/state.rs:1007` chooses the first unlocked
  track.
- `apps/composer/src-tauri/src/ui/state.rs:1012` pastes by calling the generic
  `add_clip_to_track` helper.
- `apps/composer/src-tauri/src/ui/state.rs:845` creates a new clip with default
  media, speed, reversed, enabled, effects, and transition fields.

## Test Review

- `apps/composer/src-tauri/tests/plan_ui_e2e.rs:342` asserts the Paste item is
  present.
- Existing E2E clicks Copy only; it does not click
  `composer.clip.context.paste`.
- Existing tests do not assert clipboard state, enabled state, inserted clip
  position, target track, undo stack, notices, or preserved clip properties.

## Required Test Shape

- Copy a clip, move the playhead, open the context menu, click Paste, and assert
  a new clip appears at `current_frame` on the first unlocked track.
- Assert undo stack grows and the menu is absent after dispatch.
- Assert pasted clip properties match the clipboard clip according to the
  intended paste rule.
- Open the menu with an empty clipboard and assert the disabled/no-op policy
  produces the documented user feedback.
- Lock all tracks, click Paste with a non-empty clipboard, and assert
  `No unlocked track` with no timeline mutation.

## Required Changes

- Define whether disabled Paste should remain inert or dispatch a no-op notice;
  implement that policy consistently with the plan.
- Implement a paste-specific helper that clones intended clipboard clip
  properties, assigns a new id, and inserts at the playhead.
- Expose clipboard state, Paste enabled state, inserted clip id, and track lock
  state through automation or focused test helpers.

## Verification

- `cargo test -p tench-composer clip_context_menu_paste`
- `cargo test -p tench-composer composer_plan_inspector_render_queue_ai_and_context_menu_use_real_events_ui_e2e`
- `cargo test -p tench-ui-test ui_automation`
- `git diff --check`
