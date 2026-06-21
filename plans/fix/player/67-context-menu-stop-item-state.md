# Context Menu Stop Item State

## Source Plan
- `plans/player/context-menu-stop-item-work-plan.md`

## Gap Analysis
The current E2E only asserts `player.context.stop` is present. It never activates the Stop item, so backend stop dispatch, `is_playing = false`, menu closure, repaint, and one-command behavior are unverified. See `apps/player/src-tauri/tests/plan_ui_e2e.rs:545`.

The Stop handler calls `backend.stop()` only when a backend exists, but no test double or observable automation state proves the backend received exactly one stop command. See `apps/player/src-tauri/src/ui/app.rs:1483`.

The Play/Pause menu label after Stop is not tested. A stale `Pause` label after stopping would not be caught because tests only check selector presence. See `apps/player/src-tauri/src/ui/app.rs:1717`.

Boundary clicks around the Stop row are untested, so a near-edge click could dispatch Play/Pause, Screenshot, or no command without being caught. See `apps/player/src-tauri/src/ui/app.rs:1680`.

## Plan Requirements Not Met
- Tests must activate Stop through `player.context.stop`.
- Tests must verify `is_playing` becomes false after Stop.
- Tests must prove the backend receives exactly one stop command.
- Tests must verify the context menu closes after Stop activation.
- Tests must verify the next context menu shows the correct Play/Pause label after Stop.
- Tests must verify no neighboring context item command changes screenshot, fullscreen, aspect, repeat, shuffle, media path, or playlist state.
- Tests must cover boundary clicks around the Stop row.

## Required Test Shape
- Start from `is_playing == true`, open the menu, click `player.context.stop`, and assert `is_playing == false`.
- Use a backend command spy to assert one stop request and no play/pause request.
- Assert context-menu selectors are absent after activation.
- Reopen the menu and assert the Play/Pause label is `Play`.
- Snapshot unrelated context-controlled state and assert it remains unchanged.

## Required Changes
- Add a backend stop spy or reusable backend-command test hook.
- Add context-menu Stop coverage to `plan_ui_e2e` or a focused context-menu automation test.
- Expose menuitem label/value assertions if the current automation helper cannot inspect labels.
- Add row-boundary tests for the Stop menu item.

## Verification
- `cargo test -p tench-player --test plan_ui_e2e context_menu_stop`
- `cargo test -p tench-player`
