# Color Picker Apply Button State

## Source Plan
- `plans/pixel-design/color-picker-apply-button-work-plan.md`

## Gap Analysis
The current E2E flow applies a foreground color, then only asserts the modal closed and one recent color exists. It does not assert that `fg_color` changed, `bg_color` stayed unchanged, the preview color was the committed color, or status reads `Color applied`. See `apps/pixel-design/src-tauri/tests/pixel_design_e2e.rs:210`.

The background color path is only covered through Cancel. There is no Apply test proving that background editing changes only `bg_color` and leaves `fg_color` unchanged. See `apps/pixel-design/src-tauri/tests/pixel_design_e2e.rs:227`.

The duplicate recent-color scenario from the plan is not covered. `add_recent_color` has duplicate suppression, but no UI automation test applies the same preview color twice and verifies the recent list is not duplicated or reordered incorrectly. See `apps/pixel-design/src-tauri/src/ui/state.rs:1343`.

## Plan Requirements Not Met
- Tests must verify foreground Apply changes only `fg_color`.
- Tests must verify background Apply changes only `bg_color`.
- Tests must verify Apply commits the selected preview color exactly.
- Tests must verify Apply reports `Color applied`.
- Tests must verify applying a duplicate recent color does not add a duplicate entry.

## Required Test Shape
- Open the foreground picker, adjust hue/SV, capture `color_picker_preview`, click Apply, then assert modal closed, status, `fg_color`, `bg_color`, and recent colors.
- Open the background picker, adjust hue/SV, capture `color_picker_preview`, click Apply, then assert modal closed, status, `bg_color`, `fg_color`, and recent colors.
- Reopen the picker and apply an already recent color, then assert the recent list contains one entry for that color and preserves the product-defined ordering.

## Required Changes
- Add Color Picker Apply E2E assertions for foreground, background, exact preview commit, status text, and duplicate recent-color behavior.

## Verification
- `cargo test -p tench-pixel-design --test pixel_design_e2e color_picker_apply`
- `cargo test -p tench-pixel-design`
