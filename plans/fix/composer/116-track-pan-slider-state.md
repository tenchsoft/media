# Track Pan Slider State

## Source Plan

- `plans/composer/track-pan-slider-control-work-plan.md`

## Gap Analysis

The pan slider click region sends `(track.pan + 0.1).clamp(-1.0, 1.0)`, but the `SetTrackPan` handler writes the incoming value directly. Programmatic or stale actions can bypass the supported pan range. See `apps/composer/src-tauri/src/ui/right_panel.rs:278` and `apps/composer/src-tauri/src/ui/mod.rs:395`.

The current E2E coverage clicks `composer.track.pan` and only asserts that the capture changed. It does not assert the selected track's pan value, the visible value, fill geometry, empty-state behavior, or clamp behavior beyond the range. See `apps/composer/src-tauri/tests/plan_ui_e2e.rs:293`.

The plan's mute/header synchronization scenario is not covered in the pan-slider flow.

## Plan Requirements Not Met

- Pan writes must be clamped in the action handler.
- Adjusting pan with a selected track must be tested to update both project data and visible inspector value.
- No-selected-track behavior must be tested to preserve empty state and avoid mutation.
- Dragging or dispatching beyond the pan range must be tested to clamp to `-1.0..=1.0`.
- Mute/header synchronization must be covered alongside the audio inspector controls.

## Required Test Shape

- Add a Composer UI automation test that opens the Audio inspector, activates `composer.track.pan`, and asserts the selected track's pan value plus visible label/fill update.
- Clear selection, open Audio inspector, and assert pan controls do not mutate any track.
- Dispatch or simulate pan values below `-1.0` and above `1.0` and assert clamping.
- Toggle mute in the Audio inspector and assert the track header M button remains synchronized.

## Required Changes

- Clamp `SetTrackPan` in the dispatch handler.
- Add pan-slider E2E coverage for selected track, empty state, clamping, and visible value/fill.
- Expose pan value and slider fill state through automation if current state access is insufficient.

## Verification

- `cargo test -p tench-composer --test plan_ui_e2e track_pan`
- `cargo test -p tench-composer`
