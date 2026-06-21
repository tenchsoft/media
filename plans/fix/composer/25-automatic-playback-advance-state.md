# Automatic Playback Advance State Fix Plan

## Source Plan

- `plans/composer/automatic-playback-advance-behavior-work-plan.md`

## Gap Analysis

Playback advancement is wired through `WindowEvent::AnimFrame` and
`ComposerState::advance_playback`, but the plan's state transitions are not
verified. The current E2E only checks that a static playback automation node is
present after one animation frame.

Loop in/out boundary gaps are already tracked by
`plans/fix/composer/21-automatic-loop-playback-boundary-state.md`; this plan
should cover the non-duplicated playback tick, clamp, pause, and render-update
coverage.

## Plan Requirements Not Met

- There is no test asserting that animation frames advance `current_frame`
  according to shuttle direction and speed.
- There is no test asserting the non-loop project-end clamp rule, including the
  resulting `current_frame` and `is_playing` state.
- There is no test asserting that paused playback ignores animation frames and
  leaves `current_frame` unchanged.
- There is no test proving that a playback advancement repaints the preview and
  timeline output.
- Playback, timecode, and playhead automation nodes are static selectors only;
  they do not expose current frame, rendered timecode, or playhead geometry for
  behavioral assertions.

## Code Review

- `apps/composer/src-tauri/src/ui/state.rs:721` returns early when playback is
  paused and otherwise advances by rounded shuttle speed and direction.
- `apps/composer/src-tauri/src/ui/state.rs:737` documents end-of-project clamp
  behavior, but there is no focused coverage for it.
- `apps/composer/src-tauri/src/ui/mod.rs:1093` handles animation-frame events
  and requests paint only while `is_playing`.
- `apps/composer/src-tauri/src/ui/mod.rs:1340` exposes generic automatic nodes,
  but no state value is attached to playback, timecode, or playhead nodes.

## Test Review

- `apps/composer/src-tauri/tests/plan_ui_e2e.rs:131` clicks play and sends one
  animation frame.
- `apps/composer/src-tauri/tests/plan_ui_e2e.rs:137` only asserts
  `composer.automatic.playback` is present; it does not assert frame movement,
  capture change after advancement, pause behavior, speed/direction behavior, or
  clamp behavior.

## Required Test Shape

- Add state-level tests that set shuttle direction/speed and assert exact
  `current_frame` movement for forward, reverse, and multi-speed playback.
- Add a state-level test that starts near the project end with loop disabled and
  asserts the documented clamp behavior.
- Add an E2E test that sends `AnimFrame` while paused and asserts
  `current_frame` and capture output do not advance.
- Add an E2E test that sends `AnimFrame` while playing and asserts
  `current_frame` changes, the capture changes, and playback/timecode/playhead
  automation values reflect the new frame.

## Required Changes

- Attach stable automation values for playback state, preview timecode, and
  timeline playhead frame/bounds.
- Keep E2E assertions selector-based and avoid hardcoded click coordinates.
- Keep loop boundary-specific assertions in the existing loop boundary fix
  scope instead of duplicating them here.

## Verification

- `cargo test -p tench-composer automatic_playback_advance`
- `cargo test -p tench-composer composer_plan_keyboard_shortcuts_and_automatic_playback_use_real_events_ui_e2e`
- `cargo test -p tench-ui-test ui_automation`
- `git diff --check`
