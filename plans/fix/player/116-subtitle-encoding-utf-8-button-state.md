# UTF-8 Subtitle Encoding Button State

## Source Plan
- `plans/player/subtitle-encoding-utf-8-button-work-plan.md`

## Gap Analysis
The current E2E asserts `player.subtitle.encoding.utf_8` is present but never activates it. It does not verify click behavior, toast, selected row highlight after switching away from another encoding, pre-load behavior, post-load reload behavior, or persistence after switching drawer tabs. See `apps/player/src-tauri/tests/plan_ui_e2e.rs:333`.

The handler comments that subtitles should reload when the path is known, but no subtitle path is stored or reloaded after encoding changes. See `apps/player/src-tauri/src/ui/app.rs:1120`.

The UTF-8 row highlight is drawn by text color only and is not exposed as automation selected/value state, so tests cannot assert selected state through the UI tree. See `apps/player/src-tauri/src/ui/paint_panels.rs:744`.

## Plan Requirements Not Met
- Tests must click UTF-8 and verify `subtitle_encoding == SubtitleEncoding::Utf8`.
- Tests must verify the UTF-8 toast.
- Tests must verify UTF-8 selected/highlighted state through automation after switching from another encoding.
- Tests must verify selecting UTF-8 before subtitle load affects the next load.
- Tests must verify selecting UTF-8 after subtitle load reloads subtitles or shows a clear reload-required state.
- Tests must verify UTF-8 persists after switching away from and back to Subtitles.

## Required Test Shape
- Switch to another encoding, click `player.subtitle.encoding.utf_8`, and assert state, selected UI state, and toast.
- Load subtitles after selecting UTF-8 and assert the decoder uses UTF-8.
- Load subtitles first, select UTF-8, and assert reload behavior or a documented reload-required message.
- Switch drawer tabs and return to Subtitles, then assert UTF-8 remains highlighted.

## Required Changes
- Store the current subtitle path or reload context so encoding changes can reload subtitles.
- Expose selected state for subtitle encoding rows through automation.
- Extend subtitle encoding E2E coverage for UTF-8 activation, toast, reload behavior, and persistence.

## Verification
- `cargo test -p tench-player --test plan_ui_e2e subtitle_encoding_utf_8`
- `cargo test -p tench-player`
