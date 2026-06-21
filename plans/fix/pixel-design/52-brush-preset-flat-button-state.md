# Flat Brush Preset Button State

## Source Plan
- `plans/pixel-design/brush-preset-flat-button-work-plan.md`

## Gap Analysis
Flat is defined as `bp3` with size `36`, and `set_brush_preset` updates `brush_preset`, `brush_size`, and status. However, the automation node for `pd.brush.bp3` is a generic button and does not expose selected/active state, so the highlighted Flat card and "only latest selected" behavior are not directly verifiable through automation. See `apps/pixel-design/src-tauri/src/ui/state.rs:598`, `apps/pixel-design/src-tauri/src/ui/state.rs:652`, and `apps/pixel-design/src-tauri/src/ui/mod.rs:1166`.

The current E2E coverage clicks every brush preset and only asserts `brush_preset`. It does not verify Flat-specific size, status text, active highlight, Brush/Eraser availability, eraser size preservation without switching tools, stroke output using the preset size/feel, or document preservation when selecting presets. See `apps/pixel-design/src-tauri/tests/pixel_design_e2e.rs:324`.

## Plan Requirements Not Met
- Automation metadata must expose Flat selected/active state.
- Tests must verify Flat sets brush size to `36` and status to `Brush preset: Flat`.
- Tests must verify Flat remains highlighted until another preset is selected.
- Tests must verify selecting another preset clears Flat highlight.
- Tests must verify Flat can be selected while Eraser is active without switching to Brush.
- Tests must verify Brush and Eraser strokes use the Flat size.

## Required Test Shape
- Add a Pixel Design UI automation test that clicks `pd.brush.bp3` and asserts `brush_preset == "bp3"`, `brush_size == 36`, status text, and selected/active metadata.
- Select a different brush preset and assert Flat is no longer active.
- Switch to Eraser, click `pd.brush.bp3`, and assert active tool remains Eraser while eraser stroke size uses `36`.
- Draw a Brush stroke after selecting Flat and assert changed pixels cover the expected preset-size footprint.
- Assert selecting Flat does not mutate document pixels until a stroke occurs.

## Required Changes
- Expose selected/active state for brush preset automation nodes.
- Add Flat-specific E2E tests for selection, status, highlight, Brush/Eraser behavior, and stroke footprint.

## Verification
- `cargo test -p tench-pixel-design --test pixel_design_e2e brush_preset_flat`
- `cargo test -p tench-pixel-design`
