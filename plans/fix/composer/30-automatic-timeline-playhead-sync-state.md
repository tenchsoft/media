# Automatic Timeline Playhead Sync State Fix Plan

## Source Plan

- `plans/composer/automatic-timeline-playhead-sync-behavior-work-plan.md`

## Gap Analysis

Timeline playhead drawing is derived from `current_frame`, but the behavior is
not testable through current automation. Tests only assert a generic playhead
selector exists or that captures change after keyboard input; they do not prove
the red line moved to the frame produced by seek, playback, step, or project
load.

## Plan Requirements Not Met

- There is no test that timeline seek moves the timeline playhead line to the
  clicked frame.
- There is no test that playback animation frames advance the timeline playhead
  geometry with `current_frame`.
- There is no timeline playhead geometry test for stepping backward at frame
  zero.
- There is no test that loading a project with a different duration clamps the
  rendered playhead within the timeline content area.
- Automation exposes only a static `composer.automatic.playhead` node; it does
  not expose the playhead frame, x coordinate, line bounds, or timeline content
  bounds needed for assertions.

## Code Review

- `apps/composer/src-tauri/src/ui/timeline.rs:27` maps frames to x positions.
- `apps/composer/src-tauri/src/ui/timeline.rs:36` maps pointer x positions back
  to frames.
- `apps/composer/src-tauri/src/ui/timeline_panel.rs:357` computes the red
  timeline playhead x position from `state.current_frame` and `total_frames`.
- `apps/composer/src-tauri/src/ui/mod.rs:819` seeks from timeline pointer
  position and requests repaint.
- `apps/composer/src-tauri/src/ui/mod.rs:963` through `:977` step the playhead
  from keyboard input and request repaint.
- `apps/composer/src-tauri/src/ui/mod.rs:1340` exposes a generic playhead node
  without frame or geometry values.

## Test Review

- `apps/composer/src-tauri/tests/plan_ui_e2e.rs:104` only checks
  `composer.timeline.seek` selector presence.
- `apps/composer/src-tauri/tests/plan_ui_e2e.rs:112` only checks
  `composer.automatic.playhead` selector presence.
- `apps/composer/src-tauri/tests/plan_ui_e2e.rs:364` sends keyboard step events
  and asserts capture changes only.

## Required Test Shape

- Expose a timeline playhead automation node with current frame, x coordinate,
  line bounds, triangle bounds, and timeline content bounds.
- Click `composer.timeline.seek`, then assert the resulting `current_frame` and
  playhead x position match `timeline::frame_to_x`.
- Start playback, send an animation frame, and assert the playhead automation
  frame and x coordinate advance.
- Set `current_frame` to zero, step backward, and assert the playhead remains at
  the start of the timeline content area.
- Load or inject a project with a shorter duration and assert the playhead
  remains within timeline content bounds after the state change.

## Required Changes

- Add value-bearing automation for the timeline playhead and timeline content
  area.
- Keep expected geometry in tests derived from `timeline::frame_to_x` rather
  than hardcoded pixels.
- Add focused E2E coverage for seek, playback, zero-step clamp, and
  duration-change clamp paths.

## Verification

- `cargo test -p tench-composer automatic_timeline_playhead_sync`
- `cargo test -p tench-composer composer_plan_keyboard_shortcuts_and_automatic_playback_use_real_events_ui_e2e`
- `cargo test -p tench-ui-test ui_automation`
- `git diff --check`
