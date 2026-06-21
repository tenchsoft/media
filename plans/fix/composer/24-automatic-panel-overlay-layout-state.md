# Automatic Panel Overlay Layout State Fix Plan

## Source Plan

- `plans/composer/automatic-panel-overlay-layout-behavior-work-plan.md`

## Gap Analysis

AI panel and render queue overlay rectangles are recomputed during paint, but
layout behavior is incomplete for narrow windows, right-panel changes, and
multiple-overlay z-order. Tests only assert selectors are present.

## Plan Requirements Not Met

- Render queue geometry does not account for `right_panel_w`, so it can overlap
  the inspector unexpectedly.
- Overlay rectangles are not clamped for narrow windows or small vertical space.
- Tests do not resize the right inspector and assert AI panel movement.
- Tests do not resize the timeline and assert overlay bottoms stay above it.
- Tests do not cover narrow-window render queue placement.
- Tests do not open both render queue and AI panel to verify deterministic
  z-order and click routing.
- Automation does not expose overlay container bounds as named nodes.

## Code Review

- `apps/composer/src-tauri/src/ui/timeline_panel.rs:392` positions the render
  queue at `size.width - 340.0..size.width - 12.0`, independent of
  `right_panel_w`.
- `apps/composer/src-tauri/src/ui/timeline_panel.rs:533` positions the AI panel
  relative to `center_right = size.width - right_panel_w`.
- `apps/composer/src-tauri/src/ui/timeline_panel.rs:537` and `:396` use
  `tl_y - 12.0` for overlay bottoms, but there is no minimum-height clamp.
- `apps/composer/src-tauri/src/ui/mod.rs:700` paints render queue before AI
  panel and `:716` paints quick actions after both, defining an implicit z-order.
- `apps/composer/src-tauri/src/ui/mod.rs:1173` exposes click-region selectors,
  but not overlay container bounds.

## Test Review

- `apps/composer/src-tauri/tests/plan_ui_e2e.rs:313` checks render queue
  controls after queueing a render.
- `apps/composer/src-tauri/tests/plan_ui_e2e.rs:325` checks AI feature rows after
  opening the AI panel.
- There is no geometry, resize, narrow-window, or both-overlays test.

## Required Test Shape

- Resize the right inspector and assert AI panel bounds shift with
  `center_right`.
- Resize the timeline and assert AI/render queue overlay bottoms remain above
  the timeline splitter.
- Run a narrow viewport test and assert render queue bounds are fully visible or
  clamped according to policy.
- Open render queue and AI panel together and assert topmost overlay click
  routing is deterministic.
- Assert overlay container bounds through automation, not only child rows.

## Required Changes

- Add shared overlay geometry helpers for render queue and AI panel that account
  for toolbar height, timeline height, right panel width, and viewport size.
- Clamp overlay width/height and position for small windows.
- Expose `composer.render_queue.panel` and `composer.ai.panel` automation nodes
  with bounds.
- Add E2E coverage for resize, clamp, and z-order/click routing.

## Verification

- `cargo test -p tench-composer automatic_panel_overlay_layout`
- `cargo test -p tench-composer composer_plan_inspector_render_queue_ai_and_context_menu_use_real_events_ui_e2e`
- `cargo test -p tench-ui-test ui_automation`
- `git diff --check`
