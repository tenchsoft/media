# Context Menu Aspect Item State

## Source Plan
- `plans/player/context-menu-aspect-item-work-plan.md`

## Gap Analysis
The current E2E clicks `player.context.aspect` but does not assert that aspect mode changed, that the video layout repaint reflects the new mode, that the menu closed, or that exactly one command ran. See `apps/player/src-tauri/tests/plan_ui_e2e.rs:571`.

Context-menu boundary clicks are not covered. The pointer path returns while leaving the menu open when a click lands inside the menu rectangle but outside an actual item row, such as the bottom padding or info/header area. See `apps/player/src-tauri/src/ui/app.rs:1680`.

Dynamic context-menu labels are only checked for selector presence. Tests do not verify that the Aspect label reflects the current aspect mode before and after cycling. See `apps/player/src-tauri/src/ui/app.rs:1734` and `apps/player/src-tauri/src/ui/app.rs:2246`.

## Plan Requirements Not Met
- Tests must verify the Aspect item cycles aspect mode exactly once per activation.
- Tests must verify the context menu closes after the Aspect command.
- Tests must verify the rendered layout or capture changes according to the new aspect mode.
- Tests must verify top/bottom context-menu boundary clicks choose a valid item or dismiss deterministically.
- Tests must verify the dynamic Aspect label is accurate after aspect state changes.

## Required Test Shape
- Open the menu, capture `aspect_mode`, click `player.context.aspect`, and assert the next aspect mode is selected.
- Assert `player.context.aspect` and `player.context.dismiss` are absent after the click.
- Reopen the menu and assert the Aspect menuitem label contains the new mode label.
- Click inside the menu header/padding and near the bottom boundary, then assert either a documented dismiss occurs or the intended item is invoked.
- Count state changes across one click to ensure repeat/aspect/shuffle/playback fields do not receive extra commands.

## Required Changes
- Add value-level context Aspect coverage to `plan_ui_e2e` or a focused context-menu automation test.
- Update context-menu pointer handling so clicks inside non-item menu areas dismiss or no-op according to documented behavior without leaving ambiguous stale state.
- Expose menuitem label/value assertions if the current automation helper cannot inspect labels.

## Verification
- `cargo test -p tench-player --test plan_ui_e2e context_menu_aspect`
- `cargo test -p tench-player`
