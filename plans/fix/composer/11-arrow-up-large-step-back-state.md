# Arrow Up Large Step Back State Fix Plan

## Source Plan

- `plans/composer/arrow-up-large-step-back-shortcut-control-work-plan.md`

## Gap Analysis

Arrow Up moves the playhead backward by 24 frames, but the behavior is not
tested and the shortcut does not enforce modifier or text-focus precedence.
Frame-zero clamp and preview/timecode sync need explicit verification.

## Plan Requirements Not Met

- Arrow Up does not guard modifier keys before invoking the global shortcut.
- Arrow Up can move the playhead while the subtitle editor is focused.
- The shortcut does not set a notice for visible movement or no-op at frame
  zero.
- There is no E2E coverage for Arrow Up.
- Tests do not assert the 24-frame backward delta, frame-zero clamp, or
  preview/timecode sync.
- Rapid repeated Arrow Up behavior is not covered.

## Code Review

- `apps/composer/src-tauri/src/ui/mod.rs:971` routes Arrow Up directly to
  `self.state.step_frame(-24)`.
- `apps/composer/src-tauri/src/ui/state.rs:713` clamps negative movement with
  `saturating_sub`, but returns no moved/clamped status for notices.
- `apps/composer/src-tauri/src/ui/mod.rs:911` handles subtitle text input first,
  but does not consume Arrow Up as a text-editor navigation key.

## Test Review

- `apps/composer/src-tauri/tests/plan_ui_e2e.rs` has no Arrow Up or Arrow Down
  assertions.
- Existing playback shortcut coverage checks Arrow Right and Arrow Left by
  capture changes only, not state deltas.

## Required Test Shape

- Set `current_frame` above 24, press Arrow Up, and assert it decreased by
  exactly 24 frames.
- Set `current_frame` below 24, press Arrow Up, and assert it clamps to frame
  zero with deterministic no-op/clamp status.
- Assert preview timecode and playhead automation values update after movement.
- Focus `composer.subtitle.editor`, press Arrow Up, and assert subtitle
  editing/navigation precedence while `current_frame` remains unchanged.
- Press modified Arrow Up combinations and assert they do not trigger the plain
  shortcut unless assigned.
- Repeat Arrow Up quickly and assert deterministic clamped final state.

## Required Changes

- Route Arrow Up through a shared large-step helper that returns
  moved/clamped/no-op status.
- Gate the shortcut behind focused-input and modifier rules.
- Set a notice for visible movement or no-op according to the product notice
  policy.
- Expose current frame and timecode values through automation.

## Verification

- `cargo test -p tench-composer arrow_up_large_step_back`
- `cargo test -p tench-composer composer_plan_playback_keyboard_use_real_events_ui_e2e`
- `cargo test -p tench-ui-test ui_automation`
- `git diff --check`
