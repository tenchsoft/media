# Properties Panel Brush Opacity Control State

## Source Plan
- `plans/pixel-design/properties-panel-brush-opacity-control-work-plan.md`

## Gap Analysis
The opacity control updates `state.brush_opacity`, but the automation node is labeled only `Opacity` and does not expose the displayed percentage. Tests can inspect internal state, but they cannot verify the visible Properties row value through the UI tree. See `apps/pixel-design/src-tauri/src/ui/mod.rs:1132` and `apps/pixel-design/src-tauri/src/ui/layers.rs:196`.

The Properties opacity click path does not set a status message or expose another automation acknowledgement beyond the internal state change. See `apps/pixel-design/src-tauri/src/ui/mod.rs:412`.

The current E2E coverage clicks the left side once and only asserts opacity decreased. It does not verify right-side increase, clamp at 1, clamp at 100, displayed value, tool-switch persistence, or Brush/Eraser pixel behavior. See `apps/pixel-design/src-tauri/tests/pixel_design_e2e.rs:317`.

## Plan Requirements Not Met
- Brush opacity automation must expose the displayed percentage.
- Brush opacity interaction must expose deterministic acknowledgement through status or displayed-value metadata.
- Tests must verify both decrease and increase interactions.
- Tests must verify opacity clamps at 1 and 100.
- Tests must verify opacity persists after switching tools and returning to Brush/Eraser.
- Tests must verify future Brush/Eraser strokes use the selected opacity.

## Required Test Shape
- Click the left and right halves of `pd.props.opacity` and assert state value, displayed automation value, and acknowledgement state.
- Drive repeated clicks to 1 and 100 and assert clamping.
- Change opacity, switch tools, return to Brush and Eraser, and assert the value persists.
- Draw Brush and Eraser strokes at low and high opacity and assert representative pixels differ according to the selected opacity.

## Required Changes
- Expose opacity percentage in `pd.props.opacity` automation metadata.
- Add status or equivalent acknowledgement for Properties opacity changes if displayed-value metadata is not sufficient.
- Add Brush Opacity E2E/core tests for decrease, increase, clamping, persistence, and Brush/Eraser output.

## Verification
- `cargo test -p tench-pixel-design --test pixel_design_e2e properties_brush_opacity`
- `cargo test -p tench-pixel-design`
