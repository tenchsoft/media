# Equalizer 16kHz Minus Button State

## Source Plan
- `plans/player/equalizer-16khz-minus-button-work-plan.md`

## Gap Analysis
The 16kHz minus button updates `PlayerState.eq_bands[4]`, but the backend equalizer side effect is still a TODO. No test can prove the GStreamer equalizer receives the new 16kHz value. See `apps/player/src-tauri/src/ui/app.rs:1154`.

The current E2E clicks `player.equalizer.band.4.minus` once and asserts the state decreased. It does not verify the visible dB label, repeated clamp at `-12.0`, backend dispatch, preset-then-minus behavior, or close/reopen persistence. See `apps/player/src-tauri/tests/plan_ui_e2e.rs:489`.

Automation IDs for equalizer buttons are inferred by comparing the registered action value with current state. At the clamp boundary, the minus action value can equal the current value and be exposed as `plus`, so repeated clamp testing may lose the minus selector. See `apps/player/src-tauri/src/ui/app.rs:2392`.

## Plan Requirements Not Met
- 16kHz minus must apply the new gain to the backend equalizer.
- Tests must verify the displayed 16kHz dB value updates after activation.
- Tests must verify repeated decreases clamp at the documented minimum.
- Tests must verify applying a preset and then decreasing 16kHz changes only band 4.
- Tests must verify the 16kHz value persists after closing and reopening the equalizer.
- Automation must keep the 16kHz minus selector stable at the clamp boundary.

## Required Test Shape
- Open the equalizer, click `player.equalizer.band.4.minus`, and assert `eq_bands[4]`, visible label/value, and backend band 4 value.
- Repeatedly click the minus button until below the minimum would be requested and assert the value remains `-12.0`.
- Apply a preset, click 16kHz minus, and assert only `eq_bands[4]` changed from the preset values.
- Close and reopen the equalizer and assert the 16kHz label still shows the updated value.

## Required Changes
- Wire `SetEqBand(4, value)` to the backend equalizer path or a shared audio-effect abstraction.
- Expose equalizer band values through automation labels or values.
- Make equalizer plus/minus automation IDs independent from current value comparison.
- Extend E2E coverage for clamp, preset interaction, backend dispatch, and reopen persistence.

## Verification
- `cargo test -p tench-player --test plan_ui_e2e equalizer_16khz_minus`
- `cargo test -p tench-player`
