# Render Queue Close Button State

## Source Plan

- `plans/composer/render-queue-close-button-work-plan.md`

## Gap Analysis

The close button sets `show_render_queue` to false, but current E2E coverage only asserts the close selector disappears. It does not assert `show_render_queue`, job status preservation, reopening with the same jobs, or drawer-specific click region removal. See `apps/composer/src-tauri/src/ui/timeline_panel.rs:414`, `apps/composer/src-tauri/src/ui/mod.rs:267`, and `apps/composer/src-tauri/tests/plan_ui_e2e.rs:322`.

The tests do not cover closing while a job is rendering or clicking outside the drawer according to the product's dismissal contract.

## Plan Requirements Not Met

- Closing the drawer must be tested to set `show_render_queue == false`.
- Closing while jobs are queued or rendering must be tested to preserve job status and progress.
- Reopening the queue must be tested to show the same jobs and progress.
- Drawer-specific click regions must be tested to disappear after close.
- Outside-click behavior must be defined and tested.

## Required Test Shape

- Add a Composer UI automation test that opens the render queue, clicks `composer.render_queue.close`, and asserts `show_render_queue == false`.
- Record render job ids, status, and progress before close, close the drawer, reopen it, and assert the same data remains visible.
- Put a job into `Rendering`, close the drawer, and assert closing does not change render status.
- After close, click where a drawer job control used to be and assert no stale queue action fires.
- Click outside the drawer while open and assert the configured dismissal behavior.

## Required Changes

- Add the missing render queue close scenario tests.
- Expose render queue visibility, job status, progress, and stale-region assertions through automation if current state access is insufficient.
- Adjust outside-click handling only after the intended dismissal contract is defined.

## Verification

- `cargo test -p tench-composer --test plan_ui_e2e render_queue_close`
- `cargo test -p tench-composer`
