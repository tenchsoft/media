# AI Panel Cancel Button State

## Source Plan
- `plans/pixel-design/ai-panel-cancel-button-work-plan.md`

## Gap Analysis
`cancel_ai_job` marks the first queued or running job as `Failed` and sets a status message, but it does not set `ai_cancel_requested`. The only nearby path that sets that flag is `cancel_modal_action`, so AI Panel Cancel cannot currently request engine cancellation through the stored cancel flag. See `apps/pixel-design/src-tauri/src/ui/state.rs:1199` and `apps/pixel-design/src-tauri/src/ui/state.rs:1161`.

Clicking Cancel when no queued or running job exists is a silent no-op: completed jobs are not changed, but no disabled state or status acknowledgment tells the user that there was nothing to cancel. See `apps/pixel-design/src-tauri/src/ui/state.rs:1199`.

The current E2E coverage runs a job, clicks `pd.ai.cancel`, and only asserts that no job remains `Running`. It does not verify `ai_cancel_requested`, status text, the failed/cancelled row label, the no-running-job case, completed-job preservation, or starting a new job after cancellation. See `apps/pixel-design/src-tauri/tests/pixel_design_e2e.rs:573`.

The automation tree exposes only an aggregate `pd.auto.ai_job_list` node with a count value, not per-job row status nodes. That makes UI-level verification that the cancelled row no longer shows Running weaker than the plan requires. See `apps/pixel-design/src-tauri/src/ui/mod.rs:1458`.

## Plan Requirements Not Met
- AI Panel Cancel must set the engine cancellation flag when cancelling a queued or running job.
- Cancelling with no queued or running job must acknowledge the action without mutating completed jobs.
- Tests must verify status text and cancellation flag behavior.
- Tests must verify no completed job changes when Cancel is pressed without a cancellable job.
- Tests must verify a new run after cancellation starts cleanly and resets cancel state.
- Automation metadata must expose per-job row status so tests can verify the row no longer shows Running.

## Required Test Shape
- Add a Pixel Design UI automation test that runs a job, clicks `pd.ai.cancel`, and asserts job status, status text, `ai_cancel_requested`, and job-list repaint.
- Expose per-job automation nodes and assert the cancelled job row status is no longer Running.
- Seed or create a completed job, click `pd.ai.cancel` with no queued/running job, and assert the completed job is unchanged while the UI acknowledges no cancellable job.
- Click `pd.ai.run` after cancellation and assert the new job is Running with cancel state reset.

## Required Changes
- Set `ai_cancel_requested = true` in `cancel_ai_job` when a queued or running job is cancelled.
- Add a no-job status or disabled state for Cancel when nothing can be cancelled.
- Expose AI job rows and their current status through automation metadata.
- Add cancel-specific E2E and state tests.

## Verification
- `cargo test -p tench-pixel-design --test pixel_design_e2e ai_panel_cancel`
- `cargo test -p tench-pixel-design`
