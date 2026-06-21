# Equalizer 16kHz Plus Button State

## Source Plan
- `plans/player/equalizer-16khz-plus-button-work-plan.md`

## Gap Analysis
The 16kHz plus button updates `PlayerState.eq_bands[4]`, but the backend equalizer side effect is still a TODO. No test can prove the GStreamer equalizer receives the new 16kHz value. See `apps/player/src-tauri/src/ui/app.rs:1154`.

The current E2E only clicks the 16kHz minus button. It asserts that `player.equalizer.band.4.plus` exists but never activates it, so plus behavior, visible dB label, repeated clamp at `12.0`, backend dispatch, preset-then-plus behavior, and close/reopen persistence are unverified. See `apps/player/src-tauri/tests/plan_ui_e2e.rs:474`.

Automation IDs for equalizer buttons are inferred by comparing the registered action value with current state. At the clamp boundary, the plus action value can equal the current value and remains classified as `plus` only by accident; the selector should be based on the rendered control, not the value comparison. See `apps/player/src-tauri/src/ui/app.rs:2392`.

## Plan Requirements Not Met
- 16kHz plus must apply the new gain to the backend equalizer.
- Tests must click `player.equalizer.band.4.plus` and verify the 16kHz gain increases.
- Tests must verify the displayed 16kHz dB value updates after activation.
- Tests must verify repeated increases clamp at the documented maximum.
- Tests must verify applying a preset and then increasing 16kHz changes only band 4.
- Tests must verify the 16kHz value persists after closing and reopening the equalizer.
- Automation must keep the 16kHz plus selector tied to the rendered plus button.

## Required Test Shape
- Open the equalizer, click `player.equalizer.band.4.plus`, and assert `eq_bands[4]`, visible label/value, and backend band 4 value.
- Repeatedly click the plus button until above the maximum would be requested and assert the value remains `12.0`.
- Apply a preset, click 16kHz plus, and assert only `eq_bands[4]` changed from the preset values.
- Close and reopen the equalizer and assert the 16kHz label still shows the updated value.

## Required Changes
- Wire `SetEqBand(4, value)` to the backend equalizer path or a shared audio-effect abstraction.
- Expose equalizer band values through automation labels or values.
- Make equalizer plus/minus automation IDs independent from current value comparison.
- Extend E2E coverage for plus click, clamp, preset interaction, backend dispatch, and reopen persistence.

## Verification
- `cargo test -p tench-player --test plan_ui_e2e equalizer_16khz_plus`
- `cargo test -p tench-player`
