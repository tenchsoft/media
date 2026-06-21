# CP1252 Subtitle Encoding Button State

## Source Plan
- `plans/player/subtitle-encoding-cp1252-button-work-plan.md`

## Gap Analysis
The current E2E clicks `player.subtitle.encoding.cp1252` and asserts `subtitle_encoding == SubtitleEncoding::Cp1252`, but it does not assert the toast, selected row highlight, pre-load behavior, post-load reload behavior, or persistence after switching drawer tabs. See `apps/player/src-tauri/tests/plan_ui_e2e.rs:359`.

The handler comments that subtitles should reload when the path is known, but no subtitle path is stored or reloaded after encoding changes. See `apps/player/src-tauri/src/ui/app.rs:1120`.

The CP1252 row highlight is drawn by text color only and is not exposed as automation selected/value state, so tests cannot assert selected state through the UI tree. See `apps/player/src-tauri/src/ui/paint_panels.rs:744`.

## Plan Requirements Not Met
- Tests must verify the CP1252 toast.
- Tests must verify CP1252 selected/highlighted state through automation.
- Tests must verify selecting CP1252 before subtitle load affects the next load.
- Tests must verify selecting CP1252 after subtitle load reloads subtitles or shows a clear reload-required state.
- Tests must verify CP1252 persists after switching away from and back to Subtitles.

## Required Test Shape
- Click `player.subtitle.encoding.cp1252` and assert state, selected UI state, and toast.
- Load subtitles after selecting CP1252 and assert the decoder uses CP1252.
- Load subtitles first, select CP1252, and assert reload behavior or a documented reload-required message.
- Switch drawer tabs and return to Subtitles, then assert CP1252 remains highlighted.

## Required Changes
- Store the current subtitle path or reload context so encoding changes can reload subtitles.
- Expose selected state for subtitle encoding rows through automation.
- Extend subtitle encoding E2E coverage for CP1252 toast, reload behavior, and persistence.

## Verification
- `cargo test -p tench-player --test plan_ui_e2e subtitle_encoding_cp1252`
- `cargo test -p tench-player`
