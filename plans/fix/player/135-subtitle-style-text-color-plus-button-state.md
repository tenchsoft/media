# Subtitle Style Text Color Plus Button State

## Source Plan
- `plans/player/subtitle-style-text-color-plus-button-work-plan.md`

## Gap Analysis
The Text Color plus button registers `AdjustSubtitleStyle(2, 0.0)`, and the handler for property `2` is a placeholder no-op. Activating the button cannot change `subtitle_style.text_color`. See `apps/player/src-tauri/src/ui/paint_overlays.rs:496` and `apps/player/src-tauri/src/ui/app.rs:1455`.

The style modal label is hardcoded to `Text Color: White`, so the displayed value cannot reflect any selected color. See `apps/player/src-tauri/src/ui/paint_overlays.rs:490`.

The current E2E only clicks `player.subtitle_style.text_color.plus` in a broad presence loop and does not assert state, displayed value, cycling/clamping, persistence, or rendering. See `apps/player/src-tauri/tests/plan_ui_e2e.rs:440`.

The subtitle renderer does not read `subtitle_style.text_color`; it draws subtitle text with fixed `Color::WHITE`. See `apps/player/src-tauri/src/ui/paint_video.rs:229`.

## Plan Requirements Not Met
- Text Color plus must cycle forward through a defined color set or otherwise change text color.
- Tests must verify the displayed Text Color value updates after plus.
- Tests must verify repeated plus cycles/clamps deterministically.
- Tests must verify the changed value persists after closing and reopening the style modal.
- Tests must verify subtitle rendering uses the selected text color.
- Automation must expose style values for assertions.

## Required Test Shape
- Click Text Color plus and assert `subtitle_style.text_color`, visible label, and automation value change.
- Click repeatedly through the color set and assert documented wrap/clamp behavior.
- Close and reopen the style modal and assert the selected text color persists.
- Render a visible subtitle and assert text pixels/capture reflect the selected color.

## Required Changes
- Define the available subtitle text-color set and cycle behavior.
- Implement `AdjustSubtitleStyle(2, positive_delta)` for text color.
- Render the current text color label instead of a hardcoded value.
- Apply `subtitle_style.text_color` in subtitle rendering.
- Expose subtitle style property values through automation.

## Verification
- `cargo test -p tench-player --test plan_ui_e2e subtitle_style_text_color_plus`
- `cargo test -p tench-player`
