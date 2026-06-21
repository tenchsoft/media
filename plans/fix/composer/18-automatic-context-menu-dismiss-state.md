# Automatic Context Menu Dismiss State Fix Plan

## Source Plan

- `plans/composer/automatic-context-menu-dismiss-behavior-work-plan.md`

## Gap Analysis

Composer prioritizes context-menu hit testing and clears the menu on outside
click, but the behavior is not fully verified. Disabled item handling, selection
preservation, pointer-position placement, and second-menu replacement need
targeted tests.

## Plan Requirements Not Met

- Tests do not assert the context menu appears at the right-click pointer
  location.
- Tests do not click outside the menu and verify the menu closes while the clip
  remains selected.
- Disabled menu item clicks are not tested for no action dispatch.
- Opening a second context menu is not tested for replacing the previous menu.
- Automation does not expose context-menu position as a structured value.

## Code Review

- `apps/composer/src-tauri/src/ui/mod.rs:741` stores context-menu position and
  items in state on clip right-click.
- `apps/composer/src-tauri/src/ui/mod.rs:780` checks context-menu hits before
  normal click regions.
- `apps/composer/src-tauri/src/ui/mod.rs:787` dispatches only enabled menu
  items.
- `apps/composer/src-tauri/src/ui/mod.rs:799` clears the context menu on any
  primary click that does not hit an enabled item.
- `apps/composer/src-tauri/src/ui/mod.rs:1318` exposes menu items with enabled
  state, but not menu position metadata.

## Test Review

- `apps/composer/src-tauri/tests/plan_ui_e2e.rs:338` opens the clip context menu
  and asserts item selectors are present.
- `apps/composer/src-tauri/tests/plan_ui_e2e.rs:348` clicks Copy and asserts the
  menu closes after an enabled action.
- There is no outside-click, disabled-item, second-menu, or position test.

## Required Test Shape

- Right-click a clip at a known point and assert menu item bounds begin at the
  stored pointer location.
- Click outside the menu and assert all menu items are absent while
  `selected_clip_id` is unchanged.
- Open a menu when Paste is disabled, click the disabled Paste row, and assert no
  paste action or project mutation occurs.
- Open a second context menu on another clip and assert the previous menu
  position/items are replaced.
- Click a normal underlying control while a menu is open and assert the first
  click only dismisses the menu unless the product defines otherwise.

## Required Changes

- Add targeted context-menu dismiss tests using automation bounds and point
  clicks.
- Expose context-menu position and item enabled state in automation reports.
- Keep context-menu hit testing ahead of normal click regions and document the
  disabled-item dismiss policy.

## Verification

- `cargo test -p tench-composer automatic_context_menu_dismiss`
- `cargo test -p tench-composer composer_plan_inspector_render_queue_ai_and_context_menu_use_real_events_ui_e2e`
- `cargo test -p tench-ui-test ui_automation`
- `git diff --check`
