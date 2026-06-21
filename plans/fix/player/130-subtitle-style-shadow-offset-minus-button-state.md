# Subtitle Style Shadow Offset Minus Button State

## Source Plan
- `plans/player/subtitle-style-shadow-offset-minus-button-work-plan.md`

## Gap Analysis
The current E2E clicks `player.subtitle_style.shadow_offset.minus` only in a broad presence loop and does not assert state, displayed value, repeated clamp at `0.0`, close/reopen persistence, or subtitle overlay rendering. See `apps/player/src-tauri/tests/plan_ui_e2e.rs:443`.

The subtitle renderer does not read `state.subtitle_style.shadow_offset`; it draws fixed outline offsets and no configurable shadow. Changing Shadow Offset in the style modal therefore does not affect the visible subtitle overlay. See `apps/player/src-tauri/src/ui/paint_video.rs:205`.

Automation exposes style controls as generic slider actions without current value metadata, so UI tests cannot assert the displayed Shadow Offset value through the automation tree. See `apps/player/src-tauri/src/ui/app.rs:2433`.

## Plan Requirements Not Met
- Tests must verify Shadow Offset minus changes `subtitle_style.shadow_offset`.
- Subtitle rendering must use `subtitle_style.shadow_offset`.
- Tests must verify the displayed Shadow Offset value updates after minus.
- Tests must verify repeated minus clamps at the documented minimum.
- Tests must verify the changed value persists after closing and reopening the style modal.
- Tests must verify the subtitle overlay visually reflects the new shadow offset.
- Automation must expose style values for assertions.

## Required Test Shape
- Click Shadow Offset minus and assert state, visible value, and automation value.
- Repeatedly click minus and assert `shadow_offset == 0.0`.
- Close and reopen the style modal and assert the displayed shadow offset persists.
- Render a visible subtitle before and after shadow-offset change and assert shadow pixels/capture change as expected.

## Required Changes
- Apply `subtitle_style.shadow_offset` in subtitle rendering.
- Expose subtitle style property values through automation.
- Extend subtitle style E2E coverage for state change, clamp, persistence, and overlay visual assertions.

## Verification
- `cargo test -p tench-player --test plan_ui_e2e subtitle_style_shadow_offset_minus`
- `cargo test -p tench-player`
