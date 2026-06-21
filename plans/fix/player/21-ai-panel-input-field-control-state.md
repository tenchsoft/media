# AI Panel Input Field Control State

## Source Plan
- `plans/player/ai-panel-input-field-control-work-plan.md`

## Gap Analysis
The AI input automation node is emitted through the generic click-region path, which always sets `value: None` and `focused: false`. The field can gain focus in `PlayerState`, but automation cannot observe the active focus state or draft text required by the plan. See `apps/player/src-tauri/src/ui/app.rs:2217` and `apps/player/src-tauri/src/ui/app.rs:2576`.

The existing E2E types into `player.ai.input` and presses Enter, but it does not assert that `ai_focused` became true, that other text inputs lost focus, or that `ai_input_text` contains the draft before submission. See `apps/player/src-tauri/tests/plan_ui_e2e.rs:518`.

The outside-click and exact Enter-submit scenarios are not verified. The app clears AI focus when a click misses registered regions and sends the draft through `SendAiPrompt` on Enter, but tests do not assert focus clearing, draft clearing, or the exact user chat message text. See `apps/player/src-tauri/src/ui/app.rs:1791` and `apps/player/src-tauri/src/ui/app.rs:2016`.

## Plan Requirements Not Met
- `player.ai.input` automation must expose focused state and current draft value.
- Tests must verify clicking the AI input focuses only `ai_focused`.
- Tests must verify typed text updates `ai_input_text` and appears in automation or state.
- Tests must verify clicking outside the field clears AI focus.
- Tests must verify Enter sends the exact draft as a user AI prompt and clears the draft.

## Required Test Shape
- Open the AI panel, click `player.ai.input`, and assert `ai_focused == true` while URL, subtitle-search, and chapter-name focus flags are false.
- Type `Explain this frame`, assert `PlayerState.ai_input_text` and the textbox automation `value` match.
- Click outside the input and assert `ai_focused == false`.
- Focus the input again, type text, press Enter, and assert the newest user chat message text matches the draft and `ai_input_text` is empty.

## Required Changes
- Special-case text input automation nodes so `player.ai.input` reports `value` and `focused` from `PlayerState`.
- Extend `plan_ui_e2e` AI input coverage for focus, typed draft value, outside-click blur, exact Enter submit, and draft clearing.

## Verification
- `cargo test -p tench-player --test plan_ui_e2e ai_panel_input`
- `cargo test -p tench-player`
