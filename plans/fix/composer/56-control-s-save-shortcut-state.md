# Control S Save Shortcut State Fix Plan

## Source Plan

- `plans/composer/control-s-save-shortcut-control-work-plan.md`

## Gap Analysis

Ctrl+S performs a save attempt, but the shortcut is not verified beyond a static
automatic selector. Tests do not assert save path creation, file contents,
notice text, error behavior, modifier policy, focus policy, or repeated-save
determinism.

## Plan Requirements Not Met

- Ctrl+S does not reject extra modifiers such as Ctrl+Shift+S or Ctrl+Alt+S.
- There is no test that Ctrl+S saves to an existing `save_path`.
- There is no test that Ctrl+S creates and stores the default save path when no
  path exists.
- There is no test that the saved file contains serialized project data.
- There is no test that Ctrl+S shows `Project saved` or `Save failed` notices.
- There is no test for Ctrl+S behavior while subtitle/text input is focused.
- There is no test that repeated Ctrl+S saves are deterministic and update
  save timing predictably.
- Failure logging and background-save behavior are tracked by
  `plans/fix/composer/27-automatic-project-auto-save-state.md`; this shortcut
  still needs its own routing and user-flow coverage.

## Code Review

- `apps/composer/src-tauri/src/ui/mod.rs:1012` handles Ctrl+S.
- `apps/composer/src-tauri/src/ui/mod.rs:1013` saves to an existing path when
  present.
- `apps/composer/src-tauri/src/ui/mod.rs:1019` builds a default relative
  `{}.composer` path when no save path exists.
- `apps/composer/src-tauri/src/ui/state.rs:1044` serializes and writes the
  project when `save_path` is present.
- `apps/composer/src-tauri/src/ui/state.rs:1051` sets `Project saved` and
  updates `last_auto_save` on success.
- `apps/composer/src-tauri/src/ui/state.rs:1055` reports serialization failure
  as `Save failed`.

## Test Review

- `apps/composer/src-tauri/tests/plan_ui_e2e.rs:387` presses Ctrl+S.
- `apps/composer/src-tauri/tests/plan_ui_e2e.rs:395` only asserts
  `composer.automatic.auto_save` is present.
- Existing tests do not assert `save_path`, file output, notice text,
  `last_auto_save`, modifiers, focus, or repeated shortcut behavior.

## Required Test Shape

- Set a temporary `save_path`, press Ctrl+S, and assert file contents,
  `Project saved`, and `last_auto_save`.
- Clear `save_path`, press Ctrl+S, and assert the default path is assigned and
  persisted.
- Use an invalid path, press Ctrl+S, and assert `Save failed` with no misleading
  success state.
- Press Ctrl+Shift+S and Ctrl+Alt+S and assert the shortcut policy is enforced.
- Focus subtitle/text input, press Ctrl+S, and assert documented focus
  precedence.
- Press Ctrl+S repeatedly and assert save timing and file contents remain
  deterministic.

## Required Changes

- Add a shared save command result for existing-path save, default-path save,
  and failure states.
- Add modifier and focus guards consistent with the shortcut policy.
- Expose save path, save notice, last auto-save timestamp state, and save output
  status through automation or focused test helpers.

## Verification

- `cargo test -p tench-composer control_s_save_shortcut`
- `cargo test -p tench-composer automatic_project_auto_save`
- `cargo test -p tench-composer composer_plan_keyboard_shortcuts_and_automatic_playback_use_real_events_ui_e2e`
- `git diff --check`
