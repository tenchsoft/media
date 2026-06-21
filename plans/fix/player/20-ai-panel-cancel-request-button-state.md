# AI Panel Cancel Request Button State

## Source Plan
- `plans/player/ai-panel-cancel-request-button-work-plan.md`

## Gap Analysis
`ClickAction::CancelAiRequest` only sets `ai_request_pending = false` and shows a toast. There is no Engine cancellation path, request id, cancellation token, or late-response guard, so a future Engine-backed request could still append an assistant message after Cancel. See `apps/player/src-tauri/src/ui/app.rs:1078`.

The existing E2E asserts that `player.ai.cancel` is present and clicks it, but it does not verify that pending state becomes false, the Cancel button disappears, the cancellation toast is visible, or no chat messages are appended by the cancel action. See `apps/player/src-tauri/tests/plan_ui_e2e.rs:506`.

The completed-request and clean-restart scenarios are not covered. The test sends a prompt after clicking Cancel, but it does not assert that Cancel is absent after completion or that the next request starts from clean pending/chat state. See `apps/player/src-tauri/tests/plan_ui_e2e.rs:518`.

## Plan Requirements Not Met
- Cancel AI must coordinate with Engine cancellation when Engine IPC is available.
- Cancel AI must prevent late assistant messages from the cancelled request.
- Tests must verify Cancel sets `ai_request_pending` false and removes `player.ai.cancel`.
- Tests must verify the cancellation toast appears.
- Tests must verify Cancel after completion does not mutate unrelated chat state.
- Tests must verify a new request after cancellation starts cleanly.

## Required Test Shape
- Start with `ai_request_pending = true`, click `player.ai.cancel`, and assert pending false, cancel selector absent, toast text `AI request cancelled`, and unchanged chat log length.
- Simulate or inject a cancelled request completion and assert no assistant message is appended after cancellation.
- Complete a request or use the no-Engine fallback path, assert `player.ai.cancel` is absent, then verify an attempted cancel/no-op does not change chat state.
- Send a new prompt after cancellation and assert the new user message and fallback/result are attached to the new request only.

## Required Changes
- Add Engine IPC cancellation state to the Player AI request model.
- Track request identity so cancelled request completions cannot append late assistant messages.
- Extend `plan_ui_e2e` AI cancel coverage for pending false, selector absence, toast, no late message, completed no-op, and clean restart.

## Verification
- `cargo test -p tench-player --test plan_ui_e2e ai_panel_cancel_request`
- `cargo test -p tench-player`
