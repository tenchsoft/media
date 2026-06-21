# Export Quality Control State

## Source Plan
- `plans/pixel-design/export-quality-control-work-plan.md`

## Gap Analysis
The quality row updates `export_quality` by 5 with 1-100 clamping, but the automation node is labeled only `Quality` and does not expose the displayed percentage. Tests can inspect internal state, but they cannot verify the visible quality value through the UI tree. See `apps/pixel-design/src-tauri/src/ui/mod.rs:1256` and `apps/pixel-design/src-tauri/src/ui/panels.rs:293`.

The current E2E coverage only clicks the left side once and asserts quality decreased. It does not verify right-side increase, clamp at 1, clamp at 100, repeated clicks, visible display updates, or status/acknowledgement behavior. See `apps/pixel-design/src-tauri/tests/pixel_design_e2e.rs:642`.

Export encoding does not receive `export_quality`. `export_document` saves via `image.save(path)`, so JPEG/WebP quality-capable formats ignore the selected quality value. See `apps/pixel-design/src-tauri/src/ui/mod.rs:86`.

## Plan Requirements Not Met
- Export Quality automation must expose the displayed percentage value.
- Tests must verify both decrease and increase interactions.
- Tests must verify quality clamps at 1 and 100.
- Tests must verify the displayed quality value updates after each interaction.
- Export encoding must receive `export_quality` for quality-capable formats.
- Tests must verify low and high quality exports use the selected quality value.

## Required Test Shape
- Click the left and right sides of `pd.export.quality` and assert state value, displayed automation value, and acknowledgement state.
- Drive repeated left clicks to the lower bound and assert quality clamps at 1.
- Drive repeated right clicks to the upper bound and assert quality clamps at 100.
- Export JPEG/WebP at low and high quality values and assert the encoder receives the requested quality according to product-defined output checks.

## Required Changes
- Expose current quality percentage in the `pd.export.quality` automation node.
- Route `state.export_quality` into format-specific export encoders.
- Add Export Quality decrease, increase, clamp, display, status, and encoder tests.

## Verification
- `cargo test -p tench-pixel-design --test pixel_design_e2e export_quality`
- `cargo test -p tench-pixel-design`
