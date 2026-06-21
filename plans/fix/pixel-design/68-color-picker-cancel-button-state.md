# Color Picker Cancel Button State

## Source Plan
- `plans/pixel-design/color-picker-cancel-button-work-plan.md`

## Gap Analysis
The current E2E coverage cancels after opening the background picker and changing hue, then asserts the modal closed and `bg_color` stayed unchanged. It does not assert that `fg_color` stayed unchanged, recent colors were not modified, or status reads `Color cancelled`. See `apps/pixel-design/src-tauri/tests/pixel_design_e2e.rs:227`.

There is no foreground Cancel test. The plan requires Cancel after opening from foreground and verifying the background remains untouched, but the existing UI automation only covers foreground Apply and background Cancel. See `apps/pixel-design/src-tauri/tests/pixel_design_e2e.rs:210`.

The Cancel test changes hue only. It does not cover the hue-plus-saturation/value edit path before Cancel, so rollback after both picker controls are changed is not verified. See `apps/pixel-design/src-tauri/tests/pixel_design_e2e.rs:229`.

## Plan Requirements Not Met
- Tests must verify foreground Cancel leaves both `fg_color` and `bg_color` unchanged.
- Tests must verify background Cancel leaves both `bg_color` and `fg_color` unchanged.
- Tests must verify Cancel after hue and saturation/value edits commits no color.
- Tests must verify Cancel does not add recent colors.
- Tests must verify Cancel reports `Color cancelled`.

## Required Test Shape
- Open the foreground picker, change hue and saturation/value, click Cancel, then assert modal closed, status, both swatches, preview rollback behavior, and recent colors.
- Open the background picker, change hue and saturation/value, click Cancel, then assert modal closed, status, both swatches, preview rollback behavior, and recent colors.

## Required Changes
- Add Color Picker Cancel E2E assertions for foreground rollback, background rollback, hue plus saturation/value edits, status text, and no recent-color mutation.

## Verification
- `cargo test -p tench-pixel-design --test pixel_design_e2e color_picker_cancel`
- `cargo test -p tench-pixel-design`
