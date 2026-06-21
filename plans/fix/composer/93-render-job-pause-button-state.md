# Render Job Pause Button State

## Source Plan

- `plans/composer/render-job-pause-button-work-plan.md`

## Gap Analysis

Pause buttons register `PauseRenderJob(job.id)` per row, but automation exposes every pause button as the same `composer.render_job.pause` selector. Multi-job tests cannot target a specific job reliably. See `apps/composer/src-tauri/src/ui/timeline_panel.rs:496` and `apps/composer/src-tauri/src/ui/mod.rs:1455`.

The pause handler resolves a job by id and sets its status to `RenderStatus::Queued`, but there is no paused status and no backend pause coordination for active render work. See `apps/composer/src-tauri/src/ui/mod.rs:282` and `crates/composer-core/src/project.rs:202`.

The current E2E coverage clicks one pause button and only asserts that the capture changed. It does not assert job status, notice text, progress stopping, queued-job idempotence, multi-job isolation, or stale-id safety. See `apps/composer/src-tauri/tests/plan_ui_e2e.rs:318`.

## Plan Requirements Not Met

- Active rendering jobs must coordinate with the render backend so paused work actually stops advancing.
- Multi-job pause controls must be uniquely targetable in automation by job id or row index.
- Pausing a rendering job must be tested to stop progress advancement and expose a sensible status.
- Pausing a queued job must be tested for idempotent behavior.
- Pausing one job among many must be tested to leave other jobs unaffected.
- Stale pause ids must be tested to avoid panic and unrelated queue mutation.
- Tests must assert the `Render paused` notice.

## Required Test Shape

- Add a Composer UI automation test that puts a job into `Rendering`, clicks its pause button, and asserts status plus halted progress.
- Queue multiple jobs, pause one by a job-specific selector, and assert other jobs keep their status and progress.
- Pause an already queued job and assert no duplicate job or inconsistent status appears.
- Simulate a stale pause action after removing the job and assert no panic and no unrelated queue mutation.
- Assert the notice text is visible after a successful pause.

## Required Changes

- Add job-specific automation selectors such as `composer.render_job.<id>.pause`.
- Add a renderer pause hook or clearly defined no-op backend path for jobs that are not actively rendering.
- Route pause through shared render queue state methods instead of mutating queue rows inline.
- Expose job status, progress, and notice state through automation or test state helpers.

## Verification

- `cargo test -p tench-composer --test plan_ui_e2e render_job_pause`
- `cargo test -p tench-composer`
