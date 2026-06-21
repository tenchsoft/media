# Equalizer 250Hz Plus Button State

## Source Plan
- `plans/player/equalizer-250hz-plus-button-work-plan.md`

## Gap Analysis
The 250Hz plus button updates `PlayerState.eq_bands[1]`, but the backend equalizer side effect is still a TODO. No test can prove the backend receives the new 250Hz value. See `apps/player/src-tauri/src/ui/app.rs:1154`.

The current E2E asserts `player.equalizer.band.1.plus` exists but never activates it. It does not verify state change, visible dB label, repeated clamp at `12.0`, backend dispatch, preset-then-plus behavior, or close/reopen persistence. See `apps/player/src-tauri/tests/plan_ui_e2e.rs:469`.

Automation IDs for equalizer buttons are inferred from action value versus current state instead of the rendered plus/minus control, which makes clamp-boundary assertions fragile. See `apps/player/src-tauri/src/ui/app.rs:2392`.

## Plan Requirements Not Met
- 250Hz plus must apply the new gain to the backend equalizer.
- Tests must click `player.equalizer.band.1.plus` and verify the 250Hz gain increases.
- Tests must verify the displayed 250Hz dB value updates after activation.
- Tests must verify repeated increases clamp at the documented maximum.
- Tests must verify applying a preset and then increasing 250Hz changes only band 1.
- Tests must verify the 250Hz value persists after closing and reopening the equalizer.
- Automation must keep the 250Hz plus selector tied to the rendered plus button.

## Required Test Shape
- Open the equalizer, click `player.equalizer.band.1.plus`, and assert `eq_bands[1]`, visible label/value, and backend band 1 value.
- Repeatedly click the plus button and assert the value clamps at `12.0`.
- Apply a preset, click 250Hz plus, and assert only `eq_bands[1]` changed from the preset values.
- Close and reopen the equalizer and assert the 250Hz label still shows the updated value.

## Required Changes
- Wire `SetEqBand(1, value)` to the backend equalizer path or a shared audio-effect abstraction.
- Expose equalizer band values through automation labels or values.
- Make equalizer plus/minus automation IDs independent from current value comparison.
- Extend E2E coverage for click, clamp, preset interaction, backend dispatch, and reopen persistence.

## Verification
- `cargo test -p tench-player --test plan_ui_e2e equalizer_250hz_plus`
- `cargo test -p tench-player`
