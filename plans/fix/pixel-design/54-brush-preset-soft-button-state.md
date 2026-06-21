# Soft Brush Preset Button State

## Source Plan
- `plans/pixel-design/brush-preset-soft-button-work-plan.md`

## Gap Analysis
Soft is defined as `bp2` with size `24`, and `set_brush_preset` updates `brush_preset`, `brush_size`, and status. However, the automation node for `pd.brush.bp2` is a generic button and does not expose selected/active state, so the highlighted Soft card and "only latest selected" behavior are not directly verifiable through automation. See `apps/pixel-design/src-tauri/src/ui/state.rs:593`, `apps/pixel-design/src-tauri/src/ui/state.rs:652`, and `apps/pixel-design/src-tauri/src/ui/mod.rs:1166`.

The current E2E coverage clicks every brush preset and only asserts `brush_preset`. It does not verify Soft-specific size, status text, active highlight, Brush/Eraser availability, eraser size preservation without switching tools, stroke output using the preset size/feel, or document preservation when selecting presets. See `apps/pixel-design/src-tauri/tests/pixel_design_e2e.rs:324`.

## Plan Requirements Not Met
- Automation metadata must expose Soft selected/active state.
- Tests must verify Soft sets brush size to `24` and status to `Brush preset: Soft`.
- Tests must verify Soft remains highlighted until another preset is selected.
- Tests must verify selecting another preset clears Soft highlight.
- Tests must verify Soft can be selected while Eraser is active without switching to Brush.
- Tests must verify Brush and Eraser strokes use the Soft size.

## Required Test Shape
- Add a Pixel Design UI automation test that clicks `pd.brush.bp2` and asserts `brush_preset == "bp2"`, `brush_size == 24`, status text, and selected/active metadata.
- Select a different brush preset and assert Soft is no longer active.
- Switch to Eraser, click `pd.brush.bp2`, and assert active tool remains Eraser while eraser stroke size uses `24`.
- Draw a Brush stroke after selecting Soft and assert changed pixels cover the expected preset-size footprint.
- Assert selecting Soft does not mutate document pixels until a stroke occurs.

## Required Changes
- Expose selected/active state for brush preset automation nodes.
- Add Soft-specific E2E tests for selection, status, highlight, Brush/Eraser behavior, and stroke footprint.

## Verification
- `cargo test -p tench-pixel-design --test pixel_design_e2e brush_preset_soft`
- `cargo test -p tench-pixel-design`
