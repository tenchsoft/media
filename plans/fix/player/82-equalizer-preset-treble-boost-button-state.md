# Equalizer Treble Boost Preset Button State

## Source Plan
- `plans/player/equalizer-preset-treble-boost-button-work-plan.md`

## Gap Analysis
The Treble Boost preset updates `PlayerState.eq_bands`, but it does not apply the preset to the backend equalizer. The preset path has no backend side effect. See `apps/player/src-tauri/src/ui/app.rs:1371`.

`SetEqPresetNamed` does not update `eq_preset_idx`, so selected-preset state can remain stale after choosing Treble Boost through the visible preset row. See `apps/player/src-tauri/src/ui/app.rs:1394`.

The current E2E asserts `player.equalizer.preset.treble_boost` exists but never activates it. It does not verify all five Treble Boost values, toast, backend values, repeated activation determinism, selecting another preset after Treble Boost, or close/reopen persistence. See `apps/player/src-tauri/tests/plan_ui_e2e.rs:478`.

## Plan Requirements Not Met
- Treble Boost must apply all five preset gains to the backend equalizer.
- Treble Boost must update selected-preset state consistently with the visible preset row.
- Tests must click `player.equalizer.preset.treble_boost` and verify all five bands.
- Tests must verify the `EQ: Treble Boost` toast.
- Tests must verify repeated Treble Boost activation is deterministic.
- Tests must verify selecting another preset after Treble Boost clears all stale Treble Boost values.
- Tests must verify Treble Boost values persist after closing and reopening the equalizer.

## Required Test Shape
- Click Treble Boost and assert `eq_bands == [0.0, 0.0, 0.0, 4.0, 6.0]`, `eq_preset_idx`, toast, visible labels, and backend gains.
- Click Treble Boost twice and assert state remains stable.
- Click another preset after Treble Boost and assert all five bands match the second preset exactly.
- Close and reopen the equalizer and assert Treble Boost values remain visible.

## Required Changes
- Apply named presets to the backend equalizer path.
- Update `eq_preset_idx` or replace it with a consistent selected-preset model for named preset actions.
- Expose preset/band labels or values through automation.
- Extend equalizer preset E2E coverage for Treble Boost success, toast, backend dispatch, repeat activation, preset switching, and reopen persistence.

## Verification
- `cargo test -p tench-player --test plan_ui_e2e equalizer_preset_treble_boost`
- `cargo test -p tench-player`
