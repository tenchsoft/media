# Subtitles Style Button State

## Source Plan
- `plans/player/subtitles-style-button-work-plan.md`

## Gap Analysis
The current E2E clicks `player.subtitles.style` and asserts the style controls are present, but it does not assert `subtitle_style_open`, current style values, value labels, focus/modal cleanup, or unrelated playback invariants. See `apps/player/src-tauri/tests/plan_ui_e2e.rs:382`.

The style modal does not render all values from state. Text color is hardcoded as `White`, and several style values are not exposed through automation, so tests cannot verify the modal opens with current style values. See `apps/player/src-tauri/src/ui/paint_overlays.rs:485`.

Opening the style modal uses `ShowSubtitleStyle`, which simply sets `subtitle_style_open = true`. There is no test for repeated open after style changes or for interaction with another subtitle modal already open. See `apps/player/src-tauri/src/ui/app.rs:1320`.

## Plan Requirements Not Met
- Tests must verify opening Style sets `subtitle_style_open == true`.
- Tests must verify the modal displays current subtitle style values.
- Tests must verify repeated open after style changes shows updated values.
- Tests must verify opening Style does not change media path, playback time, paused state, playlist, subtitle tracks, or selected encoding.
- Automation must expose subtitle style values for assertions.

## Required Test Shape
- Mutate subtitle style state, click `player.subtitles.style`, and assert visible labels/automation values match state.
- Open Style while another subtitle modal is open and assert the documented overlay behavior.
- Close and reopen after style changes and assert the displayed values remain current.
- Snapshot unrelated player state before opening Style and assert it remains unchanged.

## Required Changes
- Render every style label from `subtitle_style` state.
- Expose style values through automation.
- Add Subtitles Style button coverage for open state, value hydration, repeated open, and unrelated-state invariants.

## Verification
- `cargo test -p tench-player --test plan_ui_e2e subtitles_style_button`
- `cargo test -p tench-player`
