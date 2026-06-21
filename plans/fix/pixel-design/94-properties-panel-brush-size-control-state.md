# Properties Panel Brush Size Control State

## Source Plan
- `plans/pixel-design/properties-panel-brush-size-control-work-plan.md`

## Gap Analysis
The size control updates `state.brush_size`, but the automation node is labeled only `Size` and does not expose the displayed numeric value. Tests can inspect internal state, but they cannot verify the visible Properties row value through the UI tree. See `apps/pixel-design/src-tauri/src/ui/mod.rs:1127` and `apps/pixel-design/src-tauri/src/ui/layers.rs:187`.

The Properties size click path does not set a status message or expose another automation acknowledgement beyond the internal state change. See `apps/pixel-design/src-tauri/src/ui/mod.rs:404`.

The current E2E coverage clicks the left side once and only asserts size decreased. It does not verify right-side increase, clamp at 1, clamp at 200, displayed value, tool-switch persistence, or Brush/Eraser pixel behavior. See `apps/pixel-design/src-tauri/tests/pixel_design_e2e.rs:314`.

## Plan Requirements Not Met
- Brush size automation must expose the displayed value.
- Brush size interaction must expose deterministic acknowledgement through status or displayed-value metadata.
- Tests must verify both decrease and increase interactions.
- Tests must verify size clamps at 1 and 200.
- Tests must verify size persists after switching tools and returning to Brush/Eraser.
- Tests must verify future Brush/Eraser strokes use the selected size.

## Required Test Shape
- Click the left and right halves of `pd.props.size` and assert state value, displayed automation value, and acknowledgement state.
- Drive repeated clicks to 1 and 200 and assert clamping.
- Change size, switch tools, return to Brush and Eraser, and assert the value persists.
- Draw Brush and Eraser strokes at small and large sizes and assert representative pixels differ according to the selected size.

## Required Changes
- Expose brush size value in `pd.props.size` automation metadata.
- Add status or equivalent acknowledgement for Properties size changes if displayed-value metadata is not sufficient.
- Add Brush Size E2E/core tests for decrease, increase, clamping, persistence, and Brush/Eraser output.

## Verification
- `cargo test -p tench-pixel-design --test pixel_design_e2e properties_brush_size`
- `cargo test -p tench-pixel-design`
