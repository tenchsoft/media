# AI Panel Prompt Field State

## Source Plan
- `plans/pixel-design/ai-panel-prompt-field-control-work-plan.md`

## Gap Analysis
Clicking the prompt field sets `ai_prompt_focused = true`, and typed characters/backspace are routed to `ai_prompt`. However, the prompt rendering does not show a focused state or status acknowledgment, and the automation node is exposed as a generic button without prompt value or focus state. See `apps/pixel-design/src-tauri/src/ui/mod.rs:490`, `apps/pixel-design/src-tauri/src/ui/panels.rs:54`, and `apps/pixel-design/src-tauri/src/ui/mod.rs:1186`.

Only Enter clears `ai_prompt_focused`. Escape calls `cancel_modal_action`, but that function does not clear `ai_prompt_focused`, and there is no other focus-clear path when switching away from the AI panel. This can leave hidden prompt input active after Escape or persona changes. See `apps/pixel-design/src-tauri/src/ui/mod.rs:903` and `apps/pixel-design/src-tauri/src/ui/state.rs:1147`.

The current E2E coverage types into `pd.ai.prompt` and asserts only that the stored prompt length increased, then presses Enter and asserts focus cleared. It does not verify the typed prompt becomes the queued job label, backspace isolation, Escape behavior, switch-away behavior, prompt value exposure, or document-pixel preservation. See `apps/pixel-design/src-tauri/tests/pixel_design_e2e.rs:563`.

## Plan Requirements Not Met
- Prompt focus must have visible or automation-exposed acknowledgment.
- Escape must clear AI prompt focus without leaving hidden text input active.
- Switching away from the AI panel must define and test prompt focus behavior.
- Automation metadata must expose prompt value and focus state.
- Tests must verify Run AI Job uses the typed prompt prefix as the job label.
- Tests must verify Backspace changes only prompt text and does not mutate document pixels.

## Required Test Shape
- Add a Pixel Design UI automation test that clicks `pd.ai.prompt`, asserts focus/value metadata, types prompt text, and verifies the prompt display updates.
- Click `pd.ai.run` and assert the created job label uses the typed prompt prefix.
- Press Backspace while focused and assert only `ai_prompt` changes while active layer pixels remain unchanged.
- Press Escape and assert `ai_prompt_focused` is false and later typed text does not append to the hidden prompt.
- Switch away from the AI persona and assert the defined focus behavior.

## Required Changes
- Clear `ai_prompt_focused` on Escape and on persona/panel switches where the prompt should stop receiving input.
- Add focused styling or status acknowledgment for the prompt field.
- Expose `pd.ai.prompt` with text-input-style role, current value, and focused state.
- Add prompt-field-specific E2E and state tests.

## Verification
- `cargo test -p tench-pixel-design --test pixel_design_e2e ai_panel_prompt_field`
- `cargo test -p tench-pixel-design`
