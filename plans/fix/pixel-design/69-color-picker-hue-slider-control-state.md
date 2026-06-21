# Color Picker Hue Slider Control State

## Source Plan
- `plans/pixel-design/color-picker-hue-slider-control-work-plan.md`

## Gap Analysis
The hue control updates `color_hue` and `color_picker_preview`, but the E2E flow does not assert the immediate hue value, preview color, picker-open state, or `Color preview updated` status after interacting with the slider. See `apps/pixel-design/src-tauri/src/ui/mod.rs:161` and `apps/pixel-design/src-tauri/tests/pixel_design_e2e.rs:221`.

The saturation/value area is painted as a single `color_picker_preview` fill. After hue changes, the area does not render a hue-synchronized saturation/value plane, so the visual control can fail the preview/SV synchronization requirement even though state fields update. See `apps/pixel-design/src-tauri/src/ui/mod.rs:1584`.

The Apply path after hue movement is only asserted indirectly through modal close and recent-color length. It does not verify that the target swatch committed the hue preview or that the non-target swatch stayed unchanged. See `apps/pixel-design/src-tauri/tests/pixel_design_e2e.rs:223`.

The Cancel path after hue movement only covers the background picker and only asserts `bg_color` stayed unchanged. It does not verify foreground Cancel or both swatches remaining unchanged after hue edits. See `apps/pixel-design/src-tauri/tests/pixel_design_e2e.rs:227`.

Recent color selection is tested only as a direct foreground swatch assignment. There is no test that selects a recent color, opens the picker, and verifies the hue slider/preview starts from that recent color before adjustment. See `apps/pixel-design/src-tauri/src/ui/mod.rs:285` and `apps/pixel-design/src-tauri/tests/pixel_design_e2e.rs:263`.

## Plan Requirements Not Met
- Tests must verify hue slider immediately updates `color_hue`, preview color, status, and keeps the picker open.
- The saturation/value control must visually synchronize with the selected hue.
- Tests must verify hue Apply commits the preview to only the target swatch.
- Tests must verify hue Cancel commits no swatch changes.
- Tests must verify the picker hue starts from a selected recent color.

## Required Test Shape
- Open the foreground picker, interact with the hue slider, and assert `color_hue`, `color_picker_preview`, status, and modal state before Apply.
- Apply after hue movement and assert exact preview commit to `fg_color`, unchanged `bg_color`, status, and recent colors.
- Open the background picker, interact with hue, Cancel, and assert both swatches and recent colors remain unchanged.
- Select a recent color, open the picker, and assert initial hue/preview match that recent color before moving the slider.
- Add a visual capture assertion that the saturation/value area changes consistently when hue changes.

## Required Changes
- Render the saturation/value area from the selected hue instead of a single preview-color fill.
- Add hue slider E2E assertions for immediate preview state, Apply, Cancel, recent-color initialization, and SV visual synchronization.

## Verification
- `cargo test -p tench-pixel-design --test pixel_design_e2e color_picker_hue`
- `cargo test -p tench-pixel-design`
