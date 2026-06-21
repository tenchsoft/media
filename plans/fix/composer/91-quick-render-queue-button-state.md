# Quick Render Queue Button State

## Source Plan

- `plans/composer/quick-render-queue-button-work-plan.md`

## Gap Analysis

The quick Queue button toggles `show_render_queue`, but current E2E coverage does not click `composer.quick.queue`. The render queue is opened indirectly by export and closed through the drawer close button. See `apps/composer/src-tauri/src/ui/timeline_panel.rs:606`, `apps/composer/src-tauri/src/ui/mod.rs:261`, and `apps/composer/src-tauri/tests/plan_ui_e2e.rs:253`.

The no-jobs drawer scenario is untested, including the header and close button visibility when `project.render_queue` is empty. See `apps/composer/src-tauri/src/ui/timeline_panel.rs:385` and `apps/composer/src-tauri/src/ui/timeline_panel.rs:401`.

The tests do not verify that toggling the quick Queue button leaves `project.render_queue` and transport state unchanged.

## Plan Requirements Not Met

- Clicking Queue while closed must be tested to open the drawer through `composer.quick.queue`.
- Clicking Queue while open must be tested to close the drawer through the same quick action.
- Opening the drawer with no render jobs must be tested for header and close button visibility.
- The quick Queue toggle must be tested to avoid mutating `project.render_queue`.
- Clicking Queue during playback must be tested to preserve transport state.

## Required Test Shape

- Add a Composer UI automation test that clicks `composer.quick.queue` with an empty queue and asserts the drawer header and close button are visible.
- Click `composer.quick.queue` again and assert render queue selectors are absent.
- Record `project.render_queue` before and after toggles and assert it is unchanged.
- Start playback, click `composer.quick.queue`, and assert transport state is unchanged.
- Assert click regions refresh by opening and closing the drawer and then interacting with another visible control.

## Required Changes

- Add the missing quick render queue scenario tests.
- Expose queue drawer header and render queue length through automation if current selectors are insufficient.
- Adjust click-region refresh only if the new open/close interaction test exposes stale regions.

## Verification

- `cargo test -p tench-composer --test plan_ui_e2e quick_render_queue`
- `cargo test -p tench-composer`
