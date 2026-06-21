# Export Toolbar Button State

## Source Plan

- `plans/composer/export-toolbar-button-work-plan.md`

## Gap Analysis

Repeated export clicks create separate render jobs with distinct ids, but every job name is built from only the project name and format extension. That leaves repeated exports with identical names. See `apps/composer/src-tauri/src/ui/state.rs:833` and `crates/composer-core/src/project.rs:273`.

The current E2E coverage queues through the Deliver inspector button, not the top toolbar export button. The broad selector inventory includes quick queue controls, and the export flow clicks `composer.deliver.export`, so the Edit-mode toolbar scenario is unverified. See `apps/composer/src-tauri/tests/plan_ui_e2e.rs:96` and `apps/composer/src-tauri/tests/plan_ui_e2e.rs:298`.

The tests do not verify that queued jobs use export settings after resolution or FPS changes, and they do not cover repeated export clicks or empty-timeline export behavior.

## Plan Requirements Not Met

- Repeated export clicks must create separately identifiable jobs, including a unique job name or display label.
- Clicking `composer.toolbar.export` in Edit mode must be covered by UI automation.
- Export after changing Deliver resolution and FPS must assert the queued job uses those updated settings.
- Empty-timeline export behavior must be explicitly tested as either a valid empty export or an actionable error.

## Required Test Shape

- Add a Composer UI automation test that clicks `composer.toolbar.export` in Edit mode and asserts the render queue opens with a queued job.
- Click the toolbar export button repeatedly and assert each queued job has a unique id and unique name or user-visible label.
- Change resolution and FPS in Deliver mode, click Export, and assert the latest render job settings match the displayed values.
- Run the export action on an empty timeline and assert either a queued valid empty-export job or an actionable notice with no queued job.

## Required Changes

- Add a deterministic unique suffix or label strategy for repeated render job names if duplicate names are not allowed.
- Expose enough automation state to verify queued job names and export settings.
- Define and enforce the empty-timeline export contract in the enqueue path.

## Verification

- `cargo test -p tench-composer --test plan_ui_e2e export_toolbar`
- `cargo test -p tench-composer`
