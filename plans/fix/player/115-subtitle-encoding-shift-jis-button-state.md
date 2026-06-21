# Shift-JIS Subtitle Encoding Button State

## Source Plan
- `plans/player/subtitle-encoding-shift-jis-button-work-plan.md`

## Gap Analysis
The current E2E asserts `player.subtitle.encoding.shift_jis` is present but never activates it. It does not verify `subtitle_encoding`, toast, selected row highlight, pre-load behavior, post-load reload behavior, or persistence after switching drawer tabs. See `apps/player/src-tauri/tests/plan_ui_e2e.rs:334`.

The handler comments that subtitles should reload when the path is known, but no subtitle path is stored or reloaded after encoding changes. See `apps/player/src-tauri/src/ui/app.rs:1120`.

The Shift-JIS row highlight is drawn by text color only and is not exposed as automation selected/value state, so tests cannot assert selected state through the UI tree. See `apps/player/src-tauri/src/ui/paint_panels.rs:744`.

## Plan Requirements Not Met
- Tests must click Shift-JIS and verify `subtitle_encoding == SubtitleEncoding::ShiftJIS`.
- Tests must verify the Shift-JIS toast.
- Tests must verify Shift-JIS selected/highlighted state through automation.
- Tests must verify selecting Shift-JIS before subtitle load affects the next load.
- Tests must verify selecting Shift-JIS after subtitle load reloads subtitles or shows a clear reload-required state.
- Tests must verify Shift-JIS persists after switching away from and back to Subtitles.

## Required Test Shape
- Click `player.subtitle.encoding.shift_jis` and assert state, selected UI state, and toast.
- Load subtitles after selecting Shift-JIS and assert the decoder uses Shift-JIS.
- Load subtitles first, select Shift-JIS, and assert reload behavior or a documented reload-required message.
- Switch drawer tabs and return to Subtitles, then assert Shift-JIS remains highlighted.

## Required Changes
- Store the current subtitle path or reload context so encoding changes can reload subtitles.
- Expose selected state for subtitle encoding rows through automation.
- Extend subtitle encoding E2E coverage for Shift-JIS activation, toast, reload behavior, and persistence.

## Verification
- `cargo test -p tench-player --test plan_ui_e2e subtitle_encoding_shift_jis`
- `cargo test -p tench-player`
