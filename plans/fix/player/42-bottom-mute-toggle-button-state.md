# Bottom Mute Toggle Button State

## Source Plan
- `plans/player/bottom-mute-toggle-button-work-plan.md`

## Gap Analysis
Zero-volume unmute is ambiguous. `set_volume(0.0)` sets `is_muted = true`, but clicking Mute then flips `is_muted` to false while `volume` remains 0.0, so the label can show `Vol` even though effective audio output is silent. See `apps/player/src-tauri/src/ui/state.rs:1011` and `apps/player/src-tauri/src/ui/state.rs:1016`.

The existing E2E clicks `player.controls.mute` only as part of a selector-presence loop. It does not verify `is_muted`, preserved volume, label changes between `Vol` and `Mute`, zero-volume behavior, or backend `set_muted` calls. See `apps/player/src-tauri/tests/plan_ui_e2e.rs:230`.

The visible mute label is drawn from `PlayerState.is_muted`, but automation uses the generic action label and does not expose the current `Vol`/`Mute` display value or effective muted state. See `apps/player/src-tauri/src/ui/paint_controls.rs:357` and `apps/player/src-tauri/src/ui/app.rs:2544`.

## Plan Requirements Not Met
- Zero-volume unmute behavior must be clearly defined and represented in UI state.
- Tests must verify mute toggles state and label from nonzero volume.
- Tests must verify volume is preserved or restored according to product rules.
- Tests must verify zero-volume mute/unmute behavior is clear.
- Tests must verify backend `set_muted` receives the same state as the UI while playing and paused.
- Automation must expose current mute label or effective muted state.

## Required Test Shape
- Set volume to a nonzero value, click `player.controls.mute`, and assert `is_muted == true`, volume is preserved, label/value is `Mute`, and backend `set_muted(true)` was called.
- Click again and assert `is_muted == false`, volume restoration/preservation follows the documented rule, label/value is `Vol`, and backend `set_muted(false)` was called.
- Set volume to zero, click mute/unmute, and assert the UI does not show an audible `Vol` state unless volume is restored to a nonzero value.
- Repeat while paused and playing with a fake backend.

## Required Changes
- Define and implement zero-volume unmute semantics.
- Expose mute label or effective muted state through `player.controls.mute`.
- Add fake backend assertions for `set_muted`.
- Extend `plan_ui_e2e` mute coverage for nonzero volume, zero volume, label changes, backend sync, playing, and paused states.

## Verification
- `cargo test -p tench-player --test plan_ui_e2e bottom_mute_toggle`
- `cargo test -p tench-player`
