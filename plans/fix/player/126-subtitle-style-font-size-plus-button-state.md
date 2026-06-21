# Subtitle Style Font Size Plus Button State

## Source Plan
- `plans/player/subtitle-style-font-size-plus-button-work-plan.md`

## Gap Analysis
The current E2E clicks `player.subtitle_style.font_size.plus` only after a prior minus click and asserts the value returns to its original state. It does not assert the displayed value label, repeated clamp at `72.0`, close/reopen persistence, or subtitle overlay rendering. See `apps/player/src-tauri/tests/plan_ui_e2e.rs:406`.

The subtitle renderer reads `state.subtitle_font_size`, not `state.subtitle_style.font_size`. Changing Font Size in the style modal therefore does not affect the visible subtitle overlay. See `apps/player/src-tauri/src/ui/paint_video.rs:178`.

Keyboard font-size shortcuts mutate `subtitle_font_size`, while the style modal mutates `subtitle_style.font_size`, creating two independent font-size states. See `apps/player/src-tauri/src/ui/app.rs:2078`.

Automation exposes style controls as generic slider actions without current value metadata, so UI tests cannot assert the displayed Font Size value through the automation tree. See `apps/player/src-tauri/src/ui/app.rs:2433`.

## Plan Requirements Not Met
- Subtitle rendering must use the Font Size value changed by the style modal.
- Font size state must have a single source of truth or documented synchronization.
- Tests must verify the displayed Font Size value updates after plus.
- Tests must verify repeated plus clamps at the documented maximum.
- Tests must verify the changed value persists after closing and reopening the style modal.
- Tests must verify the subtitle overlay visually reflects the new font size.

## Required Test Shape
- Click Font Size plus and assert state, visible value, and automation value.
- Repeatedly click plus and assert the value clamps at `72.0`.
- Close and reopen the style modal and assert the displayed font size persists.
- Render a visible subtitle before and after font-size change and assert text bounds/capture change.
- Verify keyboard font-size shortcuts and modal font-size controls update the same state.

## Required Changes
- Unify `subtitle_font_size` and `subtitle_style.font_size` or synchronize them explicitly.
- Apply style-modal font size in subtitle rendering.
- Expose subtitle style property values through automation.
- Extend subtitle style E2E coverage for clamp, persistence, renderer impact, and keyboard/modal consistency.

## Verification
- `cargo test -p tench-player --test plan_ui_e2e subtitle_style_font_size_plus`
- `cargo test -p tench-player`
