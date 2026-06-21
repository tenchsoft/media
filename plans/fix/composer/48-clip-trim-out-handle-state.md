# Clip Trim Out Handle State Fix Plan

## Source Plan

- `plans/composer/clip-trim-out-handle-control-work-plan.md`

## Gap Analysis

Trim-out drag is wired, but media-range validation for rightward lengthening is
missing. The behavior is also untested through UI drag automation.

## Plan Requirements Not Met

- Dragging the out handle right can lengthen beyond available media range
  because no media limit is validated.
- There is no E2E test that dragging the out handle left shortens duration.
- There is no E2E test that dragging the out handle right lengthens duration
  only when media range allows.
- There is no E2E test that dragging before the in point rejects the operation
  and preserves old trim values.
- There is no E2E test that Snap moves trim-out to a nearby clip edge.
- There is no test that selection remains on the trimmed clip after trim-out.
- Automation does not expose trim drag state, resulting media range, duration,
  selection, or clip rect geometry after trim.

## Code Review

- `apps/composer/src-tauri/src/ui/mod.rs:500` hit-tests trim handles before clip
  body drag.
- `apps/composer/src-tauri/src/ui/mod.rs:508` starts `DragKind::TrimOut`.
- `apps/composer/src-tauri/src/ui/mod.rs:613` maps release x to a frame.
- `apps/composer/src-tauri/src/ui/mod.rs:614` applies `snap_position`.
- `apps/composer/src-tauri/src/ui/state.rs:951` pushes undo and calls the core
  trim-out method.
- `crates/composer-core/src/timeline.rs:492` rejects out points at or before the
  in point.
- `crates/composer-core/src/timeline.rs:496` updates duration from the new out
  point.
- `crates/composer-core/src/timeline.rs:498` can extend `media_out` without a
  source media limit check.

## Test Review

- `apps/composer/src-tauri/tests/plan_ui_e2e.rs:102` only asserts
  `composer.timeline.clip.0.trim_out` is present.
- Existing drag E2E coverage is for the left splitter, not clip trim handles.
- Existing `snap_position` unit coverage does not verify trim integration.

## Required Test Shape

- Drag the trim-out handle left and assert duration decreases while
  `timeline_in` remains unchanged.
- Drag the trim-out handle right within media range and assert duration
  increases according to the media range rule.
- Drag trim-out before the clip in point and assert no mutation plus
  `New out-point must be after in-point`.
- Enable Snap, drag near another clip edge, and assert trim-out lands on the
  snapped frame.
- Assert selection remains on the trimmed clip and clip rect bounds match the
  new frame range after a valid trim.

## Required Changes

- Define and enforce media-range limits for rightward trim-out expansion.
- Return explicit trim results so UI can distinguish valid trim, invalid
  boundary, and media-range rejection.
- Expose clip frame range, media range, duration, selection, and trim handle
  bounds through automation.

## Verification

- `cargo test -p tench-composer clip_trim_out_handle`
- `cargo test -p tench-composer-core trim_clip_out`
- `cargo test -p tench-composer composer_plan_project_timeline_controls_use_real_events_ui_e2e`
- `cargo test -p tench-ui-test ui_automation`
- `git diff --check`
