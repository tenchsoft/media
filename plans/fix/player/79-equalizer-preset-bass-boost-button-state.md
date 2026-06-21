# Equalizer Bass Boost Preset Button State

## Source Plan
- `plans/player/equalizer-preset-bass-boost-button-work-plan.md`

## Gap Analysis
The Bass Boost preset updates `PlayerState.eq_bands`, but it does not apply the preset to the backend equalizer. The preset path has no backend side effect. See `apps/player/src-tauri/src/ui/app.rs:1371`.

`SetEqPresetNamed` does not update `eq_preset_idx`, so selected-preset state can remain stale after choosing Bass Boost through the visible preset row. See `apps/player/src-tauri/src/ui/app.rs:1394`.

The current E2E asserts the Bass Boost band array but does not assert the toast, backend values, repeated activation determinism, selecting another preset after Bass Boost, or close/reopen persistence. See `apps/player/src-tauri/tests/plan_ui_e2e.rs:493`.

## Plan Requirements Not Met
- Bass Boost must apply all five preset gains to the backend equalizer.
- Bass Boost must update selected-preset state consistently with the visible preset row.
- Tests must verify the `EQ: Bass Boost` toast.
- Tests must verify repeated Bass Boost activation is deterministic.
- Tests must verify selecting another preset after Bass Boost clears all stale Bass Boost values.
- Tests must verify Bass Boost values persist after closing and reopening the equalizer.

## Required Test Shape
- Click `player.equalizer.preset.bass_boost` and assert `eq_bands`, `eq_preset_idx`, toast, visible labels, and backend gains.
- Click Bass Boost twice and assert state remains the same with no duplicate or stale transition.
- Click another preset after Bass Boost and assert all five bands match the second preset exactly.
- Close and reopen the equalizer and assert Bass Boost values remain visible.

## Required Changes
- Apply named presets to the backend equalizer path.
- Update `eq_preset_idx` or replace it with a consistent selected-preset model for named preset actions.
- Expose preset/band labels or values through automation.
- Extend equalizer preset E2E coverage for toast, backend dispatch, repeat activation, preset switching, and reopen persistence.

## Verification
- `cargo test -p tench-player --test plan_ui_e2e equalizer_preset_bass_boost`
- `cargo test -p tench-player`
