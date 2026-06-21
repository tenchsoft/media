# Clip Drag Move State Fix Plan

## Source Plan

- `plans/composer/clip-drag-move-control-work-plan.md`

## Gap Analysis

Clip drag move is wired through `DragKind::Clip`, but the behavior is not covered
by E2E. Invalid move handling is also risky: the core `move_clip` removes the
clip from the source track before proving the destination track exists.

## Plan Requirements Not Met

- Invalid destination moves can mutate project data before returning an error.
- Moving to an incompatible track type is not validated.
- There is no E2E test that dragging later on the same track updates
  `timeline_in` while preserving duration.
- There is no E2E test that dragging to another compatible track updates source
  and destination track ids correctly.
- There is no E2E test that Snap moves a dropped clip to a nearby clip boundary.
- There is no E2E test that dropping outside all tracks leaves project data
  unchanged.
- There is no automation assertion for drag state, source track id, destination
  track id, frame offset, or final clip bounds.

## Code Review

- `apps/composer/src-tauri/src/ui/mod.rs:520` starts a clip-body drag when the
  pointer is inside a clip rect.
- `apps/composer/src-tauri/src/ui/mod.rs:524` stores `DragKind::Clip` with clip
  id, source track id, and frame offset.
- `apps/composer/src-tauri/src/ui/mod.rs:581` resolves the destination track on
  pointer up.
- `apps/composer/src-tauri/src/ui/mod.rs:593` applies `snap_position` before
  moving the clip.
- `apps/composer/src-tauri/src/ui/state.rs:917` pushes undo and calls the core
  move method.
- `crates/composer-core/src/timeline.rs:522` removes the clip from the source
  before destination track lookup succeeds at `:526`.
- `crates/composer-core/src/timeline.rs:526` does not validate track-kind
  compatibility before inserting the moved clip.

## Test Review

- `apps/composer/src-tauri/tests/plan_ui_e2e.rs:183` uses drag automation only
  for the left splitter.
- Existing plan E2E selects and right-clicks clips, but does not drag a clip
  body.
- `apps/composer/src-tauri/src/ui/state.rs:1225` unit-tests `snap_position`
  directly, not drag/drop integration.

## Required Test Shape

- Drag a clip body later on the same track and assert `timeline_in` changes,
  duration remains unchanged, undo stack grows, and notice is `Clip moved`.
- Drag a clip to another compatible track and assert the source track no longer
  contains it and the destination track does.
- Enable Snap, drag near another clip boundary, and assert the final
  `timeline_in` equals the snapped boundary.
- Drag outside all track lanes and assert the full project timeline snapshot is
  unchanged.
- Attempt an invalid destination at the state/core level and assert the clip
  remains on the source track.

## Required Changes

- Validate destination track existence and compatibility before removing the
  source clip in the core move path.
- Add explicit invalid-drop result handling so UI state can report no mutation
  or a clear notice.
- Expose clip bounds, track ids, drag state, and final clip frame values through
  automation for selector-based drag assertions.

## Verification

- `cargo test -p tench-composer clip_drag_move`
- `cargo test -p tench-composer-core move_clip`
- `cargo test -p tench-composer composer_plan_project_timeline_controls_use_real_events_ui_e2e`
- `cargo test -p tench-ui-test ui_automation`
- `git diff --check`
