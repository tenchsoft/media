# Clip Trim In Handle State Fix Plan

## Source Plan

- `plans/composer/clip-trim-in-handle-control-work-plan.md`

## Gap Analysis

Trim-in drag is wired, but dragging the in handle left is not implemented
correctly in core timeline logic. The current delta calculation only handles
moving the in point later; moving it earlier changes `timeline_in` without
lengthening duration or adjusting media range correctly.

## Plan Requirements Not Met

- Dragging the in handle left within available media range does not lengthen the
  clip correctly.
- Media-range validation for leftward trim expansion is missing.
- There is no E2E test that dragging the in handle right moves clip start later
  and shortens duration.
- There is no E2E test that dragging the in handle left lengthens the clip when
  allowed.
- There is no E2E test that dragging past the out point rejects the operation
  and shows the core error notice.
- There is no E2E test that Snap moves trim-in to a nearby boundary.
- Automation does not expose trim drag state, resulting media range, duration,
  or clip rect geometry after trim.

## Code Review

- `apps/composer/src-tauri/src/ui/mod.rs:500` hit-tests trim handles before clip
  body drag.
- `apps/composer/src-tauri/src/ui/mod.rs:502` starts `DragKind::TrimIn`.
- `apps/composer/src-tauri/src/ui/mod.rs:604` maps release x to a frame.
- `apps/composer/src-tauri/src/ui/mod.rs:605` applies `snap_position`.
- `apps/composer/src-tauri/src/ui/state.rs:938` pushes undo and calls the core
  trim-in method.
- `crates/composer-core/src/timeline.rs:463` rejects in points at or beyond the
  out point.
- `crates/composer-core/src/timeline.rs:467` computes delta with
  `new_timeline_in.saturating_sub(clip.timeline_in)`, so leftward trim expansion
  has zero delta.

## Test Review

- `apps/composer/src-tauri/tests/plan_ui_e2e.rs:101` only asserts
  `composer.timeline.clip.0.trim_in` is present.
- Existing drag E2E coverage is for the left splitter, not clip trim handles.
- Existing `snap_position` unit coverage does not verify trim integration.

## Required Test Shape

- Drag the trim-in handle right and assert `timeline_in` increases, duration
  decreases, and undo stack grows.
- Drag the trim-in handle left within media range and assert `timeline_in`
  decreases while duration lengthens according to the media range rule.
- Drag trim-in past the clip out point and assert no mutation plus
  `New in-point must be before out-point`.
- Enable Snap, drag near a clip boundary, and assert trim-in lands on the
  snapped frame.
- Assert preview/timeline capture changes and clip rect bounds match the new
  frame range after a valid trim.

## Required Changes

- Update core trim-in logic to handle both rightward shortening and leftward
  lengthening with media range validation.
- Return explicit trim results so UI can distinguish valid trim, invalid
  boundary, and media-range rejection.
- Expose clip frame range, media range, duration, and trim handle bounds through
  automation.

## Verification

- `cargo test -p tench-composer clip_trim_in_handle`
- `cargo test -p tench-composer-core trim_clip_in`
- `cargo test -p tench-composer composer_plan_project_timeline_controls_use_real_events_ui_e2e`
- `cargo test -p tench-ui-test ui_automation`
- `git diff --check`
