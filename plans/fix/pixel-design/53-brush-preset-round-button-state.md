# Round Brush Preset Button State

## Source Plan
- `plans/pixel-design/brush-preset-round-button-work-plan.md`

## Gap Analysis
Round is defined as `bp1` with size `12`, and `set_brush_preset` updates `brush_preset`, `brush_size`, and status. However, the automation node for `pd.brush.bp1` is a generic button and does not expose selected/active state, so the highlighted Round card and "only latest selected" behavior are not directly verifiable through automation. See `apps/pixel-design/src-tauri/src/ui/state.rs:588`, `apps/pixel-design/src-tauri/src/ui/state.rs:652`, and `apps/pixel-design/src-tauri/src/ui/mod.rs:1166`.

The current E2E coverage clicks every brush preset and only asserts `brush_preset`. It does not verify Round-specific size, status text, active highlight, Brush/Eraser availability, eraser size preservation without switching tools, stroke output using the preset size/feel, or document preservation when selecting presets. See `apps/pixel-design/src-tauri/tests/pixel_design_e2e.rs:324`.

## Plan Requirements Not Met
- Automation metadata must expose Round selected/active state.
- Tests must verify Round sets brush size to `12` and status to `Brush preset: Round`.
- Tests must verify Round remains highlighted until another preset is selected.
- Tests must verify selecting another preset clears Round highlight.
- Tests must verify Round can be selected while Eraser is active without switching to Brush.
- Tests must verify Brush and Eraser strokes use the Round size.

## Required Test Shape
- Add a Pixel Design UI automation test that clicks `pd.brush.bp1` and asserts `brush_preset == "bp1"`, `brush_size == 12`, status text, and selected/active metadata.
- Select a different brush preset and assert Round is no longer active.
- Switch to Eraser, click `pd.brush.bp1`, and assert active tool remains Eraser while eraser stroke size uses `12`.
- Draw a Brush stroke after selecting Round and assert changed pixels cover the expected preset-size footprint.
- Assert selecting Round does not mutate document pixels until a stroke occurs.

## Required Changes
- Expose selected/active state for brush preset automation nodes.
- Add Round-specific E2E tests for selection, status, highlight, Brush/Eraser behavior, and stroke footprint.

## Verification
- `cargo test -p tench-pixel-design --test pixel_design_e2e brush_preset_round`
- `cargo test -p tench-pixel-design`
