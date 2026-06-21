# Timeline Seek Area State

## Source Plan

- `plans/composer/timeline-seek-area-control-work-plan.md`

## Gap Analysis

Timeline seek logic maps the x coordinate to a frame and clamps through `seek_to_frame`, but current E2E coverage only asserts that `composer.timeline.seek` exists. It does not click the ruler or track area, assert `current_frame`, verify playhead/timecode repaint, or test clamp behavior. See `apps/composer/src-tauri/src/ui/mod.rs:819`, `apps/composer/src-tauri/src/ui/state.rs:708`, and `apps/composer/src-tauri/tests/plan_ui_e2e.rs:104`.

Clip click regions are dispatched before the timeline seek path. A click on a rendered clip body selects the clip and returns without moving the playhead through the seek logic, so the "track area containing a clip at that frame" behavior is not guaranteed if the clicked point overlaps a clip body. See `apps/composer/src-tauri/src/ui/mod.rs:805` and `apps/composer/src-tauri/src/ui/mod.rs:819`.

The paused-playback preservation scenario is untested.

## Plan Requirements Not Met

- Ruler clicks must be tested to move `current_frame` based on x geometry.
- Track-area clicks at frames with clips must be tested to select the clip and satisfy the seek contract.
- Clicks beyond the timeline duration must be tested to clamp to the valid frame range.
- Seeking while paused must be tested to preserve paused transport state.
- The interaction priority between clip body selection and seek-area behavior must be made explicit and tested.

## Required Test Shape

- Add a Composer UI automation test that clicks a ruler point and asserts `current_frame` equals the expected mapped frame.
- Click beyond the right edge of the timeline content and assert `current_frame == total_frames - 1`.
- Click a track position that corresponds to an existing clip and assert both the expected frame and selected clip state according to the contract.
- Pause playback, click the seek area, and assert `is_playing` remains false.
- Assert preview timecode and playhead automation state update after seeking.

## Required Changes

- Add timeline seek E2E coverage for ruler, track, clamp, and paused scenarios.
- Define whether clicking a clip body should also seek; if yes, run seek before or alongside clip selection.
- Expose current frame, total frames, playhead position, and timecode through automation if current state access is insufficient.

## Verification

- `cargo test -p tench-composer --test plan_ui_e2e timeline_seek`
- `cargo test -p tench-composer`
