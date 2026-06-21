# Chapter Export Button State

## Source Plan
- `plans/player/chapter-export-button-work-plan.md`

## Gap Analysis
The native save path writes chapter JSON inside the dialog callback but ignores `std::fs::write` errors and still sends a success result string after the write attempt. It also panics for unsupported URL paths instead of surfacing an actionable failure. See `apps/player/src-tauri/src/ui/app.rs:1201`.

The current no-`app_handle` test fallback only shows `Chapters exported (test)`. The E2E asserts the toast contains `exported`, but it does not verify save dialog invocation, destination filename, file contents, write failure behavior, cancel behavior, or that playback/chapter state is unchanged by export. See `apps/player/src-tauri/tests/plan_ui_e2e.rs:281`.

The export action does not expose the exported JSON or last export result in automation/state, so tests cannot assert the actual chapter list written without a deterministic dialog/output abstraction. See `apps/player/src-tauri/src/ui/state.rs:1278`.

## Plan Requirements Not Met
- Export must surface write errors instead of reporting success.
- Tests must verify `chapters.json` save destination and written JSON content.
- Tests must verify save-dialog cancel leaves state unchanged and shows the documented result.
- Tests must verify export does not mutate unrelated playback or chapter state.
- Export needs deterministic dialog/output hooks for E2E.

## Required Test Shape
- Inject a fake save dialog path, click `player.chapters.export`, and assert the written JSON exactly matches `PlayerState.chapters`.
- Inject a write failure and assert an error toast/result without a success message.
- Inject dialog cancel and assert no file is written and playback/chapter state is unchanged.
- Repeat after adding/removing/importing chapters and assert the exported JSON reflects the current displayed list.

## Required Changes
- Return and surface file-write errors from the export callback.
- Replace callback panic on unsupported path with an actionable error.
- Add deterministic save-dialog/output injection for tests.
- Extend `plan_ui_e2e` Chapter Export coverage for success content, failure, cancel, and no unrelated state changes.

## Verification
- `cargo test -p tench-player --test plan_ui_e2e chapter_export`
- `cargo test -p tench-player`
