# Automatic AI Job List Render

## Source Plan
- `plans/pixel-design/automatic-ai-job-list-render-work-plan.md`

## Gap Analysis
`JobStatus` supports Queued, Running, Done, and Failed, but there is no Cancelled status even though the plan requires queued, running, completed, failed, and cancelled jobs to render distinctly. Current cancel behavior marks cancellable jobs as Failed. See `apps/pixel-design/src-tauri/src/ui/state.rs:242` and `apps/pixel-design/src-tauri/src/ui/state.rs:1199`.

`run_ai_job` unconditionally changes the previous job at index `1` to Done with 100 percent progress whenever a new job is inserted. That can overwrite a failed or cancelled row instead of preserving each job's own status/progress while the selected next-run tool changes. See `apps/pixel-design/src-tauri/src/ui/state.rs:1191`.

The painted job list reads `ai_jobs` directly, but automation only exposes an aggregate `pd.auto.ai_job_list` node with the job count. It does not expose per-row tool, label, status, or progress, so tests cannot verify the rendered rows automatically. See `apps/pixel-design/src-tauri/src/ui/panels.rs:144` and `apps/pixel-design/src-tauri/src/ui/mod.rs:1458`.

The current E2E coverage runs one job, checks that some job is Running, cancels, and only asserts no job remains Running. It does not run two jobs, verify row ordering, preserve row-specific tool/label/status/progress, verify a cancelled row, compare alternate paths, or verify persona switch/resize behavior. See `apps/pixel-design/src-tauri/tests/pixel_design_e2e.rs:573`.

## Plan Requirements Not Met
- Cancelled AI jobs must be represented and rendered distinctly from Failed jobs.
- Running a new job must not overwrite unrelated failed/cancelled row state.
- Automation metadata must expose each AI job row's tool, label, status, and progress.
- Tests must verify two-job ordering and row-specific tool/label/status/progress preservation.
- Tests must verify cancelled job rows render correctly.
- Tests must verify job-list rendering remains correct after tool switches, persona switches, and viewport resize.
- Tests must verify equivalent state changes from alternate paths produce identical job-list output.

## Required Test Shape
- Add a Pixel Design UI automation test that runs two jobs with different selected AI tools and asserts per-row tool, label, status, progress, and ordering.
- Cancel one running job and assert the row renders Cancelled or the product-defined cancelled state distinctly from Failed.
- Switch AI tools after jobs exist and assert existing row tools do not change.
- Switch personas and resize the viewport, then assert per-row automation metadata and visual capture remain correct.
- Use capture assertions to verify the job-list area repaints and remains valid/nonblank.

## Required Changes
- Add a product-defined Cancelled state or otherwise distinguish user cancellation from failure in model and rendering.
- Update run/cancel job transitions so previous rows keep their own tool, label, status, and progress.
- Expose per-job row automation nodes with tool, label, status, and progress values.
- Add AI job list render E2E and state tests.

## Verification
- `cargo test -p tench-pixel-design --test pixel_design_e2e automatic_ai_job_list_render`
- `cargo test -p tench-pixel-design`
