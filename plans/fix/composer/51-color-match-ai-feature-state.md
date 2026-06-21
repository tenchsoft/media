# Color Match AI Feature State Fix Plan

## Source Plan

- `plans/composer/color-match-ai-feature-button-work-plan.md`

## Gap Analysis

The Color Match row is rendered, but activating it only sets a generic queued
notice. There is no Color Match-specific action, source/target prerequisite
validation, Engine IPC request, pending/progress state, cancellation, failure
handling, or result application.

## Plan Requirements Not Met

- Color Match does not have a dedicated click action; it uses the generic
  `RunAiFeature(String)` action.
- Clicking Color Match does not validate required source and target timeline
  media prerequisites.
- Clicking Color Match does not open a configuration flow when a source/target
  choice is required.
- Clicking Color Match does not send work through Tench Engine IPC.
- Color Match has no per-workflow pending, success, failure, cancellation, or
  result-application state.
- The Color Match row does not render pending or progress state.
- Concurrent AI workflow behavior for Color Match is undefined.
- Failure or cancellation cannot clear Color Match pending state because pending
  state does not exist.
- There is no test that clicks `composer.ai.feature.color_match`.
- There is no test for missing-media messaging, IPC payload, progress,
  cancellation, failure, or partial-result rollback for Color Match.

## Code Review

- `apps/composer/src-tauri/src/ui/timeline_panel.rs:559` renders the Color Match
  row.
- `apps/composer/src-tauri/src/ui/timeline_panel.rs:587` registers all AI rows
  as `ClickAction::RunAiFeature`.
- `apps/composer/src-tauri/src/ui/mod.rs:247` handles all AI feature clicks by
  setting a queued notice only.
- `apps/composer/src-tauri/src/ui/mod.rs:1445` derives
  `composer.ai.feature.color_match` from the generic string action.
- `apps/composer/src-tauri/src/ui/state.rs` has no AI workflow state model for
  Color Match.

## Test Review

- `apps/composer/src-tauri/tests/plan_ui_e2e.rs:331` asserts
  `composer.ai.feature.color_match` is present.
- `apps/composer/src-tauri/tests/plan_ui_e2e.rs:335` clicks only Auto Cut and
  asserts capture change.
- Existing AI fixes for other rows track shared AI workflow state gaps, but
  they do not provide Color Match-specific prerequisite, payload, or result
  application coverage.

## Required Test Shape

- Click Color Match with valid source and target clip data and assert a Color
  Match workflow enters pending/progress state.
- Click Color Match without required media or target/reference data and assert
  an actionable notice with no Engine IPC request.
- Assert the Engine IPC request identifies the Color Match workflow, source
  clip/media id, target clip/media id, and result application target.
- Click another AI row while Color Match is pending and assert the defined
  queue/block/cancel policy.
- Simulate cancellation and failure, then assert pending clears and project data
  remains unchanged.
- Simulate success and assert color-match results apply only to the intended
  target clip.

## Required Changes

- Add a typed Color Match AI workflow variant instead of relying on the generic
  string action.
- Validate source/target prerequisites or open a configuration flow before
  dispatch.
- Route Color Match through Tench Engine IPC and track lifecycle state keyed by
  workflow type.
- Render and expose row state for `composer.ai.feature.color_match`.
- Define and enforce concurrent AI workflow policy across all Composer AI rows.

## Verification

- `cargo test -p tench-composer color_match_ai_feature`
- `cargo test -p tench-composer composer_plan_inspector_render_queue_ai_and_context_menu_use_real_events_ui_e2e`
- `cargo test -p tench-ui-test ui_automation`
- `git diff --check`
