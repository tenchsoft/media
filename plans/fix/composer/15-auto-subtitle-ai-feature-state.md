# Auto Subtitle AI Feature State Fix Plan

## Source Plan

- `plans/composer/auto-subtitle-ai-feature-button-work-plan.md`

## Gap Analysis

The Auto Subtitle row is visible and clickable, but it only sets a generic
queued notice. It does not validate audio prerequisites, dispatch Engine IPC,
track pending/progress state, or apply subtitle results safely.

## Plan Requirements Not Met

- Auto Subtitle does not validate required media/audio data before starting.
- Auto Subtitle does not send work through Tench Engine IPC.
- There is no workflow-specific pending, success, failure, cancellation, or
  result-application state.
- The Auto Subtitle row does not render pending/progress/disabled/error state.
- Concurrent AI workflow behavior is undefined.
- Successful subtitle result application to subtitle tracks or subtitle editor
  state is not modeled.
- Tests do not click Auto Subtitle or verify any workflow state.

## Code Review

- `apps/composer/src-tauri/src/ui/timeline_panel.rs:556` lists `Auto Subtitle`
  and registers it with `ClickAction::RunAiFeature`.
- `apps/composer/src-tauri/src/ui/mod.rs:247` handles all AI features by setting
  a generic `{name} queued` notice.
- `apps/composer/src-tauri/src/ui/state.rs` has subtitle text and subtitle track
  support, but no AI subtitle workflow state.
- `apps/composer/src-tauri/src/ui/mod.rs:1445` maps the row to
  `composer.ai.feature.auto_subtitle`, but automation does not expose workflow
  state.

## Test Review

- `apps/composer/src-tauri/tests/plan_ui_e2e.rs:328` only checks
  `composer.ai.feature.auto_subtitle` is present.
- `apps/composer/src-tauri/tests/plan_ui_e2e.rs:335` clicks Auto Cut, not Auto
  Subtitle.
- There is no test for missing audio, Engine IPC payload, progress, concurrent
  workflow policy, failure/cancel cleanup, or subtitle result application.

## Required Test Shape

- Add a UI E2E test that clicks Auto Subtitle with fixture audio/timeline data
  and asserts a workflow-specific pending/progress state.
- Add a missing-audio test that shows an actionable notice and does not start an
  Engine request.
- Add a backend/unit test for the Engine IPC request payload and workflow id.
- Simulate success and assert subtitle results are applied through a controlled
  subtitle track or subtitle-editor path.
- Simulate failure and cancellation and assert pending state clears without
  partial project mutation.
- Click another AI feature while Auto Subtitle is pending and assert the chosen
  queue/block/cancel policy.

## Required Changes

- Add an Auto Subtitle workflow state and Engine IPC dispatch path.
- Validate audio/timeline prerequisites before creating the request.
- Track pending, progress, success, failure, cancellation, and applied-result
  states separately from generic notices.
- Render and expose row state for `composer.ai.feature.auto_subtitle`.
- Implement a safe subtitle result application path with rollback or all-or-none
  semantics.

## Verification

- `cargo test -p tench-composer auto_subtitle_ai_feature`
- `cargo test -p tench-composer composer_plan_inspector_render_queue_ai_and_context_menu_use_real_events_ui_e2e`
- `cargo test -p tench-ui-test ui_automation`
- `git diff --check`
