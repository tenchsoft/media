# Chapter Import Button State

## Source Plan
- `plans/player/chapter-import-button-work-plan.md`

## Gap Analysis
The native import callback sends imported JSON through `DialogResult::File`, but `process_dialog_results` handles every `File` result by calling `load_and_play`. As a result, a real file-picker import attempts to open `import_chapters:<json>` as media instead of replacing the chapter list. See `apps/player/src-tauri/src/ui/app.rs:280` and `apps/player/src-tauri/src/ui/app.rs:1227`.

The native path silently does nothing when the dialog sender is unavailable, the user selects an unsupported path, the file read fails, or the picker is cancelled. The click path also does not request a repaint after import-test fallback state changes. See `apps/player/src-tauri/src/ui/app.rs:1227`.

The current E2E uses an escaped JSON string that is not valid chapter JSON and asserts the chapter count remains unchanged, so it does not verify that import replaces the list, preserves title/time/AI metadata, or leaves playback state unchanged. See `apps/player/src-tauri/tests/plan_ui_e2e.rs:293`.

## Plan Requirements Not Met
- Native file-picker import must parse and apply the selected chapter JSON instead of routing it through media loading.
- Import activation and completion/error paths must update user-visible state deterministically and request repaint.
- Tests must verify valid JSON import replaces the displayed chapter list with the expected chapter values.
- Tests must verify import after an existing chapter-list change and confirm unrelated playback state is unchanged.
- Tests must cover picker cancel and read/parse failure behavior without silently swallowing the result.

## Required Test Shape
- Inject valid chapter JSON, click `player.chapters.import`, and assert the chapter list exactly matches the imported title, time, and `ai_generated` values.
- Modify the chapter list first, import different JSON, and assert the rendered rows and state reflect the latest import.
- Capture playback state before import and assert media path, current time, paused state, playlist, and selected playlist index are unchanged.
- Simulate cancel, unreadable file, and malformed JSON paths and assert no stale success toast appears.

## Required Changes
- Add a dedicated dialog result for imported chapter JSON or handle the import prefix before media loading.
- Surface read, parse, unsupported-path, missing-sender, and cancel outcomes through `PlayerState` toast/state.
- Request repaint after every import state mutation.
- Replace the current import assertion in `plan_ui_e2e` with value-level success and failure coverage.

## Verification
- `cargo test -p tench-player --test plan_ui_e2e chapter_import`
- `cargo test -p tench-player`
