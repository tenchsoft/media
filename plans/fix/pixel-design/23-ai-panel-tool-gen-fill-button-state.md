# Gen Fill AI Panel Tool Button State

## Source Plan
- `plans/pixel-design/ai-panel-tool-gen-fill-button-work-plan.md`

## Gap Analysis
The panel click handler sets `expanded_ai = AiTool::GenFill`, and the panel renderer highlights rows from that shared state. However, the automation node for `pd.ai.panel.gen_fill` is a generic button and does not expose selected/active state, so the row highlight required by the plan is not directly verifiable through automation. See `apps/pixel-design/src-tauri/src/ui/mod.rs:496`, `apps/pixel-design/src-tauri/src/ui/panels.rs:101`, and `apps/pixel-design/src-tauri/src/ui/mod.rs:1208`.

The current E2E coverage clicks every AI panel tool and only asserts that `expanded_ai` changed. It does not verify Gen Fill specifically, the highlighted panel row, Run AI Job using Gen Fill, strip/panel shared selected state for Gen Fill, prompt text preservation while focused, or document data preservation. See `apps/pixel-design/src-tauri/tests/pixel_design_e2e.rs:558`.

## Plan Requirements Not Met
- Automation metadata must expose the Gen Fill panel row selected/active state.
- Tests must verify selecting Gen Fill from the panel updates the highlighted row.
- Tests must verify the next Run AI Job creates a job with `AiTool::GenFill`.
- Tests must verify selecting Gen Fill in the panel after selecting another AI tool in the strip keeps both views on one shared state.
- Tests must verify prompt text is preserved when Gen Fill is selected while the prompt is focused.

## Required Test Shape
- Add a Pixel Design UI automation test that clicks `pd.ai.panel.gen_fill` and asserts `expanded_ai == AiTool::GenFill` plus selected/active metadata for the Gen Fill panel row.
- Select a different `pd.ai_tool.*` strip button, then click `pd.ai.panel.gen_fill` and assert both panel and strip expose Gen Fill as selected.
- Focus `pd.ai.prompt`, type text, click `pd.ai.panel.gen_fill`, and assert the prompt text is unchanged.
- Click `pd.ai.run` after selecting Gen Fill and assert the created job uses `AiTool::GenFill`.
- Assert active layer pixels or document dirty state are unchanged by selecting Gen Fill.

## Required Changes
- Expose selected/active state for AI panel and AI tool strip automation nodes.
- Add Gen Fill-specific E2E tests for panel selection, shared state, prompt preservation, and run-job tool selection.

## Verification
- `cargo test -p tench-pixel-design --test pixel_design_e2e ai_panel_tool_gen_fill`
- `cargo test -p tench-pixel-design`
