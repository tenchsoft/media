# Control Shift Z Redo Shortcut State Fix Plan

## Source Plan

- `plans/composer/control-shift-z-redo-shortcut-control-work-plan.md`

## Gap Analysis

Redo state exists and has unit coverage, but the Ctrl+Shift+Z shortcut is not
covered by E2E. Modifier policy, focus policy, no-redo feedback, and repeated
redo behavior are unverified.

## Plan Requirements Not Met

- Ctrl+Shift+Z does not reject extra modifiers such as Ctrl+Shift+Alt+Z.
- There is no E2E test that Ctrl+Shift+Z restores a redo snapshot.
- There is no E2E test that Ctrl+Shift+Z shows `Redo`.
- There is no E2E test that Ctrl+Shift+Z with an empty redo stack shows
  `Nothing to redo`.
- There is no test for Ctrl+Shift+Z behavior while subtitle/text input is
  focused.
- There is no test that repeated Ctrl+Shift+Z presses remain deterministic at
  redo stack boundaries.
- Automation does not expose undo/redo stack lengths or notice text for
  shortcut assertions.

## Code Review

- `apps/composer/src-tauri/src/ui/mod.rs:1031` handles Ctrl+Shift+Z.
- `apps/composer/src-tauri/src/ui/mod.rs:1034` calls `state.redo`.
- `apps/composer/src-tauri/src/ui/state.rs:524` restores from `redo_stack`.
- `apps/composer/src-tauri/src/ui/state.rs:531` sets `Redo` on success.
- `apps/composer/src-tauri/src/ui/state.rs:535` sets `Nothing to redo` when the
  redo stack is empty.

## Test Review

- `apps/composer/src-tauri/src/ui/state.rs:1183` unit-tests direct
  `undo()`/`redo()` state calls.
- There is no E2E test pressing Ctrl+Shift+Z.

## Required Test Shape

- Make an undoable project change, press Ctrl+Z, then press Ctrl+Shift+Z and
  assert the project state is restored with `Redo`.
- Press Ctrl+Shift+Z when `redo_stack` is empty and assert `Nothing to redo`
  with no project mutation.
- Press Ctrl+Shift+Alt+Z and assert the shortcut policy is enforced.
- Focus subtitle/text input, press Ctrl+Shift+Z, and assert documented focus
  precedence.
- Press Ctrl+Shift+Z repeatedly and assert redo stack boundaries are stable.

## Required Changes

- Add modifier and focus guards consistent with the shortcut policy.
- Expose undo/redo stack lengths, latest notice, and a project snapshot summary
  through automation or focused test helpers.
- Keep Ctrl+Shift+Z routed through the same redo state method used by any future
  visible redo control.

## Verification

- `cargo test -p tench-composer control_shift_z_redo_shortcut`
- `cargo test -p tench-composer composer_plan_keyboard_shortcuts_and_automatic_playback_use_real_events_ui_e2e`
- `git diff --check`
