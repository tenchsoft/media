# Equalizer Flat Preset Button State

## Source Plan
- `plans/player/equalizer-preset-flat-button-work-plan.md`

## Gap Analysis
The Flat preset updates `PlayerState.eq_bands`, but it does not apply the preset to the backend equalizer. The preset path has no backend side effect. See `apps/player/src-tauri/src/ui/app.rs:1371`.

`SetEqPresetNamed` does not update `eq_preset_idx`, so selected-preset state can remain stale after choosing Flat through the visible preset row. See `apps/player/src-tauri/src/ui/app.rs:1394`.

The current E2E asserts `player.equalizer.preset.flat` exists but never activates it. It does not verify all five Flat values, toast, backend values, repeated activation determinism, selecting another preset after Flat, or close/reopen persistence. See `apps/player/src-tauri/tests/plan_ui_e2e.rs:476`.

## Plan Requirements Not Met
- Flat must apply all five preset gains to the backend equalizer.
- Flat must update selected-preset state consistently with the visible preset row.
- Tests must click `player.equalizer.preset.flat` and verify all five bands become `0.0`.
- Tests must verify the `EQ: Flat` toast.
- Tests must verify repeated Flat activation is deterministic.
- Tests must verify selecting another preset after Flat clears all stale Flat values.
- Tests must verify Flat values persist after closing and reopening the equalizer.

## Required Test Shape
- Change bands away from zero, click Flat, and assert `eq_bands`, `eq_preset_idx`, toast, visible labels, and backend gains.
- Click Flat twice and assert state remains stable.
- Click another preset after Flat and assert all five bands match the second preset exactly.
- Close and reopen the equalizer and assert Flat values remain visible.

## Required Changes
- Apply named presets to the backend equalizer path.
- Update `eq_preset_idx` or replace it with a consistent selected-preset model for named preset actions.
- Expose preset/band labels or values through automation.
- Extend equalizer preset E2E coverage for Flat success, toast, backend dispatch, repeat activation, preset switching, and reopen persistence.

## Verification
- `cargo test -p tench-player --test plan_ui_e2e equalizer_preset_flat`
- `cargo test -p tench-player`
