# Subtitle Style Font Family Plus Button State

## Source Plan
- `plans/player/subtitle-style-font-family-plus-button-work-plan.md`

## Gap Analysis
The Font Family plus button registers `AdjustSubtitleStyle(1, 0.0)`, and the handler for property `1` is a no-op. Activating the button cannot change `subtitle_style.font_family`. See `apps/player/src-tauri/src/ui/paint_overlays.rs:496` and `apps/player/src-tauri/src/ui/app.rs:1454`.

The current E2E only asserts `player.subtitle_style.font_family.plus` is present. It never clicks the button or verifies font-family state, displayed value, cycling/clamping, persistence, or rendering. See `apps/player/src-tauri/tests/plan_ui_e2e.rs:438`.

The subtitle renderer does not read `subtitle_style.font_family`; it draws subtitles with the default text path only. See `apps/player/src-tauri/src/ui/paint_video.rs:213`.

## Plan Requirements Not Met
- Font Family plus must cycle forward or otherwise change font family according to product rules.
- Tests must verify the displayed Font Family value updates after plus.
- Tests must verify repeated plus cycles/clamps deterministically.
- Tests must verify the changed value persists after closing and reopening the style modal.
- Tests must verify subtitle rendering uses the selected font family.
- Automation must expose style values for assertions.

## Required Test Shape
- Click Font Family plus and assert `subtitle_style.font_family`, visible label, and automation value change.
- Click repeatedly through the full font list and assert documented wrap/clamp behavior.
- Close and reopen the style modal and assert the selected font family persists.
- Render a visible subtitle and assert the renderer receives or reflects the selected font family.

## Required Changes
- Define the available subtitle font-family list and cycle behavior.
- Implement `AdjustSubtitleStyle(1, positive_delta)` for font family.
- Apply `subtitle_style.font_family` in subtitle rendering.
- Expose subtitle style property values through automation.
- Add Font Family plus E2E coverage for cycle, persistence, and rendering.

## Verification
- `cargo test -p tench-player --test plan_ui_e2e subtitle_style_font_family_plus`
- `cargo test -p tench-player`
