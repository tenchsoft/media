# Style Transfer AI Panel Tool Button State

## Source Plan
- `plans/pixel-design/ai-panel-tool-style-transfer-button-work-plan.md`

## Gap Analysis
The panel click handler sets `expanded_ai = AiTool::StyleTransfer`, and the panel renderer highlights rows from that shared state. However, the automation node for `pd.ai.panel.style_transfer` is a generic button and does not expose selected/active state, so the row highlight required by the plan is not directly verifiable through automation. See `apps/pixel-design/src-tauri/src/ui/mod.rs:496`, `apps/pixel-design/src-tauri/src/ui/panels.rs:101`, and `apps/pixel-design/src-tauri/src/ui/mod.rs:1208`.

The current E2E coverage clicks every AI panel tool and only asserts that `expanded_ai` changed. It does not verify Style Transfer specifically, the highlighted panel row, Run AI Job using Style Transfer, strip/panel shared selected state for Style Transfer, prompt text preservation while focused, or document data preservation. See `apps/pixel-design/src-tauri/tests/pixel_design_e2e.rs:558`.

## Plan Requirements Not Met
- Automation metadata must expose the Style Transfer panel row selected/active state.
- Tests must verify selecting Style Transfer from the panel updates the highlighted row.
- Tests must verify the next Run AI Job creates a job with `AiTool::StyleTransfer`.
- Tests must verify selecting Style Transfer in the panel after selecting another AI tool in the strip keeps both views on one shared state.
- Tests must verify prompt text is preserved when Style Transfer is selected while the prompt is focused.

## Required Test Shape
- Add a Pixel Design UI automation test that clicks `pd.ai.panel.style_transfer` and asserts `expanded_ai == AiTool::StyleTransfer` plus selected/active metadata for the Style Transfer panel row.
- Select a different `pd.ai_tool.*` strip button, then click `pd.ai.panel.style_transfer` and assert both panel and strip expose Style Transfer as selected.
- Focus `pd.ai.prompt`, type text, click `pd.ai.panel.style_transfer`, and assert the prompt text is unchanged.
- Click `pd.ai.run` after selecting Style Transfer and assert the created job uses `AiTool::StyleTransfer`.
- Assert active layer pixels or document dirty state are unchanged by selecting Style Transfer.

## Required Changes
- Expose selected/active state for AI panel and AI tool strip automation nodes.
- Add Style Transfer-specific E2E tests for panel selection, shared state, prompt preservation, and run-job tool selection.

## Verification
- `cargo test -p tench-pixel-design --test pixel_design_e2e ai_panel_tool_style_transfer`
- `cargo test -p tench-pixel-design`
