# Automatic Dialog Result Processing State Fix Plan

## Source Plan

- `plans/composer/automatic-dialog-result-processing-behavior-work-plan.md`

## Gap Analysis

Composer has a dialog-result channel and drains pending results during paint, but
the behavior is not fully exercised. Current E2E import coverage bypasses the
channel, and cancellation, save paths, multiple files, and sender lifecycle are
not tested.

## Plan Requirements Not Met

- E2E import coverage uses `test_next_media` instead of sending
  `DialogResult::FilesOpened` through the channel.
- Multiple pending dialog results are not tested.
- `Cancelled` is handled only by a catch-all no-op and is not tested as a
  distinct result.
- `SavePath` handling is not tested for setting `save_path` and persisting the
  project.
- `FileOpened` and `FilesOpened` are not both tested.
- The global `OnceLock` sender ignores replacement failures, so multiple
  ComposerApp instances may leave dialog results wired to an old receiver.

## Code Review

- `apps/composer/src-tauri/src/ui/mod.rs:39` defines `FileOpened`,
  `FilesOpened`, `SavePath`, and `Cancelled`.
- `apps/composer/src-tauri/src/ui/mod.rs:51` sends results through the global
  sender if one is registered.
- `apps/composer/src-tauri/src/ui/mod.rs:123` and `:136` call
  `DIALOG_SENDER.set(tx)` and ignore failure, which can keep stale sender state.
- `apps/composer/src-tauri/src/ui/mod.rs:178` drains results with `try_recv`.
- `apps/composer/src-tauri/src/ui/mod.rs:182` imports all files from
  `FilesOpened`; `:188` imports one `FileOpened`; `:192` handles `SavePath`;
  `_ => {}` collapses cancellation and other results.
- `apps/composer/src-tauri/tests/plan_ui_e2e.rs:120` uses `inject_test_media`
  and does not exercise the dialog channel.

## Test Review

- `apps/composer/src-tauri/tests/plan_ui_e2e.rs:125` asserts one injected media
  item was imported.
- There is no channel-based test for multiple files, cancellation, save path,
  or draining more than one result in a single UI processing pass.
- There is no test for creating multiple app instances and ensuring dialog
  results reach the active instance.

## Required Test Shape

- Add a test-only helper to enqueue `DialogResult` into the app's active
  receiver, then capture once and assert all pending results are drained.
- Send `FilesOpened` with multiple paths and assert every file imports.
- Send `FileOpened` and assert a single import.
- Send `Cancelled` and assert no media/save mutation and an explicit no-op
  status if desired.
- Send `SavePath` and assert `save_path` is stored and project persistence is
  attempted through a controlled fixture path.
- Construct multiple ComposerApp instances and assert dialog results are routed
  to the intended receiver.

## Required Changes

- Add channel-based test injection that does not bypass `process_dialog_results`.
- Handle `Cancelled` explicitly in the match, even if it remains a no-op.
- Replace or reset the global dialog sender safely for each app instance, or
  move dialog routing behind an instance-scoped testable abstraction.
- Add tests for multiple result draining and save-path processing.

## Verification

- `cargo test -p tench-composer automatic_dialog_result_processing`
- `cargo test -p tench-composer composer_plan_timeline_controls_use_real_events_ui_e2e`
- `cargo test -p tench-ui-test ui_automation`
- `git diff --check`
