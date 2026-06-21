# Voice To Text AI Feature State

## Source Plan

- `plans/composer/voice-to-text-ai-feature-button-work-plan.md`

## Gap Analysis

The Voice-to-Text row is rendered in the AI panel, but it uses the generic `RunAiFeature(String)` action rather than a dedicated workflow action. See `apps/composer/src-tauri/src/ui/timeline_panel.rs:553` and `apps/composer/src-tauri/src/ui/mod.rs:1445`.

Clicking any AI feature only sets a queued notice. There is no prerequisite validation, Engine IPC request, pending state, success/failure state, cancellation handling, or result application state for Voice To Text. See `apps/composer/src-tauri/src/ui/mod.rs:247`.

The current E2E coverage asserts that `composer.ai.feature.voice_to_text` exists, but it only clicks Auto Cut and does not exercise Voice To Text. See `apps/composer/src-tauri/tests/plan_ui_e2e.rs:325`.

## Plan Requirements Not Met

- Voice To Text must have a dedicated workflow action or typed workflow id instead of a generic string-only path.
- Required media/audio prerequisites must be validated before starting the workflow.
- Voice To Text work must be sent through Tench Engine IPC.
- Pending, success, failure, cancellation, and result-application state must be tracked separately for Voice To Text.
- Clicking another AI feature while Voice To Text is pending must follow a visible queue/block/cancel rule.
- Failure or cancellation must clear pending state without corrupting project data.

## Required Test Shape

- Add a Composer UI automation test that clicks `composer.ai.feature.voice_to_text` with valid audio media and asserts a pending state plus Engine IPC request.
- Click Voice To Text without required audio and assert an actionable missing-prerequisite notice with no Engine request.
- Start Voice To Text, click another AI feature, and assert the configured queue/block/cancel behavior.
- Simulate failure or cancellation and assert pending state clears and project data remains unchanged.
- Simulate success and assert the transcript/subtitle result is applied through the intended state path.

## Required Changes

- Replace or wrap generic `RunAiFeature(String)` with typed AI workflow actions.
- Add Voice To Text prerequisite validation.
- Add Engine IPC integration for the Voice To Text workflow.
- Add per-workflow state for pending, progress, success, failure, cancellation, and result application.
- Expose Voice To Text workflow state through automation.

## Verification

- `cargo test -p tench-composer --test plan_ui_e2e voice_to_text_ai`
- `cargo test -p tench-composer`
