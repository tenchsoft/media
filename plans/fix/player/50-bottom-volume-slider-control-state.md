# Bottom Volume Slider Control State

## Source Plan
- `plans/player/bottom-volume-slider-control-work-plan.md`

## Gap Analysis
The existing E2E clicks `player.controls.volume` once and only asserts the capture changed. It does not verify the computed volume, filled bar width, mute-at-zero behavior, far-right 100 percent behavior, drag updates, or backend `set_volume` calls. See `apps/player/src-tauri/tests/plan_ui_e2e.rs:157`.

The volume automation node is a generic slider without a current value, so tests cannot assert the slider value through automation. See `apps/player/src-tauri/src/ui/app.rs:2551`.

The pointer hit handling computes the volume bar x-position from hardcoded control offsets instead of sharing the painted volume rect. This can drift from rendering if spacing or control layout changes, and it is not tested with side-panel or narrow layouts. See `apps/player/src-tauri/src/ui/app.rs:1777` and `apps/player/src-tauri/src/ui/paint_controls.rs:372`.

## Plan Requirements Not Met
- Tests must verify far-left click sets volume to 0 and `is_muted == true`.
- Tests must verify far-right click sets volume to 1.0.
- Tests must verify dragging updates volume continuously and calls backend `set_volume`.
- Tests must verify filled bar width reflects the current volume.
- Automation must expose current volume value and muted state.
- Hit-test geometry must stay in sync with the painted volume slider rect.

## Required Test Shape
- Click the left edge of `player.controls.volume` and assert `volume == 0.0`, `is_muted == true`, backend `set_volume(0.0)`, and empty fill.
- Click the right edge and assert `volume == 1.0`, `is_muted == false`, backend `set_volume(1.0)`, and full fill.
- Drag across the slider and assert intermediate backend calls and state updates.
- Open a side panel or resize and assert the click bounds still map to the painted slider.

## Required Changes
- Expose volume value and muted state in the volume slider automation node.
- Share painted volume-slider geometry with pointer hit handling or add geometry assertions to prevent drift.
- Add fake backend volume-call assertions.
- Extend `plan_ui_e2e` Volume coverage for left/right clicks, drag, fill width, backend sync, mute-at-zero, and layout changes.

## Verification
- `cargo test -p tench-player --test plan_ui_e2e bottom_volume_slider`
- `cargo test -p tench-player`
