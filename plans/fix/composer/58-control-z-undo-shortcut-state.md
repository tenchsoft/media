# Control Z Undo Shortcut State Fix Plan

## Source Plan

- `plans/composer/control-z-undo-shortcut-control-work-plan.md`

## Gap Analysis

Undo state exists and has unit coverage, but the Ctrl+Z shortcut is not covered
by E2E. Modifier policy, focus policy, no-undo feedback, and repeated undo
behavior are unverified.

## Plan Requirements Not Met

- Ctrl+Z does not reject extra modifiers such as Ctrl+Alt+Z.
- There is no E2E test that Ctrl+Z restores an undo snapshot.
- There is no E2E test that Ctrl+Z shows `Undo`.
- There is no E2E test that Ctrl+Z with an empty undo stack shows
  `Nothing to undo`.
- There is no test for Ctrl+Z behavior while subtitle/text input is focused.
- There is no test that repeated Ctrl+Z presses remain deterministic at undo
  stack boundaries.
- Automation does not expose undo/redo stack lengths or notice text for
  shortcut assertions.

## Code Review

- `apps/composer/src-tauri/src/ui/mod.rs:1025` handles Ctrl+Z when Shift is not
  active.
- `apps/composer/src-tauri/src/ui/mod.rs:1028` calls `state.undo`.
- `apps/composer/src-tauri/src/ui/state.rs:509` restores from `undo_stack`.
- `apps/composer/src-tauri/src/ui/state.rs:516` sets `Undo` on success.
- `apps/composer/src-tauri/src/ui/state.rs:520` sets `Nothing to undo` when the
  undo stack is empty.

## Test Review

- `apps/composer/src-tauri/src/ui/state.rs:1183` unit-tests direct
  `undo()`/`redo()` state calls.
- There is no E2E test pressing Ctrl+Z.

## Required Test Shape

- Make an undoable project change, press Ctrl+Z, and assert the previous project
  state is restored with `Undo`.
- Press Ctrl+Z when `undo_stack` is empty and assert `Nothing to undo` with no
  project mutation.
- Press Ctrl+Alt+Z and assert the shortcut policy is enforced.
- Focus subtitle/text input, press Ctrl+Z, and assert documented focus
  precedence.
- Press Ctrl+Z repeatedly and assert undo stack boundaries are stable.

## Required Changes

- Add modifier and focus guards consistent with the shortcut policy.
- Expose undo/redo stack lengths, latest notice, and a project snapshot summary
  through automation or focused test helpers.
- Keep Ctrl+Z routed through the same undo state method used by any future
  visible undo control.

## Verification

- `cargo test -p tench-composer control_z_undo_shortcut`
- `cargo test -p tench-composer composer_plan_keyboard_shortcuts_and_automatic_playback_use_real_events_ui_e2e`
- `git diff --check`
