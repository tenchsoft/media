# Foreground Color Swatch Button State

## Source Plan
- `plans/pixel-design/foreground-color-swatch-button-work-plan.md`

## Gap Analysis
The foreground swatch automation node is labeled only `FG` and does not expose the displayed color value. Tests can inspect state, but they cannot verify through the UI tree that the swatch display and picker initialization match. See `apps/pixel-design/src-tauri/src/ui/mod.rs:1028`.

The current E2E coverage opens the foreground picker and verifies picker controls are present, but it does not assert `color_picker_target_fg`, `active_modal`, `color_picker_original`, `color_picker_preview`, or HSV values initialized from `fg_color`. See `apps/pixel-design/src-tauri/tests/pixel_design_e2e.rs:210`.

Foreground Apply is not verified against the target swatch. Existing coverage applies a color but only asserts the modal closed and recent-color count, not that `fg_color` changed and `bg_color` remained unchanged. See `apps/pixel-design/src-tauri/tests/pixel_design_e2e.rs:221`.

Foreground Cancel is not covered. There is no test that opens the foreground picker, changes hue or saturation/value, cancels, and verifies the original foreground color remains unchanged. See `apps/pixel-design/src-tauri/src/ui/state.rs:1330`.

The Eyedropper-to-foreground-picker scenario is not covered. Eyedropper samples into `fg_color`, but no test opens the picker after sampling and verifies HSV controls and preview start from the sampled foreground color. See `apps/pixel-design/src-tauri/src/ui/state.rs:1358`.

## Plan Requirements Not Met
- Foreground swatch automation must expose the displayed foreground color.
- Tests must verify foreground picker target mode and initialization from current `fg_color`.
- Tests must verify foreground Apply changes only `fg_color`.
- Tests must verify foreground Cancel restores the original foreground color.
- Tests must verify picker HSV/preview initialization after Eyedropper sampling.

## Required Test Shape
- Click `pd.color.fg` and assert modal state, foreground target, original color, preview color, HSV state, and swatch automation value.
- Change picker controls and Apply, then assert `fg_color`, unchanged `bg_color`, recent colors, status, and closed modal.
- Change picker controls and Cancel, then assert original `fg_color`, unchanged `bg_color`, recent colors, status, and closed modal.
- Sample a known pixel with Eyedropper, open the foreground picker, and assert preview/HSV/swatch values match the sampled color.

## Required Changes
- Expose foreground swatch color value in automation.
- Add foreground swatch picker-open, Apply, Cancel, and Eyedropper-initialization tests.

## Verification
- `cargo test -p tench-pixel-design --test pixel_design_e2e foreground_color_swatch`
- `cargo test -p tench-pixel-design`
