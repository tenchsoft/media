# Control M Export Shortcut State Fix Plan

## Source Plan

- `plans/composer/control-m-export-shortcut-control-work-plan.md`

## Gap Analysis

Ctrl+M calls the same render enqueue state method used by Export buttons, but
the shortcut behavior is only smoke-tested through selector presence. It does
not verify job count, captured export settings, notice, modifier policy, focus
policy, or repeated shortcut behavior.

## Plan Requirements Not Met

- Ctrl+M does not reject extra modifiers such as Ctrl+Alt+M.
- There is no test that Ctrl+M increments render queue length exactly once.
- There is no test that the enqueued job captures the current export settings.
- There is no test that Ctrl+M shows the `Render queued` notice and opens the
  render queue.
- There is no test for Ctrl+M behavior while subtitle/text input is focused.
- There is no test that repeated Ctrl+M presses are deterministic, including
  whether multiple jobs should queue or duplicates should be blocked.
- There is no missing-target/precondition test for exporting an empty timeline
  or invalid export configuration.

## Code Review

- `apps/composer/src-tauri/src/ui/mod.rs:258` routes visible Export actions to
  `state.enqueue_render`.
- `apps/composer/src-tauri/src/ui/mod.rs:1041` handles Ctrl+M.
- `apps/composer/src-tauri/src/ui/mod.rs:1042` calls `state.enqueue_render`
  directly.
- `apps/composer/src-tauri/src/ui/state.rs:833` creates the render job name.
- `apps/composer/src-tauri/src/ui/state.rs:839` enqueues a render job with the
  current export settings.
- `apps/composer/src-tauri/src/ui/state.rs:841` shows the render queue.

## Test Review

- `apps/composer/src-tauri/tests/plan_ui_e2e.rs:397` presses Ctrl+M.
- `apps/composer/src-tauri/tests/plan_ui_e2e.rs:405` only asserts
  `composer.render_job.pause` is present.
- Existing Export button coverage checks a render job exists, but does not
  cover Ctrl+M-specific routing, modifiers, focus, or repeat behavior.

## Required Test Shape

- Snapshot render queue length, press Ctrl+M, and assert exactly one new job was
  enqueued.
- Change export settings, press Ctrl+M, and assert the new job stores those
  settings.
- Assert render queue visibility and `Render queued` notice after Ctrl+M.
- Press Ctrl+Alt+M and assert the shortcut policy is enforced.
- Focus subtitle/text input, press Ctrl+M, and assert the documented focus
  precedence.
- Press Ctrl+M repeatedly and assert the chosen queueing/deduping policy.
- Try Ctrl+M with an empty or invalid project/export state and assert the
  expected no-op or actionable message.

## Required Changes

- Add a shared export command result that both visible Export buttons and Ctrl+M
  can use for success, invalid target, and invalid settings.
- Add modifier and focus guards consistent with the shortcut policy.
- Expose render queue length, latest job settings, render queue visibility, and
  notice text through automation or focused test helpers.

## Verification

- `cargo test -p tench-composer control_m_export_shortcut`
- `cargo test -p tench-composer composer_plan_keyboard_shortcuts_and_automatic_playback_use_real_events_ui_e2e`
- `git diff --check`
