# Background Color Swatch Button State

## Source Plan
- `plans/pixel-design/background-color-swatch-button-work-plan.md`

## Gap Analysis
The background swatch hit test calls `open_color_picker(false)`, and Apply routes the preview color to `bg_color` when the target is background. Gradient also reads both `fg_color` and `bg_color`. However, automation does not expose the color picker target, preview/original color, or swatch color values, so initialization from the current background color and Apply routing are not directly verifiable through UI metadata. See `apps/pixel-design/src-tauri/src/ui/mod.rs:274`, `apps/pixel-design/src-tauri/src/ui/state.rs:1281`, and `apps/pixel-design/src-tauri/src/ui/mod.rs:1473`.

The current E2E coverage clicks the background swatch and verifies Cancel preserves `bg_color`, but it does not apply a new background color, assert `fg_color` is unchanged, verify picker initialization from the current background, or verify the next Gradient drag uses foreground-to-background colors. See `apps/pixel-design/src-tauri/tests/pixel_design_e2e.rs:227`.

## Plan Requirements Not Met
- Automation metadata must expose color picker target mode and current preview/original color.
- Tests must verify the background picker initializes from the current `bg_color`.
- Tests must verify Apply changes `bg_color` while leaving `fg_color` unchanged.
- Tests must verify Cancel preserves `bg_color` and closes the picker.
- Tests must verify Gradient output uses the newly applied foreground-to-background colors.

## Required Test Shape
- Add a Pixel Design UI automation test that clicks `pd.color.bg` and asserts color picker target is background with preview/original color matching `bg_color`.
- Change hue/SV, click `pd.color_picker.apply`, and assert `bg_color` changes, `fg_color` remains unchanged, status text updates, and the picker closes.
- Reopen background picker, change preview, click Cancel, and assert `bg_color` remains unchanged.
- Select Gradient, drag over a known area, and assert sampled pixels interpolate from foreground to the applied background color.

## Required Changes
- Expose foreground/background swatch color values and color picker target/preview/original values through automation metadata.
- Add background swatch E2E tests covering apply, cancel, initialization, and gradient use.

## Verification
- `cargo test -p tench-pixel-design --test pixel_design_e2e background_color_swatch`
- `cargo test -p tench-pixel-design`
