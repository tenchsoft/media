# Automatic Loop Playback Boundary State Fix Plan

## Source Plan

- `plans/composer/automatic-loop-playback-boundary-behavior-work-plan.md`

## Gap Analysis

Loop playback checks in/out boundaries during playback advancement, but the
behavior is not verified and boundary validity is not modeled. Tests do not
prove jumps to the in point, no-point behavior, disabled-loop behavior, or marker
sync.

## Plan Requirements Not Met

- There is no test that playback jumps back to `in_point` after reaching
  `out_point`.
- Loop-enabled behavior without both points is not tested.
- Current-frame behavior when already beyond `out_point` is not tested.
- Disabling loop at the out point is not tested.
- Invalid boundary ordering, such as `in_point >= out_point`, is not handled or
  reported.
- Automation does not expose loop, in-point, out-point, or current-frame values.
- Marker and notice rendering from loop state is not verified.

## Code Review

- `apps/composer/src-tauri/src/ui/state.rs:721` advances playback only while
  `is_playing`.
- `apps/composer/src-tauri/src/ui/state.rs:729` checks loop playback and seeks
  to `in_point` when both points exist and `current_frame >= out_point`.
- `apps/composer/src-tauri/src/ui/mod.rs:1058` sets in/out points from keyboard
  shortcuts and uses notices.
- `apps/composer/src-tauri/src/ui/mod.rs:1075` toggles `loop_playback` and sets
  a notice.
- `apps/composer/src-tauri/src/ui/timeline_panel.rs:70` and `:79` render in/out
  markers, but automation does not expose marker frame values.

## Test Review

- `apps/composer/src-tauri/tests/plan_ui_e2e.rs:134` sends one animation frame
  and checks only `composer.automatic.playback` is present.
- `apps/composer/src-tauri/tests/plan_ui_e2e.rs:377` smoke-tests `i`, `o`, and
  `b` by capture change only.
- There is no state-level or E2E test for loop boundary behavior.

## Required Test Shape

- Set `in_point`, `out_point`, `loop_playback`, and `is_playing`, advance frames,
  and assert `current_frame` returns to `in_point` when it reaches `out_point`.
- Enable loop with only one or zero boundary points and assert playback does not
  jump.
- Start beyond `out_point` while playing and assert the next advancement follows
  the documented clamp or jump rule.
- Disable loop and assert playback no longer jumps at `out_point`.
- Test invalid `in_point >= out_point` behavior and assert an actionable notice
  or deterministic no-op.
- Assert in/out marker automation values and loop notice state.

## Required Changes

- Add a loop-boundary helper that validates boundary ordering and returns
  advanced/jumped/no-op status.
- Expose current frame, loop state, and in/out marker values through automation.
- Add focused unit and E2E coverage for loop boundary scenarios.
- Keep marker rendering and notices derived from the same state used by playback.

## Verification

- `cargo test -p tench-composer automatic_loop_playback_boundary`
- `cargo test -p tench-composer composer_plan_playback_keyboard_use_real_events_ui_e2e`
- `cargo test -p tench-ui-test ui_automation`
- `git diff --check`
