# Subtitle Style Background Opacity Minus Button State

## Source Plan
- `plans/player/subtitle-style-background-opacity-minus-button-work-plan.md`

## Gap Analysis
The current E2E clicks `player.subtitle_style.background_opacity.minus` once and asserts `subtitle_style.bg_opacity` decreased, but it does not assert the displayed value label, repeated clamp at `0.0`, close/reopen persistence, or subtitle overlay rendering. See `apps/player/src-tauri/tests/plan_ui_e2e.rs:412`.

The subtitle renderer does not read `state.subtitle_style.bg_opacity`; it paints the subtitle background with the fixed `SUBTITLE_BG` constant. Changing background opacity in state therefore does not affect the visible subtitle overlay. See `apps/player/src-tauri/src/ui/paint_video.rs:194`.

Automation exposes style controls as generic slider actions without current value metadata, so UI tests cannot assert the displayed background opacity value through the automation tree. See `apps/player/src-tauri/src/ui/app.rs:2433`.

## Plan Requirements Not Met
- Subtitle rendering must use `subtitle_style.bg_opacity`.
- Tests must verify the displayed Background Opacity value updates after minus.
- Tests must verify repeated minus clamps at the documented minimum.
- Tests must verify the changed value persists after closing and reopening the style modal.
- Tests must verify the subtitle overlay background visually reflects the new opacity.
- Automation must expose style values for assertions.

## Required Test Shape
- Click Background Opacity minus and assert state, visible value, and automation value.
- Repeatedly click minus and assert `bg_opacity == 0.0`.
- Close and reopen the style modal and assert the displayed opacity persists.
- Render a visible subtitle before and after opacity change and assert the background pixels/capture change as expected.

## Required Changes
- Apply `subtitle_style.bg_opacity` in subtitle background rendering.
- Expose subtitle style property values through automation.
- Extend subtitle style E2E coverage for clamp, persistence, and overlay visual assertions.

## Verification
- `cargo test -p tench-player --test plan_ui_e2e subtitle_style_background_opacity_minus`
- `cargo test -p tench-player`
