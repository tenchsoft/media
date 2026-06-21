# Deliver Inspector Export Button State Fix Plan

## Source Plan

- `plans/composer/deliver-inspector-export-button-work-plan.md`

## Gap Analysis

Deliver Export enqueues a render job and opens the queue, but export setting
validation is missing and tests do not verify that the queued job copied the
currently displayed settings.

## Plan Requirements Not Met

- Export settings are not validated before enqueueing.
- There is no test that changing format updates the queued job extension.
- There is no test that changing resolution stores that resolution in the job.
- There is no test that clicking Export while another render is queued appends a
  new job instead of replacing the existing one.
- There is no test that queued jobs receive unique ids through the Deliver
  button path.
- There is no test that invalid export settings block enqueue and show an
  actionable error.
- There is no automation value for displayed export settings or the latest
  render job settings.

## Code Review

- `apps/composer/src-tauri/src/ui/right_panel.rs:503` registers the Deliver
  Export button as `ClickAction::Export`.
- `apps/composer/src-tauri/src/ui/mod.rs:258` dispatches Export to
  `state.enqueue_render`.
- `apps/composer/src-tauri/src/ui/state.rs:833` builds the render job name from
  project name and export format extension.
- `apps/composer/src-tauri/src/ui/state.rs:839` copies current export settings
  into the render job.
- `apps/composer/src-tauri/src/ui/state.rs:841` opens the render queue.
- `crates/composer-core/src/project.rs:273` assigns a new render job id and
  pushes the job.

## Test Review

- `apps/composer/src-tauri/tests/plan_ui_e2e.rs:309` changes export format.
- `apps/composer/src-tauri/tests/plan_ui_e2e.rs:311` clicks
  `composer.deliver.export`.
- `apps/composer/src-tauri/tests/plan_ui_e2e.rs:313` only asserts the render
  queue is non-empty.
- Existing E2E does not assert job name, job settings, job id uniqueness,
  append behavior, notice text, or invalid-setting behavior.

## Required Test Shape

- Change export format, click Deliver Export, and assert the latest render job
  name extension and settings format match the displayed field.
- Change resolution, click Export, and assert the latest job stores that width
  and height.
- Queue once, click Export again, and assert render queue length increases with
  unique ids.
- Inject invalid settings such as zero width, zero height, non-finite fps, or
  zero bitrate and assert no job is queued plus an actionable error notice.
- Assert `show_render_queue` is true and notice text is `Render queued` on a
  valid export.

## Required Changes

- Add export settings validation before enqueueing.
- Return a render enqueue result that exposes success and validation errors.
- Expose displayed export setting values and latest render job values through
  automation or focused test helpers.

## Verification

- `cargo test -p tench-composer deliver_inspector_export_button`
- `cargo test -p tench-composer composer_plan_inspector_render_queue_ai_and_context_menu_use_real_events_ui_e2e`
- `cargo test -p tench-ui-test ui_automation`
- `git diff --check`
