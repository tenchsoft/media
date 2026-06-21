# Equalizer Vocal Preset Button State

## Source Plan
- `plans/player/equalizer-preset-vocal-button-work-plan.md`

## Gap Analysis
The visible Vocal preset uses the named preset path, but the shared preset table defines `Voice` with different band values. `SetEqPresetNamed("Vocal")` applies `[-2.0, 0.0, 4.0, 3.0, -1.0]`, while `EqPreset::PRESETS` contains `Voice` with `[-2.0, 0.0, 4.0, 3.0, 0.0]`. See `apps/player/src-tauri/src/ui/app.rs:1381` and `apps/player/src-tauri/src/ui/state.rs:336`.

The Vocal preset updates `PlayerState.eq_bands`, but it does not apply the preset to the backend equalizer. The preset path has no backend side effect. See `apps/player/src-tauri/src/ui/app.rs:1371`.

`SetEqPresetNamed` does not update `eq_preset_idx`, so selected-preset state can remain stale after choosing Vocal through the visible preset row. See `apps/player/src-tauri/src/ui/app.rs:1394`.

The current E2E asserts `player.equalizer.preset.vocal` exists but never activates it. It does not verify all five Vocal values, toast, backend values, repeated activation determinism, selecting another preset after Vocal, or close/reopen persistence. See `apps/player/src-tauri/tests/plan_ui_e2e.rs:479`.

## Plan Requirements Not Met
- Vocal preset name and band values must be defined in one consistent source of truth.
- Vocal must apply all five preset gains to the backend equalizer.
- Vocal must update selected-preset state consistently with the visible preset row.
- Tests must click `player.equalizer.preset.vocal` and verify all five bands.
- Tests must verify the `EQ: Vocal` toast.
- Tests must verify repeated Vocal activation is deterministic.
- Tests must verify selecting another preset after Vocal clears all stale Vocal values.
- Tests must verify Vocal values persist after closing and reopening the equalizer.

## Required Test Shape
- Click Vocal and assert the documented Vocal band array, `eq_preset_idx`, toast, visible labels, and backend gains.
- Assert the visible preset row, selected preset state, and shared preset table all use the same Vocal name and values.
- Click Vocal twice and assert state remains stable.
- Click another preset after Vocal and assert all five bands match the second preset exactly.
- Close and reopen the equalizer and assert Vocal values remain visible.

## Required Changes
- Consolidate named preset definitions with `EqPreset::PRESETS`.
- Apply named presets to the backend equalizer path.
- Update `eq_preset_idx` or replace it with a consistent selected-preset model for named preset actions.
- Expose preset/band labels or values through automation.
- Extend equalizer preset E2E coverage for Vocal success, toast, backend dispatch, repeat activation, preset switching, and reopen persistence.

## Verification
- `cargo test -p tench-player --test plan_ui_e2e equalizer_preset_vocal`
- `cargo test -p tench-player`
