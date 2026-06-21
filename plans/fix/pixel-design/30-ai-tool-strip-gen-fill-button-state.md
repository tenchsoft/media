# Gen Fill AI Tool Strip Button State

## Source Plan
- `plans/pixel-design/ai-tool-strip-gen-fill-button-work-plan.md`

## Gap Analysis
The AI tool strip handler sets `expanded_ai` and updates status, and the strip renderer highlights from that shared state. However, the automation node for `pd.ai_tool.gen_fill` is a generic button and does not expose selected/active state, so the strip highlight and "only latest selected" behavior are not directly verifiable through automation. See `apps/pixel-design/src-tauri/src/ui/mod.rs:263`, `apps/pixel-design/src-tauri/src/ui/toolbar.rs:169`, and `apps/pixel-design/src-tauri/src/ui/mod.rs:1012`.

The current E2E coverage clicks every AI strip tool and only asserts that `expanded_ai` changed. It does not verify Gen Fill specifically, status text, strip highlight, AI panel reflected selection, Run AI Job creating a Gen Fill job, prompt handling, document-pixel preservation, or running-job preservation while changing the next-run tool. See `apps/pixel-design/src-tauri/tests/pixel_design_e2e.rs:554`.

## Plan Requirements Not Met
- Automation metadata must expose the Gen Fill strip button selected/active state.
- Tests must verify selecting Gen Fill updates status text.
- Tests must verify the AI panel reflects Gen Fill after strip selection.
- Tests must verify the next Run AI Job creates a job with `AiTool::GenFill`.
- Tests must verify switching between strip buttons leaves only Gen Fill selected when it is the latest selection.
- Tests must verify selecting Gen Fill while a job is running preserves that running job.
- Tests must verify selecting Gen Fill does not mutate document pixels.

## Required Test Shape
- Add a Pixel Design UI automation test that clicks `pd.ai_tool.gen_fill` and asserts `expanded_ai == AiTool::GenFill`, status text, and selected/active metadata for the strip button.
- Assert the matching `pd.ai.panel.gen_fill` row is selected after strip selection.
- Type a prompt, click `pd.ai.run`, and assert the created job uses `AiTool::GenFill`.
- Start or seed a running job, select another AI strip tool, then select Gen Fill and assert the existing job remains running while `expanded_ai` changes.
- Assert active layer pixels or document dirty state are unchanged by selecting Gen Fill.

## Required Changes
- Expose selected/active state for AI tool strip and AI panel automation nodes.
- Add Gen Fill-specific E2E tests for strip selection, status, shared panel state, run-job tool selection, and running-job preservation.

## Verification
- `cargo test -p tench-pixel-design --test pixel_design_e2e ai_tool_strip_gen_fill`
- `cargo test -p tench-pixel-design`
