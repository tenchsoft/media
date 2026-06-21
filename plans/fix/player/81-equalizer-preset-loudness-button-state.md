# Equalizer Loudness Preset Button State

## Source Plan
- `plans/player/equalizer-preset-loudness-button-work-plan.md`

## Gap Analysis
The Loudness preset updates `PlayerState.eq_bands`, but it does not apply the preset to the backend equalizer. The preset path has no backend side effect. See `apps/player/src-tauri/src/ui/app.rs:1371`.

`SetEqPresetNamed` does not update `eq_preset_idx`, so selected-preset state can remain stale after choosing Loudness through the visible preset row. See `apps/player/src-tauri/src/ui/app.rs:1394`.

The current E2E asserts `player.equalizer.preset.loudness` exists but never activates it. It does not verify all five Loudness values, toast, backend values, repeated activation determinism, selecting another preset after Loudness, or close/reopen persistence. See `apps/player/src-tauri/tests/plan_ui_e2e.rs:480`.

## Plan Requirements Not Met
- Loudness must apply all five preset gains to the backend equalizer.
- Loudness must update selected-preset state consistently with the visible preset row.
- Tests must click `player.equalizer.preset.loudness` and verify all five bands.
- Tests must verify the `EQ: Loudness` toast.
- Tests must verify repeated Loudness activation is deterministic.
- Tests must verify selecting another preset after Loudness clears all stale Loudness values.
- Tests must verify Loudness values persist after closing and reopening the equalizer.

## Required Test Shape
- Click Loudness and assert `eq_bands == [4.0, 2.0, 0.0, 2.0, 4.0]`, `eq_preset_idx`, toast, visible labels, and backend gains.
- Click Loudness twice and assert state remains stable.
- Click another preset after Loudness and assert all five bands match the second preset exactly.
- Close and reopen the equalizer and assert Loudness values remain visible.

## Required Changes
- Apply named presets to the backend equalizer path.
- Update `eq_preset_idx` or replace it with a consistent selected-preset model for named preset actions.
- Expose preset/band labels or values through automation.
- Extend equalizer preset E2E coverage for Loudness success, toast, backend dispatch, repeat activation, preset switching, and reopen persistence.

## Verification
- `cargo test -p tench-player --test plan_ui_e2e equalizer_preset_loudness`
- `cargo test -p tench-player`
