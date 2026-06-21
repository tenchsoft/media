# Canvas Eyedropper Click Control State

## Source Plan
- `plans/pixel-design/canvas-eyedropper-click-control-work-plan.md`

## Gap Analysis
Eyedropper maps the click through the current document rect and samples `composited_image` when available, then writes RGB into `fg_color`, adds it to recent colors, and sets status. However, transparent-pixel behavior is not defined: alpha is ignored, so an empty transparent pixel samples only its RGB channels with no explicit transparent/empty handling. See `apps/pixel-design/src-tauri/src/ui/mod.rs:637` and `apps/pixel-design/src-tauri/src/ui/state.rs:1358`.

The current E2E coverage clicks Eyedropper after a pan and only asserts the status contains "Sampled color". It does not verify foreground color, recent color insertion/de-duplication, composited multi-layer sampling, transparent-pixel behavior, recently painted color sampling, zoom/pan coordinate accuracy, or that document pixels/history remain unchanged. See `apps/pixel-design/src-tauri/tests/pixel_design_e2e.rs:483`.

## Plan Requirements Not Met
- Transparent or empty pixel sampling behavior must be defined and tested.
- Tests must verify foreground color changes to the sampled composited color.
- Tests must verify sampled colors are added to recent colors without duplicates.
- Tests must verify multi-layer composited sampling respects visibility/opacity.
- Tests must verify recently painted colors can be sampled.
- Tests must verify pointer-to-document coordinates after zoom and pan.
- Tests must verify Eyedropper does not mutate document pixels or push document history.

## Required Test Shape
- Add a Pixel Design UI automation test that builds known pixel colors across layers, clicks `pd.canvas.eyedropper_click`, and asserts `fg_color`, status RGB, and recent colors.
- Sample a transparent/empty pixel and assert the product-defined foreground/recent behavior.
- Paint a known color, refresh/capture, sample it, and assert the sampled color matches.
- Zoom and pan, sample at known canvas fractions, and assert mapped document pixels are sampled.
- Assert document pixels, dirty state, and history are unchanged by Eyedropper sampling.

## Required Changes
- Define and implement transparent/empty pixel sampling behavior.
- Expose sampled color and recent color metadata through automation if direct state assertions are insufficient.
- Add Eyedropper E2E and state tests for composited sampling, transparency, recent colors, zoom/pan, and no document mutation.

## Verification
- `cargo test -p tench-pixel-design --test pixel_design_e2e canvas_eyedropper_click`
- `cargo test -p tench-pixel-design`
