# Arrow Down Large Step Forward State Fix Plan

## Source Plan

- `plans/composer/arrow-down-large-step-forward-shortcut-control-work-plan.md`

## Gap Analysis

Arrow Down advances the playhead by 24 frames through `step_frame(24)`, but the
shortcut contract is not complete. Modifier handling, subtitle-editor
precedence, notices, and focused verification are missing.

## Plan Requirements Not Met

- Arrow Down does not guard modifier keys, so modified Arrow Down combinations
  can trigger the same global large-step shortcut.
- Arrow Down still moves the playhead while the subtitle editor is focused.
- The shortcut does not set a notice when the visible playhead state changes.
- Missing-target or no-op behavior is not reported.
- Tests do not assert the 24-frame delta or clamp at project duration.
- Tests do not cover rapid repeated Arrow Down keypresses.

## Code Review

- `apps/composer/src-tauri/src/ui/mod.rs:975` routes Arrow Down directly to
  `self.state.step_frame(24)` without modifier or focused-input checks.
- `apps/composer/src-tauri/src/ui/state.rs:712` clamps positive movement through
  `seek_to_frame`, but returns no operation result for notice/no-op handling.
- `apps/composer/src-tauri/src/ui/mod.rs:911` handles subtitle text input before
  global shortcuts, but does not consume navigation keys such as Arrow Down.
- `apps/composer/src-tauri/src/ui/mod.rs:1346` exposes only a generic playhead
  automation status node, not the current frame value.

## Test Review

- `apps/composer/src-tauri/tests/plan_ui_e2e.rs:364` and `:370` cover Arrow
  Right and Arrow Left by capture change only.
- There is no Arrow Down E2E assertion for frame delta, duration clamp,
  modifier suppression, subtitle focus, or repeated keypresses.

## Required Test Shape

- Add an E2E test that records `current_frame`, presses Arrow Down once, and
  asserts it increased by exactly 24 or clamped to the final frame.
- Set `current_frame` near project duration, press Arrow Down, and assert it
  clamps at `total_frames() - 1`.
- Focus `composer.subtitle.editor`, press Arrow Down, and assert subtitle
  editing behavior takes precedence while `current_frame` remains unchanged.
- Press modified Arrow Down combinations and assert they do not trigger the
  plain shortcut unless explicitly assigned.
- Repeat Arrow Down quickly and assert deterministic clamped final state.

## Required Changes

- Add a shared large-step helper that returns moved/clamped/no-op status.
- Route Arrow Down through that helper and set a notice when the playhead moves
  or clamps.
- Gate the shortcut behind focused-input and modifier rules.
- Expose playhead frame value through automation so visual sync is testable.

## Verification

- `cargo test -p tench-composer arrow_down_large_step_forward`
- `cargo test -p tench-composer composer_plan_playback_keyboard_use_real_events_ui_e2e`
- `cargo test -p tench-ui-test ui_automation`
- `git diff --check`
