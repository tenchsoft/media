# Find Similar Frames AI Feature Button State

## Source Plan
- `plans/player/ai-feature-find-similar-frames-button-work-plan.md`

## Gap Analysis
`ClickAction::SendAiPrompt` does not guard against `ai_request_pending == true`. If Find similar frames is clicked while another request is pending, the handler still appends another user prompt and fallback message instead of disabling, preventing duplicates, or explicitly queueing. See `apps/player/src-tauri/src/ui/app.rs:1062`.

Player does not currently route AI prompts through an Engine IPC client when one is available. The Send AI path always appends the local system fallback message, so the Engine IPC branch required by the plan is missing. See `apps/player/src-tauri/src/ui/app.rs:1070`.

The existing E2E clicks all AI feature buttons and only asserts that `ai_chat_log.len()` increased. It does not verify that `player.ai.feature.find_similar_frames` records the exact `Find similar frames` user prompt, that the system fallback is visible when Engine IPC is unavailable, that media-loaded state is preserved, or that pending-request duplicate behavior is handled. See `apps/player/src-tauri/tests/plan_ui_e2e.rs:526`.

## Plan Requirements Not Met
- Send AI must prevent duplicate Find similar frames requests or explicitly queue them while another request is pending.
- Send AI must have an Engine IPC dispatch path when Engine IPC is configured.
- Tests must verify the exact `Find similar frames` prompt is appended as a user chat message.
- Tests must verify the no-Engine fallback message is displayed after clicking the feature.
- Tests must verify the feature behavior with media loaded and while another request is pending.

## Required Test Shape
- Open the AI panel with a deterministic loaded-media state, click `player.ai.feature.find_similar_frames`, and assert the newest user chat message text is exactly `Find similar frames`.
- Assert the chat log or visible AI panel output includes the Player Engine IPC fallback when no Engine IPC client is configured.
- Set `ai_request_pending = true`, click `player.ai.feature.find_similar_frames`, and assert the UI either blocks the click without adding a duplicate message or records an explicit queued state.

## Required Changes
- Add pending-request handling to `ClickAction::SendAiPrompt` before appending a new prompt.
- Add an Engine IPC-backed AI dispatch path for Player and keep the current fallback only for unavailable IPC.
- Extend `plan_ui_e2e` AI feature coverage with exact prompt, fallback, media-loaded, and pending-request assertions for Find similar frames.

## Verification
- `cargo test -p tench-player --test plan_ui_e2e ai_feature_find_similar_frames`
- `cargo test -p tench-player`
