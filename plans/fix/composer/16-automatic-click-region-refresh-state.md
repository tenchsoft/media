# Automatic Click Region Refresh State Fix Plan

## Source Plan

- `plans/composer/automatic-click-region-refresh-behavior-work-plan.md`

## Gap Analysis

Composer clears and rebuilds click regions during paint, but the refresh
contract is not fully verified. Tests assert selector presence or absence in a
few flows, but do not prove stale coordinates stop dispatching or that resized
controls receive updated hit regions.

## Plan Requirements Not Met

- There is no test that clicks an old render-queue button coordinate after the
  queue closes and proves no stale action dispatches.
- Resize/panel splitter tests do not assert that button hit regions move with
  rendered controls.
- Tab-switch tests do not assert that controls from inactive tabs are no longer
  clickable.
- Overlay priority from rebuilt click-region order is not tested.
- Automation does not expose a click-region inventory separate from selectors,
  making stale-region debugging indirect.

## Code Review

- `apps/composer/src-tauri/src/ui/mod.rs:656` clears click regions at the start
  of paint.
- `apps/composer/src-tauri/src/ui/mod.rs:662` collects regions into a local
  buffer during visible surface painting.
- `apps/composer/src-tauri/src/ui/mod.rs:727` merges collected regions into
  `self.click_regions` after rendering.
- `apps/composer/src-tauri/src/ui/mod.rs:164` hit-tests the rebuilt list in
  reverse order, giving later-painted surfaces priority.
- `apps/composer/src-tauri/src/ui/mod.rs:805` dispatches pointer actions from
  the current click-region list.

## Test Review

- `apps/composer/src-tauri/tests/plan_ui_e2e.rs:322` closes the render queue and
  asserts `composer.render_queue.close` is absent, but does not click the old
  close button coordinate.
- `apps/composer/src-tauri/tests/plan_ui_e2e.rs:181` drags a splitter and checks
  capture change, but does not assert moved control bounds or hit behavior.
- There is no active-tab/inactive-tab stale click-region test.

## Required Test Shape

- Capture bounds for `composer.render_queue.close`, close the render queue, then
  click the old center point and assert no render-queue action runs.
- Open render queue again and assert the newly exposed close/pause/cancel regions
  are clickable.
- Drag a splitter, capture a known button's new bounds, and assert the old center
  no longer dispatches while the new center does.
- Switch left or inspector tabs and assert controls from the previous tab are
  absent and stale coordinates do not dispatch previous-tab actions.
- Add an overlay priority test where overlapping visible surfaces dispatch the
  topmost surface action.

## Required Changes

- Add click-region refresh tests using automation bounds and point clicks.
- Expose optional click-region diagnostics in automation reports, including
  action id, bounds, and paint order.
- Keep click-region collection tied to visible surface painting and ensure every
  state change that affects layout requests a repaint before user interaction.

## Verification

- `cargo test -p tench-composer automatic_click_region_refresh`
- `cargo test -p tench-composer composer_plan_inspector_render_queue_ai_and_context_menu_use_real_events_ui_e2e`
- `cargo test -p tench-ui-test ui_automation`
- `git diff --check`
