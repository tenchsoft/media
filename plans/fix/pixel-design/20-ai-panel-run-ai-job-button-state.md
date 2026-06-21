# AI Panel Run AI Job Button State

## Source Plan
- `plans/pixel-design/ai-panel-run-ai-job-button-work-plan.md`

## Gap Analysis
`run_ai_job` inserts a new job at index `0`, marks it `Running`, sets progress to `0`, clears `ai_cancel_requested`, and sets a running status. However, whenever another job exists, it unconditionally changes the previous job at index `1` to `Done` with `100` progress. That can overwrite a previously failed or cancelled job and leaves the previous-running-job rule implicit instead of coherent with cancel state. See `apps/pixel-design/src-tauri/src/ui/state.rs:1172`.

The current E2E coverage clicks `pd.ai.run`, asserts the aggregate job list exists, and checks that some job is `Running`. It does not verify top insertion, selected tool, exact progress, status text, empty-prompt fallback, long-prompt truncation without mutating stored prompt, previous job preservation, or cancel-state reset. See `apps/pixel-design/src-tauri/tests/pixel_design_e2e.rs:573`.

The automation tree exposes only an aggregate `pd.auto.ai_job_list` node with a count value, not per-job row label/status/progress nodes. That prevents UI-level assertions for ordering, displayed label truncation, and row progress. See `apps/pixel-design/src-tauri/src/ui/mod.rs:1458`.

## Plan Requirements Not Met
- Previous job resolution must be explicitly defined and must not corrupt failed or cancelled jobs.
- Tests must verify the new job is inserted at the top with the selected tool, Running status, and `0` progress.
- Tests must verify status text after Run AI Job.
- Tests must verify empty-prompt fallback label.
- Tests must verify long-prompt label truncation without mutating the stored prompt.
- Tests must verify job ordering and cancel-state reset when running while another job exists.
- Automation metadata must expose per-job row label, status, and progress.

## Required Test Shape
- Add a Pixel Design UI automation test that clicks `pd.ai.run` and asserts `ai_jobs[0]` tool, label, `Running` status, progress `0`, status text, and job-list repaint.
- Clear the prompt, run a job, and assert the label falls back to the selected AI tool request.
- Type a long prompt, run a job, and assert the displayed job label is truncated while `ai_prompt` remains unchanged.
- Run with an existing failed/cancelled job and assert the previous job is preserved according to product rules.
- Run after cancellation and assert `ai_cancel_requested` is reset.
- Assert per-job automation nodes expose the same label/status/progress shown in the painted job list.

## Required Changes
- Define previous-job resolution rules and update `run_ai_job` so failed or cancelled jobs are not overwritten as `Done`.
- Expose AI job rows and their label/status/progress through automation metadata.
- Add run-button-specific E2E and state tests.

## Verification
- `cargo test -p tench-pixel-design --test pixel_design_e2e ai_panel_run_ai_job`
- `cargo test -p tench-pixel-design`
