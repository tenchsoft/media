# 0.25x Speed Option Button State

## Source Plan
- `plans/player/speed-menu-option-0-25x-button-work-plan.md`

## Gap Analysis
The current E2E verifies that clicking `player.speed.0_25x` sets `PlayerState.playback_rate` to `0.25`, but it does not assert backend rate dispatch, menu closure, selected-option highlight after reopening, or playing/paused behavior. See `apps/player/src-tauri/tests/plan_ui_e2e.rs:176`.

`SetSpeed(0.25)` calls `backend.set_playback_rate(0.25)` only when a backend exists, but there is no backend spy proving the rate was applied to audio/video playback. See `apps/player/src-tauri/src/ui/app.rs:785`.

The selected speed highlight is drawn by text color only and is not exposed as automation selected/value state, so tests cannot assert that 0.25x is highlighted through the UI tree. See `apps/player/src-tauri/src/ui/paint_controls.rs:532`.

## Plan Requirements Not Met
- Tests must verify backend playback rate is set to `0.25`.
- Tests must verify the speed menu closes after selecting 0.25x.
- Tests must verify reopening the menu marks 0.25x as selected.
- Tests must verify selecting 0.25x while playing changes playback immediately.
- Tests must verify selecting 0.25x while paused preserves paused state and the next play uses 0.25x.

## Required Test Shape
- Attach a backend rate spy, select 0.25x, and assert one `set_playback_rate(0.25)` call.
- Assert `show_speed_menu == false` and speed option selectors are absent after selection.
- Reopen the speed menu and assert `player.speed.0_25x` exposes selected state/value.
- Repeat while `is_playing == true` and while paused, asserting playback state invariants.

## Required Changes
- Add a backend playback-rate test hook.
- Expose selected state for speed menu options through automation.
- Extend speed menu E2E coverage for menu closure, selected highlight, backend dispatch, and playing/paused behavior.

## Verification
- `cargo test -p tench-player --test plan_ui_e2e speed_menu_option_0_25x`
- `cargo test -p tench-player`
