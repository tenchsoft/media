# Arrow Right Step Forward State Fix Plan

## Source Plan

- `plans/composer/arrow-right-step-forward-shortcut-control-work-plan.md`

## Gap Analysis

Arrow Right advances the playhead by one frame, but the shortcut is not fully
verified or gated. The current tests rely on capture changes and do not prove
frame movement, clamp behavior, preview/timecode sync, or text-focus precedence.

## Plan Requirements Not Met

- Arrow Right does not guard modifier keys before invoking the global shortcut.
- Arrow Right can move the playhead while the subtitle editor is focused.
- The shortcut does not set a notice for the user-visible playhead change or
  no-op at project duration.
- Tests do not assert the one-frame delta or clamp at project duration.
- Tests do not verify preview/timecode values after movement.
- Rapid repeated Arrow Right behavior is not covered.

## Code Review

- `apps/composer/src-tauri/src/ui/mod.rs:967` routes Arrow Right directly to
  `self.state.step_frame(1)`.
- `apps/composer/src-tauri/src/ui/state.rs:716` clamps positive movement through
  `seek_to_frame`, but returns no moved/clamped status for notices.
- `apps/composer/src-tauri/src/ui/mod.rs:911` handles subtitle text input first,
  but does not consume Arrow Right as a text-editor navigation key.

## Test Review

- `apps/composer/src-tauri/tests/plan_ui_e2e.rs:364` presses Arrow Right and
  only asserts the capture changed.
- There is no test for exact frame delta, duration clamp, subtitle focus,
  modifier suppression, repeated keypresses, or timecode value updates.

## Required Test Shape

- Record `current_frame`, press Arrow Right once, and assert it increased by one
  frame unless already clamped at the end.
- Set `current_frame` to the final frame, press Arrow Right, and assert it
  remains clamped with deterministic no-op status.
- Assert preview timecode and playhead automation values update after movement.
- Focus `composer.subtitle.editor`, press Arrow Right, and assert subtitle
  editing/navigation precedence while `current_frame` remains unchanged.
- Press modified Arrow Right combinations and assert they do not trigger the
  plain shortcut unless assigned.
- Repeat Arrow Right quickly and assert deterministic clamped final state.

## Required Changes

- Route Arrow Right through a shared playhead step helper that returns
  moved/clamped/no-op status.
- Gate the shortcut behind focused-input and modifier rules.
- Set a notice for visible movement or no-op according to the product notice
  policy.
- Expose current frame and timecode values through automation.

## Verification

- `cargo test -p tench-composer arrow_right_step_forward`
- `cargo test -p tench-composer composer_plan_playback_keyboard_use_real_events_ui_e2e`
- `cargo test -p tench-ui-test ui_automation`
- `git diff --check`
