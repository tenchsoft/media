# Color Picker Saturation Value Control State

## Source Plan
- `plans/pixel-design/color-picker-saturation-value-control-work-plan.md`

## Gap Analysis
The saturation/value control updates `color_saturation`, `color_value`, and `color_picker_preview`, but the E2E flow does not assert those immediate state changes, preview color, picker-open state, or `Color preview updated` status after interacting with the control. See `apps/pixel-design/src-tauri/src/ui/mod.rs:170` and `apps/pixel-design/src-tauri/tests/pixel_design_e2e.rs:222`.

The saturation/value area is painted as a single `color_picker_preview` fill instead of a hue-based saturation/value plane. That makes the visual control unable to represent low saturation, high saturation, black, and white choices accurately. See `apps/pixel-design/src-tauri/src/ui/mod.rs:1584`.

The muted-color scenario is not tested. Existing coverage does not apply a low-saturation value to foreground and then verify Brush uses that muted foreground color on canvas pixels. See `apps/pixel-design/src-tauri/tests/pixel_design_e2e.rs:221`.

The black and white extremes are not tested. The coordinate path clamps saturation/value, but there is no UI automation test proving edge clicks or drags produce valid preview colors and valid committed colors. See `apps/pixel-design/src-tauri/src/ui/mod.rs:172`.

Cancel after saturation/value edits is not tested. The existing Cancel path changes hue only and asserts only the background swatch. See `apps/pixel-design/src-tauri/tests/pixel_design_e2e.rs:227`.

## Plan Requirements Not Met
- Tests must verify saturation/value immediately updates `color_saturation`, `color_value`, preview color, status, and keeps the picker open.
- The saturation/value area must visually represent the selected hue across saturation and value.
- Tests must verify a low-saturation foreground color is used by Brush.
- Tests must verify black and white extreme coordinates clamp to valid preview and committed colors.
- Tests must verify Cancel after saturation/value changes commits no swatch changes.

## Required Test Shape
- Open the foreground picker, interact with the saturation/value area, and assert saturation, value, preview, status, and modal state before Apply.
- Apply a low-saturation foreground color, draw with Brush, and assert representative pixels use that muted color.
- Click or drag to black and white extremes, then assert preview and committed swatch colors remain valid and clamped.
- Open the picker, change saturation/value, click Cancel, and assert both swatches and recent colors remain unchanged.
- Add a visual capture assertion that the saturation/value area displays a hue-synchronized 2D range instead of a single flat preview color.

## Required Changes
- Render the saturation/value area as a hue-based saturation/value control.
- Add saturation/value E2E assertions for immediate preview state, muted Brush output, black/white extremes, Cancel rollback, and visual synchronization.

## Verification
- `cargo test -p tench-pixel-design --test pixel_design_e2e color_picker_saturation_value`
- `cargo test -p tench-pixel-design`
