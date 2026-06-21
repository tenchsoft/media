# Track Volume Slider State

## Source Plan

- `plans/composer/track-volume-slider-control-work-plan.md`

## Gap Analysis

The volume slider click region sends `(track.volume + 0.1).min(2.0)`, but the `SetTrackVolume` handler writes the incoming value directly. Programmatic or stale actions can bypass the supported volume range. See `apps/composer/src-tauri/src/ui/right_panel.rs:241` and `apps/composer/src-tauri/src/ui/mod.rs:389`.

The current E2E coverage clicks `composer.track.volume` and only asserts that the capture changed. It does not assert the selected track's volume value, the visible value, filled slider geometry, empty-state behavior, or clamp behavior beyond the range. See `apps/composer/src-tauri/tests/plan_ui_e2e.rs:293`.

The plan's mute/header synchronization scenario is not covered in the volume-slider flow.

## Plan Requirements Not Met

- Volume writes must be clamped in the action handler.
- Adjusting volume with a selected track must be tested to update both project data and visible inspector value.
- No-selected-track behavior must be tested to preserve empty state and avoid mutation.
- Dragging or dispatching beyond the volume range must be tested to clamp to the supported min/max.
- Mute/header synchronization must be covered alongside the audio inspector controls.

## Required Test Shape

- Add a Composer UI automation test that opens the Audio inspector, activates `composer.track.volume`, and asserts the selected track's volume value plus visible label/fill update.
- Clear selection, open Audio inspector, and assert volume controls do not mutate any track.
- Dispatch or simulate volume values below the minimum and above `2.0` and assert clamping.
- Toggle mute in the Audio inspector and assert the track header M button remains synchronized.

## Required Changes

- Clamp `SetTrackVolume` in the dispatch handler.
- Add volume-slider E2E coverage for selected track, empty state, clamping, and visible value/fill.
- Expose volume value and slider fill state through automation if current state access is insufficient.

## Verification

- `cargo test -p tench-composer --test plan_ui_e2e track_volume`
- `cargo test -p tench-composer`
