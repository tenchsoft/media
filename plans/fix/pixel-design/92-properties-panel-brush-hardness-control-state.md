# Properties Panel Brush Hardness Control State

## Source Plan
- `plans/pixel-design/properties-panel-brush-hardness-control-work-plan.md`

## Gap Analysis
The hardness control updates `state.brush_hardness`, but Brush/Eraser rasterization does not read that value. `BrushStroke::new` only receives color, size, opacity, and eraser mode, and the stroke core has no hardness field. See `apps/pixel-design/src-tauri/src/ui/mod.rs:420`, `apps/pixel-design/src-tauri/src/ui/state.rs:816`, and `crates/pixel-core/src/stroke.rs:35`.

The hardness automation node is labeled only `Hardness` and does not expose the displayed percentage. Tests can inspect internal state, but they cannot verify the visible Properties row value through the UI tree. See `apps/pixel-design/src-tauri/src/ui/mod.rs:1137` and `apps/pixel-design/src-tauri/src/ui/layers.rs:205`.

The current E2E coverage clicks the left side once and only asserts hardness decreased. It does not verify right-side increase, clamp at 0, clamp at 100, displayed value, status or acknowledgement, tool-switch persistence, or Brush/Eraser pixel behavior. See `apps/pixel-design/src-tauri/tests/pixel_design_e2e.rs:320`.

## Plan Requirements Not Met
- Brush/Eraser rasterization must consume the updated brush hardness value.
- Brush hardness automation must expose the displayed percentage.
- Tests must verify both decrease and increase interactions.
- Tests must verify hardness clamps at 0 and 100.
- Tests must verify hardness persists after switching tools and returning to Brush/Eraser.
- Tests must verify future Brush/Eraser strokes reflect the selected hardness.

## Required Test Shape
- Click the left and right halves of `pd.props.hardness` and assert state value, displayed automation value, and acknowledgement state.
- Drive repeated clicks to 0 and 100 and assert clamping.
- Change hardness, switch tools, return to Brush and Eraser, and assert the value persists.
- Draw Brush and Eraser strokes at low and high hardness and assert representative pixels differ according to the product-defined hardness model.

## Required Changes
- Add hardness to the brush stroke/rasterization model or otherwise route `state.brush_hardness` into Brush/Eraser rendering.
- Expose hardness percentage in `pd.props.hardness` automation metadata.
- Add Brush Hardness E2E/core tests for decrease, increase, clamping, persistence, and stroke output.

## Verification
- `cargo test -p tench-pixel-design --test pixel_design_e2e properties_brush_hardness`
- `cargo test -p tench-pixel-design`
