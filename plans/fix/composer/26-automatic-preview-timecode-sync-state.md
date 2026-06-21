# Automatic Preview Timecode Sync State Fix Plan

## Source Plan

- `plans/composer/automatic-preview-timecode-sync-behavior-work-plan.md`

## Gap Analysis

Preview timecode sync is not behaviorally verified. Existing tests only assert
generic selector presence or capture changes after unrelated controls; they do
not prove that the preview label, monitor playhead, total duration, or in/out
overlays are derived from the latest timeline state.

## Plan Requirements Not Met

- There is no test that timeline seek updates the preview timecode and monitor
  playhead to the same frame.
- There is no test that keyboard stepping updates the preview frame counter by
  the exact step amount.
- There is no test that IN and OUT preview overlays appear at positions derived
  from their marker frames.
- There is no test that changing timeline duration recalculates the total
  preview timecode.
- Automation does not expose preview timecode text, monitor playhead bounds, or
  IN/OUT overlay marker positions.

## Code Review

- `apps/composer/src-tauri/src/ui/preview.rs:21` formats timecode.
- `apps/composer/src-tauri/src/ui/preview.rs:30` maps frames to monitor x
  positions.
- `apps/composer/src-tauri/src/ui/preview_panel.rs:55` draws the current and
  total timecode label.
- `apps/composer/src-tauri/src/ui/preview_panel.rs:74` draws the monitor
  playhead from `current_frame`.
- `apps/composer/src-tauri/src/ui/preview_panel.rs:83` and `:103` draw IN and
  OUT overlays, but no automation node exposes their computed positions.
- `apps/composer/src-tauri/src/ui/mod.rs:1340` exposes static automatic nodes
  without the rendered preview values needed for assertions.

## Test Review

- `apps/composer/src-tauri/tests/plan_ui_e2e.rs:111` checks
  `composer.automatic.playback` and `composer.automatic.playhead` selector
  presence, but not timecode text or playhead position.
- `apps/composer/src-tauri/tests/plan_ui_e2e.rs:364` sends keyboard step events
  and checks capture changes only.
- There is no E2E path that seeks the timeline and asserts preview timecode,
  monitor playhead geometry, and `current_frame` agree.

## Required Test Shape

- Add automation values for preview timecode text, preview current frame, total
  frame count, monitor playhead bounds, and IN/OUT overlay bounds.
- Seek the timeline through `composer.timeline.seek`, then assert
  `current_frame`, preview timecode text, and monitor playhead x position all
  match the same frame.
- Step with ArrowLeft/ArrowRight and assert the preview frame counter changes by
  the exact documented step amount.
- Set IN/OUT markers, assert overlay nodes appear, and compare their x
  positions against `preview::monitor_playhead_x`.
- Add or extend a clip to change timeline duration and assert the total
  timecode text recalculates from `total_frames()` and `fps()`.

## Required Changes

- Expose dedicated preview automation nodes instead of relying on generic
  `composer.automatic.*` placeholders for this behavior.
- Keep preview value calculation derived during paint or automation inventory
  construction; do not store duplicate preview positions in mutable state.
- Reuse `preview::format_timecode` and `preview::monitor_playhead_x` in tests
  for expected values.

## Verification

- `cargo test -p tench-composer automatic_preview_timecode_sync`
- `cargo test -p tench-composer composer_plan_keyboard_shortcuts_and_automatic_playback_use_real_events_ui_e2e`
- `cargo test -p tench-ui-test ui_automation`
- `git diff --check`
