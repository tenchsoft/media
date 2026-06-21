# Subtitle Style Stroke Width Plus Button State

## Source Plan
- `plans/player/subtitle-style-stroke-width-plus-button-work-plan.md`

## Gap Analysis
The current E2E clicks `player.subtitle_style.stroke_width.plus` only in a broad presence loop and does not assert state, displayed value, repeated clamp at `5.0`, close/reopen persistence, or subtitle overlay rendering. See `apps/player/src-tauri/tests/plan_ui_e2e.rs:442`.

The subtitle renderer does not read `state.subtitle_style.stroke_width`; it draws a fixed outline pattern. Changing Stroke Width in the style modal therefore does not affect the visible subtitle overlay. See `apps/player/src-tauri/src/ui/paint_video.rs:205`.

Automation exposes style controls as generic slider actions without current value metadata, so UI tests cannot assert the displayed Stroke Width value through the automation tree. See `apps/player/src-tauri/src/ui/app.rs:2433`.

## Plan Requirements Not Met
- Tests must verify Stroke Width plus changes `subtitle_style.stroke_width`.
- Subtitle rendering must use `subtitle_style.stroke_width`.
- Tests must verify the displayed Stroke Width value updates after plus.
- Tests must verify repeated plus clamps at the documented maximum.
- Tests must verify the changed value persists after closing and reopening the style modal.
- Tests must verify the subtitle overlay visually reflects the new stroke width.
- Automation must expose style values for assertions.

## Required Test Shape
- Click Stroke Width plus and assert state, visible value, and automation value.
- Repeatedly click plus and assert `stroke_width == 5.0`.
- Close and reopen the style modal and assert the displayed stroke width persists.
- Render a visible subtitle before and after stroke-width change and assert outline pixels/capture change as expected.

## Required Changes
- Apply `subtitle_style.stroke_width` in subtitle rendering.
- Expose subtitle style property values through automation.
- Extend subtitle style E2E coverage for state change, clamp, persistence, and overlay visual assertions.

## Verification
- `cargo test -p tench-player --test plan_ui_e2e subtitle_style_stroke_width_plus`
- `cargo test -p tench-player`
