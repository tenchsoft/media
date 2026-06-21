# Context Menu Play Pause Item State

## Source Plan
- `plans/player/context-menu-play-pause-item-work-plan.md`

## Gap Analysis
The current E2E clicks `player.context.play_pause` and only asserts the capture changed. It does not assert `is_playing`, backend play/pause dispatch, menu closure, or that exactly one toggle happened. See `apps/player/src-tauri/tests/plan_ui_e2e.rs:557`.

The context item delegates to the shared `ClickAction::PlayPause` path, whose backend/UI ordering gap is already captured in `plans/fix/player/43-bottom-play-pause-button-state.md`. This plan still needs context-menu-specific verification for the entry point and dynamic label. See `apps/player/src-tauri/src/ui/app.rs:1482`.

Dynamic Play/Pause labels are not value-tested. The menu label is computed from `state.is_playing`, but tests only assert selector presence and would not catch a stale Play/Pause label after state changes. See `apps/player/src-tauri/src/ui/app.rs:1717`.

## Plan Requirements Not Met
- Tests must verify context-menu Play/Pause toggles playback exactly once.
- Tests must verify the context menu closes after activation.
- Tests must verify the Play/Pause menu label matches current playback state before and after toggling.
- Tests must prove the backend receives the matching play or pause command.
- Tests must verify no neighboring context item command changes aspect, repeat, shuffle, media path, or playlist.

## Required Test Shape
- Open the menu with playback stopped, assert label `Play`, click `player.context.play_pause`, and assert `is_playing == true`.
- Reopen the menu, assert label `Pause`, click again, and assert `is_playing == false`.
- Use the shared play/pause backend test hook from the bottom button fix to assert exactly one backend command per click.
- Assert context-menu selectors are absent after each activation.
- Snapshot unrelated context-controlled state and assert it remains unchanged.

## Required Changes
- Add context-menu Play/Pause coverage to `plan_ui_e2e` or a focused context-menu automation test.
- Reuse or add the backend command spy required by `plans/fix/player/43-bottom-play-pause-button-state.md`.
- Expose menuitem label/value assertions if the current automation helper cannot inspect labels.

## Verification
- `cargo test -p tench-player --test plan_ui_e2e context_menu_play_pause`
- `cargo test -p tench-player`
