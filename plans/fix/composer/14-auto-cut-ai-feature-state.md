# Auto Cut AI Feature State Fix Plan

## Source Plan

- `plans/composer/auto-cut-ai-feature-button-work-plan.md`

## Gap Analysis

The Auto Cut row is clickable, but it only sets a queued notice. There is no
workflow state, prerequisite validation, Engine IPC request, progress, failure,
or result-application path.

## Plan Requirements Not Met

- Auto Cut does not validate required timeline, media, or audio data before
  starting.
- Auto Cut does not send work through Tench Engine IPC.
- There is no per-workflow pending, success, failure, cancellation, or
  result-application state.
- The Auto Cut row does not render pending/progress state.
- Concurrent AI feature behavior is undefined.
- Failure or cancellation cannot clear pending state because pending state does
  not exist.
- Tests do not assert the Auto Cut notice, state, prerequisites, IPC dispatch, or
  data preservation.

## Code Review

- `apps/composer/src-tauri/src/ui/timeline_panel.rs:555` lists `Auto Cut` as a
  row and registers `ClickAction::RunAiFeature("Auto Cut")`.
- `apps/composer/src-tauri/src/ui/mod.rs:247` handles all AI features by setting
  a generic `{name} queued` notice.
- `apps/composer/src-tauri/src/ui/state.rs` has `show_ai_panel`, but no AI
  workflow state model.
- `apps/composer/src-tauri/src/ui/mod.rs:1445` maps the row to
  `composer.ai.feature.auto_cut`, but automation does not expose pending or
  disabled state.

## Test Review

- `apps/composer/src-tauri/tests/plan_ui_e2e.rs:335` clicks
  `composer.ai.feature.auto_cut` and asserts only that the capture changed.
- There is no test for missing media/audio prerequisites.
- There is no test for Engine IPC request shape, pending state, concurrent
  workflow policy, failure, cancellation, or partial-data rollback.

## Required Test Shape

- Add a UI E2E test that clicks Auto Cut with valid fixture timeline data and
  asserts an Auto Cut workflow enters pending/progress state.
- Add a backend/unit test for the Engine IPC request payload and workflow id.
- Add a missing-prerequisite test that asserts an actionable notice and no
  pending request.
- Click a second AI feature while Auto Cut is pending and assert the chosen
  queue/block/cancel rule.
- Simulate failure and cancellation and assert pending state clears and project
  data remains unchanged.
- Simulate a successful result and assert cuts are applied through a controlled
  timeline mutation path.

## Required Changes

- Add a Composer AI workflow state model keyed by workflow type.
- Implement an Auto Cut command path that validates prerequisites and dispatches
  to Tench Engine through IPC.
- Track pending, progress, success, failure, cancellation, and applied-result
  states separately from the generic notice string.
- Render and expose row pending/disabled/error state for
  `composer.ai.feature.auto_cut`.
- Define and enforce concurrent AI workflow policy.

## Verification

- `cargo test -p tench-composer auto_cut_ai_feature`
- `cargo test -p tench-composer composer_plan_inspector_render_queue_ai_and_context_menu_use_real_events_ui_e2e`
- `cargo test -p tench-ui-test ui_automation`
- `git diff --check`
