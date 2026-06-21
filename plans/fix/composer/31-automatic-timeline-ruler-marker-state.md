# Automatic Timeline Ruler Marker State Fix Plan

## Source Plan

- `plans/composer/automatic-timeline-ruler-marker-behavior-work-plan.md`

## Gap Analysis

Timeline ruler markers are drawn inline, but marker frames, labels, and grid
line positions are not exposed through automation or tested. Zoom state is also
not applied to ruler or clip geometry, so the plan's zoom-alignment scenario is
not implemented in a meaningful way.

## Plan Requirements Not Met

- There is no test that short projects still render valid ruler markers.
- There is no test that changing fps through a template recalculates marker
  labels in seconds.
- There is no test that adding clips expands marker range to cover the new
  timeline duration.
- There is no test that ruler marker lines remain aligned with clip geometry
  after zoom changes.
- Timeline zoom state is not used by ruler or clip geometry.
- Automation does not expose ruler marker frames, labels, x coordinates, grid
  line bounds, or timeline content bounds.
- The E2E selector smoke test does not assert `composer.automatic.ruler`.

## Code Review

- `apps/composer/src-tauri/src/ui/timeline_panel.rs:40` reads
  `total_frames()` and `fps()` during paint.
- `apps/composer/src-tauri/src/ui/timeline_panel.rs:44` uses an inline fixed
  `60` frame marker interval.
- `apps/composer/src-tauri/src/ui/timeline_panel.rs:47` computes marker x with
  `timeline::frame_to_x`.
- `apps/composer/src-tauri/src/ui/timeline_panel.rs:48` computes marker labels
  in seconds from fps.
- `apps/composer/src-tauri/src/ui/timeline.rs:99` defines zoom-aware frame
  mapping helpers, but the ruler path does not use them.
- `apps/composer/src-tauri/src/ui/mod.rs:1345` creates only a generic automatic
  ruler node.

## Test Review

- `apps/composer/src-tauri/tests/plan_ui_e2e.rs:110` through `:113` check a few
  automatic selector placeholders, but not `composer.automatic.ruler`.
- Existing tests do not inspect ruler labels, marker line positions, duration
  range, fps changes, zoom changes, or clip alignment.

## Required Test Shape

- Extract deterministic marker generation into a helper that can be unit tested
  for short and longer durations.
- Expose automation nodes for ruler markers with frame, label, x coordinate,
  and grid line bounds.
- Select a different-fps template and assert marker labels recalculate from the
  new fps.
- Add or extend clips, then assert marker frames cover the new duration.
- Change zoom and assert ruler marker x positions stay aligned with clip rects
  computed from the same timeline geometry model.

## Required Changes

- Use one shared timeline geometry path for ruler markers, clip rects, seek
  mapping, and zoomed content width.
- Apply `state.zoom` consistently where zoom is intended to change timeline
  content width.
- Keep ruler rendering independent from clip selection state and cover that
  with a regression test.

## Verification

- `cargo test -p tench-composer automatic_timeline_ruler_marker`
- `cargo test -p tench-composer composer_plan_left_panel_templates_effects_transitions_and_subtitles_use_real_events_ui_e2e`
- `cargo test -p tench-ui-test ui_automation`
- `git diff --check`
