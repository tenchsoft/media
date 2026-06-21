# Render Job Cancel Button State

## Source Plan

- `plans/composer/render-job-cancel-button-work-plan.md`

## Gap Analysis

Cancel buttons register `CancelRenderJob(job.id)` per row, but automation exposes every cancel button as the same `composer.render_job.cancel` selector. Multi-job tests cannot target a specific job reliably. See `apps/composer/src-tauri/src/ui/timeline_panel.rs:482` and `apps/composer/src-tauri/src/ui/mod.rs:1455`.

The cancel handler resolves a job by id and marks it `RenderStatus::Failed`, but there is no backend render cancellation path for an active renderer. See `apps/composer/src-tauri/src/ui/mod.rs:270` and `crates/composer-core/src/project.rs:202`.

The current E2E coverage clicks one cancel button and only asserts that the capture changed. It does not assert the job status, notice text, active-render progress stopping, multi-job isolation, or stale-button no-panic behavior. See `apps/composer/src-tauri/tests/plan_ui_e2e.rs:320`.

## Plan Requirements Not Met

- Active rendering jobs must call a backend render cancellation path when one exists.
- Multi-job cancel controls must be uniquely targetable in automation by job id or row index.
- Canceling one job among many must be tested to leave other jobs unchanged.
- Canceling a rendering job must be tested to stop progress advancement.
- Clicking a stale cancel button after the job is removed must be tested to avoid panic.
- Tests must assert the resulting job status and `Render cancelled` notice.

## Required Test Shape

- Add a Composer UI automation test that queues a job, clicks its cancel button, and asserts the job status is `Failed` or `Cancelled` plus the notice is visible.
- Queue multiple jobs, cancel one by a job-specific selector, and assert other jobs keep their status and progress.
- Put a job into `Rendering`, cancel it, and assert progress no longer advances.
- Simulate a stale cancel action after removing the job and assert no panic and no unrelated queue mutation.

## Required Changes

- Add job-specific automation selectors such as `composer.render_job.<id>.cancel`.
- Add a renderer cancellation hook or clearly defined no-op backend path for jobs that are not actively rendering.
- Route cancel through shared render queue state methods instead of mutating queue rows inline.
- Expose job status, progress, and notice state through automation or test state helpers.

## Verification

- `cargo test -p tench-composer --test plan_ui_e2e render_job_cancel`
- `cargo test -p tench-composer`
