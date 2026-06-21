# Bottom Speed Menu Toggle Button State

## Source Plan
- `plans/player/bottom-speed-menu-toggle-button-work-plan.md`

## Gap Analysis
The existing E2E opens the speed menu and clicks speed options, but it does not assert that toggling the Speed button a second time closes the menu or that opening/closing the menu leaves `playback_rate` unchanged. See `apps/player/src-tauri/tests/plan_ui_e2e.rs:160`.

Speed option clicks set the rate and close the menu, but the test does not assert the menu is absent after each selection. It reopens the menu for the next option without verifying close behavior directly. See `apps/player/src-tauri/src/ui/app.rs:785`.

The speed menu anchor is stored during paint from the current controls-bar geometry, but tests do not verify the menu is anchored above the rate label when a drawer or AI side panel changes the controls width. See `apps/player/src-tauri/src/ui/paint_controls.rs:383` and `apps/player/src-tauri/src/ui/paint_controls.rs:518`.

Automation does not expose `show_speed_menu`, current anchor, or rate-label value through `player.controls.speed_menu`, so layout and no-rate-change assertions depend on internal state only. See `apps/player/src-tauri/src/ui/app.rs:2361`.

## Plan Requirements Not Met
- Tests must verify toggling the Speed button open and closed without changing playback rate.
- Tests must verify selecting a speed closes the menu.
- Tests must verify the menu anchors to the rate label with side panels open.
- Automation must expose speed menu open state, anchor, or current rate label.

## Required Test Shape
- Record `playback_rate`, click `player.controls.speed_menu`, assert `show_speed_menu == true` and rate unchanged.
- Click `player.controls.speed_menu` again and assert `show_speed_menu == false`, speed options absent, and rate unchanged.
- Open the menu, click a speed option, and assert the rate changes and speed option selectors are absent.
- Open a drawer or AI panel, open the speed menu, and assert the menu bounds are anchored above the resized controls-bar rate label.

## Required Changes
- Expose speed menu open state, anchor, or rate label through automation.
- Extend `plan_ui_e2e` Speed Menu coverage for toggle close, no-rate-change, option-close, and side-panel anchor behavior.

## Verification
- `cargo test -p tench-player --test plan_ui_e2e bottom_speed_menu_toggle`
- `cargo test -p tench-player`
