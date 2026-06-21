# Top Bar Open Button State

## Source Plan
- `plans/pixel-design/top-bar-open-button-work-plan.md`

## Gap Analysis
The Open button sets `pending_file_action = Some(FileAction::Open)` and status, but the inspected UI code does not route that pending state to a platform file picker or equivalent destination selection flow. See `apps/pixel-design/src-tauri/src/ui/mod.rs:225` and `apps/pixel-design/src-tauri/src/ui/state.rs:477`.

`load_image` can replace the document after decoding, but it does not reset or rebuild history for the newly loaded document and does not clear `pending_file_action`. Old history snapshots can remain associated with the previous document. See `apps/pixel-design/src-tauri/src/ui/mod.rs:40`.

Opening a chosen file does not protect unsaved dirty edits before replacing the current document. The button keeps the current document intact while pending, but completion replaces it immediately after decode succeeds without an unsaved-change confirmation or documented overwrite rule. See `apps/pixel-design/src-tauri/src/ui/mod.rs:41`.

Cancel behavior is not implemented or tested. There is no flow that clears or preserves `pending_file_action` after cancelling the picker while proving document, history, and canvas remain unchanged. See `apps/pixel-design/src-tauri/src/ui/state.rs:452`.

The current E2E coverage clicks Open and only asserts `pending_file_action == Some(FileAction::Open)`. It does not verify status, picker state, dirty-document protection, cancel behavior, successful image load, thumbnail refresh, history reset, or decode error status. See `apps/pixel-design/src-tauri/tests/pixel_design_e2e.rs:350`.

## Plan Requirements Not Met
- Pending Open state must be routed to a platform file picker or equivalent testable picker.
- Open completion must reset history to the loaded document's initial snapshot.
- Open completion must clear or resolve `pending_file_action`.
- Dirty-document protection or an explicit overwrite rule must be implemented before replacing unsaved work.
- Cancel must leave current document, history, canvas, and dirty state unchanged.
- Tests must verify successful load refreshes canvas and thumbnails and reports the loaded document name.
- Tests must verify decode errors surface in status without replacing the current document.

## Required Test Shape
- Click Open and assert pending state, status text, picker automation state, and unchanged current document.
- With dirty edits present, attempt Open and assert the product-defined confirmation/overwrite behavior before loading.
- Cancel the picker and assert pending state, document, history, dirty state, canvas capture, and thumbnails are unchanged.
- Choose a valid image path and assert document replacement, status, history reset, dirty state, flattened capture, and thumbnails.
- Choose an invalid image and assert actionable error status and no document/history/canvas replacement.

## Required Changes
- Connect `FileAction::Open` pending state to the native or testable file picker flow.
- Route selected paths through `load_image` and resolve pending state consistently on success, cancel, and error.
- Reset history when a new document is loaded.
- Add dirty-document protection or document the overwrite behavior in code and tests.
- Add Open button E2E tests for pending state, dirty protection, cancel, successful load, thumbnail refresh, history reset, and decode errors.

## Verification
- `cargo test -p tench-pixel-design --test pixel_design_e2e top_bar_open`
- `cargo test -p tench-pixel-design`
