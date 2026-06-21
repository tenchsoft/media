# Subtitle Style Position Minus Button State

## Source Plan
- `plans/player/subtitle-style-position-minus-button-work-plan.md`

## Gap Analysis
The current E2E clicks `player.subtitle_style.position.minus` only after a prior plus click and asserts the value returns to its original state. It does not assert the displayed value label, repeated clamp at `0.0`, close/reopen persistence, or subtitle overlay rendering. See `apps/player/src-tauri/tests/plan_ui_e2e.rs:430`.

The subtitle renderer does not read `state.subtitle_style.position`; it uses a fixed subtitle y-position near the bottom of the video. Changing Position in the style modal therefore does not move the visible subtitle overlay. See `apps/player/src-tauri/src/ui/paint_video.rs:178`.

Automation exposes style controls as generic slider actions without current value metadata, so UI tests cannot assert the displayed Position value through the automation tree. See `apps/player/src-tauri/src/ui/app.rs:2433`.

## Plan Requirements Not Met
- Subtitle rendering must use `subtitle_style.position`.
- Tests must verify the displayed Position value updates after minus.
- Tests must verify repeated minus clamps at the documented minimum.
- Tests must verify the changed value persists after closing and reopening the style modal.
- Tests must verify the subtitle overlay visually moves according to the new position.
- Automation must expose style values for assertions.

## Required Test Shape
- Click Position minus and assert state, visible value, and automation value.
- Repeatedly click minus and assert `position == 0.0`.
- Close and reopen the style modal and assert the displayed position persists.
- Render a visible subtitle before and after position change and assert subtitle bounds/capture move as expected.

## Required Changes
- Apply `subtitle_style.position` in subtitle rendering.
- Expose subtitle style property values through automation.
- Extend subtitle style E2E coverage for clamp, persistence, and overlay position assertions.

## Verification
- `cargo test -p tench-player --test plan_ui_e2e subtitle_style_position_minus`
- `cargo test -p tench-player`
