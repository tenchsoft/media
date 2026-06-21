# Auto Subtitle Encoding Button State

## Source Plan
- `plans/player/subtitle-encoding-auto-button-work-plan.md`

## Gap Analysis
There is no `Auto` subtitle encoding state. The Auto row dispatches `SetSubtitleEncoding("Auto")`, but the handler falls through to `SubtitleEncoding::Utf8`, so selecting Auto actually selects UTF-8. See `apps/player/src-tauri/src/ui/paint_panels.rs:730` and `apps/player/src-tauri/src/ui/app.rs:1107`.

The Auto row is never highlighted because its `enc_val` is `None` and `is_current` is hardcoded false for that case. See `apps/player/src-tauri/src/ui/paint_panels.rs:744`.

The handler comments that subtitles should reload when the path is known, but no subtitle path is stored or reloaded after encoding changes. See `apps/player/src-tauri/src/ui/app.rs:1120`.

The current E2E asserts `player.subtitle.encoding.auto` is present but never activates it. It does not verify Auto state, toast, pre-load behavior, post-load reload behavior, or persistence after switching drawer tabs. See `apps/player/src-tauri/tests/plan_ui_e2e.rs:332`.

## Plan Requirements Not Met
- Auto must be represented as a distinct subtitle decoding mode.
- Auto selection must not silently become UTF-8.
- Auto row must expose highlighted/selected state when active.
- Tests must click Auto and verify the selected encoding state and toast.
- Tests must verify selecting Auto before subtitle load affects the next load.
- Tests must verify selecting Auto after subtitle load reloads subtitles or shows a clear reload-required state.
- Tests must verify Auto persists after switching away from and back to Subtitles.

## Required Test Shape
- Click `player.subtitle.encoding.auto` and assert a distinct Auto state, selected UI state, and `Encoding: Auto` toast.
- Load subtitles after selecting Auto and assert the decoder uses auto detection.
- Load subtitles first, select Auto, and assert reload behavior or a documented reload-required message.
- Switch drawer tabs and return to Subtitles, then assert Auto remains highlighted.

## Required Changes
- Add `SubtitleEncoding::Auto` or a separate decoding mode model.
- Update the Auto row highlight logic to reflect active Auto state.
- Store the current subtitle path or reload context so encoding changes can reload subtitles.
- Extend subtitle encoding E2E coverage for Auto activation, toast, reload, and persistence.

## Verification
- `cargo test -p tench-player --test plan_ui_e2e subtitle_encoding_auto`
- `cargo test -p tench-player`
