# Bg Remove AI Panel Tool Button State

## Source Plan
- `plans/pixel-design/ai-panel-tool-bg-remove-button-work-plan.md`

## Gap Analysis
The panel click handler sets `expanded_ai = AiTool::BgRemove`, and the panel renderer highlights rows from that shared state. However, the automation node for `pd.ai.panel.bg_remove` is a generic button and does not expose selected/active state, so the row highlight required by the plan is not directly verifiable through automation. See `apps/pixel-design/src-tauri/src/ui/mod.rs:496`, `apps/pixel-design/src-tauri/src/ui/panels.rs:101`, and `apps/pixel-design/src-tauri/src/ui/mod.rs:1208`.

The current E2E coverage clicks every AI panel tool and only asserts that `expanded_ai` changed. It does not verify Bg Remove specifically, the highlighted panel row, Run AI Job using Bg Remove, strip/panel shared selected state for Bg Remove, prompt text preservation while focused, or document data preservation. See `apps/pixel-design/src-tauri/tests/pixel_design_e2e.rs:558`.

## Plan Requirements Not Met
- Automation metadata must expose the Bg Remove panel row selected/active state.
- Tests must verify selecting Bg Remove from the panel updates the highlighted row.
- Tests must verify the next Run AI Job creates a job with `AiTool::BgRemove`.
- Tests must verify selecting Bg Remove in the panel after selecting another AI tool in the strip keeps both views on one shared state.
- Tests must verify prompt text is preserved when Bg Remove is selected while the prompt is focused.

## Required Test Shape
- Add a Pixel Design UI automation test that clicks `pd.ai.panel.bg_remove` and asserts `expanded_ai == AiTool::BgRemove` plus selected/active metadata for the Bg Remove panel row.
- Select a different `pd.ai_tool.*` strip button, then click `pd.ai.panel.bg_remove` and assert both panel and strip expose Bg Remove as selected.
- Focus `pd.ai.prompt`, type text, click `pd.ai.panel.bg_remove`, and assert the prompt text is unchanged.
- Click `pd.ai.run` after selecting Bg Remove and assert the created job uses `AiTool::BgRemove`.
- Assert active layer pixels or document dirty state are unchanged by selecting Bg Remove.

## Required Changes
- Expose selected/active state for AI panel and AI tool strip automation nodes.
- Add Bg Remove-specific E2E tests for panel selection, shared state, prompt preservation, and run-job tool selection.

## Verification
- `cargo test -p tench-pixel-design --test pixel_design_e2e ai_panel_tool_bg_remove`
- `cargo test -p tench-pixel-design`
