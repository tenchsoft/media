# Quick AI Panel Button State

## Source Plan

- `plans/composer/quick-ai-panel-button-work-plan.md`

## Gap Analysis

The quick AI button toggles `show_ai_panel`, but the current E2E coverage only clicks `composer.quick.ai` once and asserts feature selectors are present. It does not click the button again to assert the panel closes, assert that only `show_ai_panel` changed, or verify that no AI task is queued by the toggle itself. See `apps/composer/src-tauri/src/ui/timeline_panel.rs:624`, `apps/composer/src-tauri/src/ui/mod.rs:264`, and `apps/composer/src-tauri/tests/plan_ui_e2e.rs:325`.

The test closes the render queue before opening AI, so the required overlay layering behavior with the render queue open is untested. See `apps/composer/src-tauri/tests/plan_ui_e2e.rs:322`.

The playback-continuity scenario is untested.

## Plan Requirements Not Met

- Clicking AI while open must be tested to close the panel.
- The quick AI toggle must be tested to mutate only `show_ai_panel` and not enqueue an AI feature.
- AI panel layering with the render queue open must be tested.
- Clicking AI during playback must be tested to preserve playback state.

## Required Test Shape

- Add a Composer UI automation test that clicks `composer.quick.ai`, asserts `show_ai_panel == true`, then clicks it again and asserts the AI panel selectors are absent.
- Assert no AI feature notice, job, or task state is created by the toggle alone.
- Open the render queue, click `composer.quick.ai`, and assert both overlays follow the intended z-order and remain actionable.
- Start playback, click `composer.quick.ai`, and assert playback state is unchanged.

## Required Changes

- Add the missing quick AI panel scenario tests.
- Expose `show_ai_panel`, overlay order, and AI task/notice state through automation if existing state access is insufficient.
- Adjust overlay ordering only if the render-queue layering test exposes an issue.

## Verification

- `cargo test -p tench-composer --test plan_ui_e2e quick_ai_panel`
- `cargo test -p tench-composer`
