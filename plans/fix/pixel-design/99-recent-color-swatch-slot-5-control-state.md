# Recent Color Swatch Slot 5 Control State

## Source Plan
- `plans/pixel-design/recent-color-swatch-slot-5-control-work-plan.md`

## Gap Analysis
Selecting recent color slot 5 sets `fg_color` and status, but it does not promote the selected color in `recent_colors`. The plan requires recent color ordering to update when a slot is selected. See `apps/pixel-design/src-tauri/src/ui/mod.rs:285`.

Recent swatch automation nodes are only created for occupied slots and are labeled `Recent N` without exposing the stored color value. Tests cannot verify the exact slot 5 color or click an empty fixed slot 5 through a stable selector. See `apps/pixel-design/src-tauri/src/ui/mod.rs:1038`.

The current E2E coverage creates six recent colors, clicks each recent slot, and only asserts status contains the slot number. It does not verify slot 5's exact color becomes foreground, ordering promotion, foreground swatch repaint, empty-slot behavior, or Brush/Fill pixels using the selected color. See `apps/pixel-design/src-tauri/tests/pixel_design_e2e.rs:263`.

## Plan Requirements Not Met
- Selecting slot 5 must promote that color in recent color order.
- Recent slot 5 automation must expose the stored color value.
- Empty slot 5 behavior must be testable and must not change colors.
- Tests must verify slot 5's exact stored color becomes `fg_color`.
- Tests must verify Brush and Fill use the selected slot 5 color.

## Required Test Shape
- Populate recent colors, record slot 5's color, click `pd.color.recent.5`, and assert `fg_color`, foreground swatch value, status, picker-closed state, and recent ordering.
- In an empty recent-color state, click slot 5 by the product-defined fixed slot control and assert `fg_color`, `bg_color`, recent colors, status, and modal state remain unchanged.
- Select slot 5, paint with Brush and Fill, and assert representative pixels match the stored slot color.

## Required Changes
- Promote selected recent colors according to the product-defined ordering rule.
- Expose fixed recent slot automation nodes, including empty state and stored color metadata.
- Add Recent Color Slot 5 E2E tests for exact color selection, ordering promotion, empty-slot no-op behavior, foreground repaint, and Brush/Fill output.

## Verification
- `cargo test -p tench-pixel-design --test pixel_design_e2e recent_color_slot_5`
- `cargo test -p tench-pixel-design`
