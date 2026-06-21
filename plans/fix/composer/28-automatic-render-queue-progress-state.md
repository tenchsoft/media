# Automatic Render Queue Progress State Fix Plan

## Source Plan

- `plans/composer/automatic-render-queue-progress-behavior-work-plan.md`

## Gap Analysis

Render queue rows are visible after export, but progress and status behavior is
not complete or verified. Progress values are drawn directly without clamping,
there is no UI update path for backend render progress, and cancellation is
shown as a failed job rather than a distinct understandable cancelled state.

## Plan Requirements Not Met

- Progress bar fill width and percentage text are not clamped to `0..100`.
- There is no render backend progress update path that mutates queued jobs and
  requests a repaint.
- Completed jobs are not exercised by UI state or E2E tests.
- Cancelled jobs are displayed with the `Failed` status label, which makes a
  user-initiated cancellation indistinguishable from a render failure.
- There is no test that a queued render row displays initial `0% Queued`
  status.
- There is no test that progress changes update both the fill width and
  percentage text.
- There is no test that completion changes the row to `Done` and `100%`.
- There is no test that failure or cancellation leaves an understandable row
  status.
- Automation does not expose render job row status text, progress text, or
  progress bar bounds.

## Code Review

- `crates/composer-core/src/project.rs:198` stores render progress as `u8`,
  which can still exceed `100`.
- `crates/composer-core/src/project.rs:204` has no cancelled status variant.
- `crates/composer-core/src/project.rs:273` enqueues jobs with progress `0` and
  status `Queued`.
- `apps/composer/src-tauri/src/ui/timeline_panel.rs:450` maps status variants to
  labels.
- `apps/composer/src-tauri/src/ui/timeline_panel.rs:458` renders raw
  `job.progress` text.
- `apps/composer/src-tauri/src/ui/timeline_panel.rs:473` computes fill width
  from raw `job.progress as f64 / 100.0` without clamping.
- `apps/composer/src-tauri/src/ui/mod.rs:270` handles cancellation by changing
  the job status to `Failed`.

## Test Review

- `crates/composer-core/src/project.rs:337` checks enqueue status only at the
  project model level.
- `apps/composer/src-tauri/tests/plan_ui_e2e.rs:312` asserts a render job exists
  and queue controls are present after export.
- `apps/composer/src-tauri/tests/plan_ui_e2e.rs:318` clicks pause and cancel but
  only checks capture changes, not job status text or progress rendering.

## Required Test Shape

- Add a render queue paint or E2E test that queues a job and asserts row name,
  `0%`, and `Queued` status through automation values.
- Mutate a job to `Rendering` with progress values such as `37` and `150`, then
  assert percentage text and progress bar fill are clamped correctly.
- Mutate a job to `Completed` and assert the row shows `Done` with `100%`.
- Trigger cancel and failure paths separately and assert each row label remains
  understandable.
- Add automation nodes per render job row with stable id, status value,
  progress value, progress text, and progress bar bounds.

## Required Changes

- Clamp render progress through a shared helper before text and geometry are
  computed.
- Add a distinct cancellation representation or a UI label that differentiates
  user cancellation from render failure.
- Add a backend-progress-to-UI update method that updates the matching job,
  preserves row identity, and requests repaint after progress/status changes.

## Verification

- `cargo test -p tench-composer automatic_render_queue_progress`
- `cargo test -p tench-composer composer_plan_inspector_render_queue_ai_and_context_menu_use_real_events_ui_e2e`
- `cargo test -p tench-ui-test ui_automation`
- `git diff --check`
