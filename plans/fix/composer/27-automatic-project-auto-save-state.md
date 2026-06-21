# Automatic Project Auto Save State Fix Plan

## Source Plan

- `plans/composer/automatic-project-auto-save-behavior-work-plan.md`

## Gap Analysis

Auto-save currently runs from the paint path and performs synchronous project
serialization and filesystem writes. The behavior is also not covered by tests:
no test proves interval triggering, no-save-path no-op behavior, failure notice
behavior, or manual-save timer reset.

## Plan Requirements Not Met

- Auto-save can block rendering because `check_auto_save` is called during
  paint and invokes the synchronous save path directly.
- Save failures are surfaced as notices, but they are not logged with debugging
  context.
- `check_auto_save` returns `true` after attempting a save even when the save
  fails.
- There is no test that setting `save_path`, making a project change, and
  passing the interval writes the project and updates `last_auto_save`.
- There is no test that auto-save without a save path does not create a default
  file.
- There is no test that serialization or filesystem failures show a
  `Save failed` notice and leave auto-save timing in a predictable state.
- There is no test that a manual save before the interval resets subsequent
  auto-save timing.

## Code Review

- `apps/composer/src-tauri/src/ui/mod.rs:653` processes per-paint state work.
- `apps/composer/src-tauri/src/ui/mod.rs:655` calls `check_auto_save` from the
  paint path.
- `apps/composer/src-tauri/src/ui/state.rs:483` returns `false` without a save
  path.
- `apps/composer/src-tauri/src/ui/state.rs:489` attempts auto-save when the
  interval elapses.
- `apps/composer/src-tauri/src/ui/state.rs:490` calls the synchronous save
  routine and then returns `true` without knowing whether the write succeeded.
- `apps/composer/src-tauri/src/ui/state.rs:1048` performs the runtime save call
  synchronously and only updates `last_auto_save` on success.

## Test Review

- `apps/composer/src-tauri/tests/plan_ui_e2e.rs:395` only asserts that the
  static `composer.automatic.auto_save` selector is present after Ctrl+S.
- There is no state-level test that manipulates `last_auto_save` and
  `auto_save_interval`.
- There is no failure-path test for an unwritable save path.

## Required Test Shape

- Add state tests with a temporary save path that force an elapsed interval,
  call `check_auto_save`, and assert the file contents and `last_auto_save`.
- Add a no-save-path test that forces an elapsed interval and asserts no file is
  created and no default path is assigned.
- Add a failure-path test with an invalid or unwritable path that asserts a
  `Save failed` notice, no misleading success status, and no successful timer
  reset.
- Add a manual-save timing test that saves manually, advances the previous
  `last_auto_save` value in a controlled way, and asserts the next auto-save
  decision is predictable.

## Required Changes

- Move long-running save work out of the paint-critical path or add an
  asynchronous/background save dispatch that reports completion back to UI
  state.
- Return an explicit auto-save result such as skipped, queued, saved, or failed
  instead of a boolean that conflates attempted and successful saves.
- Log serialization and filesystem failures with path and project context while
  keeping the user-facing notice concise.

## Verification

- `cargo test -p tench-composer automatic_project_auto_save`
- `cargo test -p tench-composer composer_plan_keyboard_shortcuts_and_automatic_playback_use_real_events_ui_e2e`
- `git diff --check`
