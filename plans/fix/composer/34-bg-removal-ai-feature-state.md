# BG Removal AI Feature State Fix Plan

## Source Plan

- `plans/composer/bg-removal-ai-feature-button-work-plan.md`

## Gap Analysis

The BG Removal row is rendered, but activating it only goes through the generic
AI notice path. There is no BG Removal-specific action, prerequisite validation,
Engine IPC request, pending/progress state, cancellation, failure handling, or
result application.

## Plan Requirements Not Met

- BG Removal does not have a dedicated click action; it uses the generic
  `RunAiFeature(String)` action.
- Clicking BG Removal does not validate selected timeline video/media
  prerequisites.
- Clicking BG Removal does not send work through Tench Engine IPC.
- BG Removal has no per-workflow pending, success, failure, cancellation, or
  result-application state.
- The BG Removal row does not render pending or progress state.
- Concurrent AI workflow behavior for BG Removal is undefined.
- Failure or cancellation cannot clear BG Removal pending state because pending
  state does not exist.
- There is no test that clicks `composer.ai.feature.bg_removal`.
- There is no test for missing-media messaging, IPC payload, progress,
  cancellation, failure, or partial-result rollback for BG Removal.

## Code Review

- `apps/composer/src-tauri/src/ui/timeline_panel.rs:558` renders the BG Removal
  row.
- `apps/composer/src-tauri/src/ui/timeline_panel.rs:587` registers all AI rows
  as `ClickAction::RunAiFeature`.
- `apps/composer/src-tauri/src/ui/mod.rs:247` handles all AI feature clicks by
  setting a queued notice only.
- `apps/composer/src-tauri/src/ui/mod.rs:1445` derives
  `composer.ai.feature.bg_removal` from the generic string action.
- `apps/composer/src-tauri/src/ui/state.rs` has no AI workflow state model for
  BG Removal.

## Test Review

- `apps/composer/src-tauri/tests/plan_ui_e2e.rs:330` asserts
  `composer.ai.feature.bg_removal` is present.
- `apps/composer/src-tauri/tests/plan_ui_e2e.rs:335` clicks only Auto Cut and
  asserts capture change.
- Existing AI fixes for Auto Cut and Auto Subtitle track shared AI workflow
  state gaps, but they do not provide BG Removal-specific prerequisite,
  payload, or result-application coverage.

## Required Test Shape

- Click BG Removal with a selected video clip and assert a BG Removal workflow
  enters pending/progress state.
- Click BG Removal without required media and assert an actionable notice with
  no Engine IPC request.
- Assert the Engine IPC request identifies the BG Removal workflow, selected
  clip/media ids, and output application target.
- Click another AI row while BG Removal is pending and assert the defined
  queue/block/cancel policy.
- Simulate cancellation and failure, then assert pending clears and project data
  remains unchanged.
- Simulate success and assert results are applied only to the intended clip or
  media asset.

## Required Changes

- Add a typed BG Removal AI workflow variant instead of relying on the generic
  string action.
- Validate selected video/media prerequisites before dispatch.
- Route BG Removal through Tench Engine IPC and track lifecycle state keyed by
  workflow type.
- Render and expose row state for `composer.ai.feature.bg_removal`.
- Define and enforce concurrent AI workflow policy across all Composer AI rows.

## Verification

- `cargo test -p tench-composer bg_removal_ai_feature`
- `cargo test -p tench-composer composer_plan_inspector_render_queue_ai_and_context_menu_use_real_events_ui_e2e`
- `cargo test -p tench-ui-test ui_automation`
- `git diff --check`
