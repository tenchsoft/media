# Export Scale Control State

## Source Plan
- `plans/pixel-design/export-scale-control-work-plan.md`

## Gap Analysis
The scale row updates `export_scale` by 10 with 10-400 clamping, but the automation node is labeled only `Scale` and does not expose the displayed scale percentage or output dimensions. Tests can inspect internal state, but they cannot verify the visible Scale and Output rows through the UI tree. See `apps/pixel-design/src-tauri/src/ui/mod.rs:1261` and `apps/pixel-design/src-tauri/src/ui/panels.rs:301`.

The current E2E coverage only clicks the right side once and asserts scale increased. It does not verify left-side decrease, clamp at 10, clamp at 400, repeated clicks, displayed scale updates, or displayed output-dimension updates. See `apps/pixel-design/src-tauri/tests/pixel_design_e2e.rs:645`.

Export scaling is implemented in `export_document`, but there is no test that writes files at 50 percent and 200 percent and verifies the resulting image dimensions. See `apps/pixel-design/src-tauri/src/ui/mod.rs:86`.

## Plan Requirements Not Met
- Export Scale automation must expose the displayed percentage value and output dimensions.
- Tests must verify both decrease and increase interactions.
- Tests must verify scale clamps at 10 percent and 400 percent.
- Tests must verify displayed Scale and Output rows update after each interaction.
- Tests must verify exported image dimensions at 50 percent and 200 percent.

## Required Test Shape
- Click the left and right sides of `pd.export.scale` and assert state value, displayed automation value, output dimensions, and acknowledgement state.
- Drive repeated left clicks to the lower bound and assert scale clamps at 10 percent.
- Drive repeated right clicks to the upper bound and assert scale clamps at 400 percent.
- Export at 50 percent and 200 percent to temporary files and assert decoded image dimensions match the selected scale.

## Required Changes
- Expose current scale percentage and output dimensions in Export panel automation nodes.
- Add Export Scale decrease, increase, clamp, display, output-dimension, and exported-file tests.

## Verification
- `cargo test -p tench-pixel-design --test pixel_design_e2e export_scale`
- `cargo test -p tench-pixel-design`
