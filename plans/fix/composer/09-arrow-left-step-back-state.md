# Arrow Left Step Back State Fix Plan

## Source Plan

- `plans/composer/arrow-left-step-back-shortcut-control-work-plan.md`

## Gap Analysis

Arrow Left moves the playhead back one frame, but the shortcut is not fully
verified or gated. Timecode sync, subtitle-editor precedence, modifier handling,
notices, and clamp behavior need explicit coverage.

## Plan Requirements Not Met

- Arrow Left does not guard modifier keys before invoking the global shortcut.
- Arrow Left can move the playhead while the subtitle editor is focused.
- The shortcut does not set a notice for the user-visible playhead change or
  no-op at frame zero.
- Tests do not assert the one-frame delta, frame-zero clamp, or preview/timecode
  sync.
- Rapid repeated Arrow Left behavior is not tested.
- Automation does not expose current frame/timecode values for direct
  verification.

## Code Review

- `apps/composer/src-tauri/src/ui/mod.rs:963` routes Arrow Left directly to
  `self.state.step_frame(-1)`.
- `apps/composer/src-tauri/src/ui/state.rs:713` clamps negative movement with
  `saturating_sub`, but returns no moved/clamped status for notice handling.
- `apps/composer/src-tauri/src/ui/mod.rs:911` gives subtitle text input some
  precedence, but does not consume Arrow Left as a text-editor navigation key.
- `apps/composer/src-tauri/src/ui/mod.rs:1344` and `:1346` expose generic
  automatic timecode/playhead nodes without frame values.

## Test Review

- `apps/composer/src-tauri/tests/plan_ui_e2e.rs:370` presses Arrow Left and only
  asserts capture changed.
- There is no test for frame delta, clamp at zero, subtitle focus, modifier
  suppression, repeated keypresses, or timecode value updates.

## Required Test Shape

- Set `current_frame` above zero, press Arrow Left, and assert it decreased by
  exactly one frame.
- Set `current_frame` to zero, press Arrow Left, and assert it remains zero with
  deterministic no-op status.
- Assert preview timecode and playhead automation values update after movement.
- Focus `composer.subtitle.editor`, press Arrow Left, and assert text-editor
  precedence while `current_frame` remains unchanged.
- Press modified Arrow Left combinations and assert they do not trigger the
  plain shortcut unless assigned.
- Repeat Arrow Left quickly and assert deterministic clamped final state.

## Required Changes

- Route Arrow Left through a shared playhead step helper that returns
  moved/clamped/no-op status.
- Gate the shortcut behind focused-input and modifier rules.
- Set a notice for visible movement or no-op according to the product notice
  policy.
- Expose current frame and timecode values through automation.

## Verification

- `cargo test -p tench-composer arrow_left_step_back`
- `cargo test -p tench-composer composer_plan_playback_keyboard_use_real_events_ui_e2e`
- `cargo test -p tench-ui-test ui_automation`
- `git diff --check`
