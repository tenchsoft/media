# Top Bar Save Button State

## Source Plan
- `plans/pixel-design/top-bar-save-button-work-plan.md`

## Gap Analysis
The Top Bar Save button does not transition the app into a save-destination flow when the document has no file path. `save_document()` only changes status text, even though `FileAction::SaveAs` exists, so an unsaved document cannot be saved through the button without an external call to `save_to_path()`. See `apps/pixel-design/src-tauri/src/ui/mod.rs:61` and `apps/pixel-design/src-tauri/src/ui/state.rs:477`.

If another file action is already pending, Save does not replace or clear it. The existing E2E clicks Open and then Save, but the implementation can leave `pending_file_action` as `Open` while only changing the status message, which does not satisfy the required state handoff for a related active state. See `apps/pixel-design/src-tauri/src/ui/mod.rs:236` and `apps/pixel-design/src-tauri/src/ui/mod.rs:240`.

The current E2E coverage only asserts that the status text contains `Save`. It does not verify an existing-path save writes a file, clears the dirty dot, preserves dirty state on failure, or reloads the saved image to compare persisted pixels against the canvas. See `apps/pixel-design/src-tauri/tests/pixel_design_e2e.rs:350`.

## Plan Requirements Not Met
- Save on an untitled document must open or request a save destination through `FileAction::SaveAs`.
- Save must replace stale pending file actions such as `Open` with the correct Save/Save As state.
- Tests must verify existing-path Save writes the document and clears the dirty dot.
- Tests must verify failed Save reports the error and keeps dirty state.
- Tests must verify saved pixels persist by reopening or reloading the written file.

## Required Test Shape
- Set up a dirty document with `file_path = None`, click `pd.top.save`, and assert `pending_file_action == Some(FileAction::SaveAs)`, status text is actionable, dirty state remains, and no pending Open action remains.
- Set up a dirty document with a temporary file path, click `pd.top.save`, assert the file exists, dirty dot is absent, status reports success, and `image-runtime` can reload matching pixels.
- Save to an invalid or unwritable path, assert status reports failure and `document.dirty` remains true.

## Required Changes
- Change `save_document()` so the no-path branch sets `pending_file_action = Some(FileAction::SaveAs)` and clears conflicting pending file actions.
- Ensure Save from an existing pending Open state cannot leave `FileAction::Open` active.
- Add Top Bar Save E2E coverage for Save As routing, existing-path write, dirty-dot clearing, failure preservation, and persisted pixel reload.

## Verification
- `cargo test -p tench-pixel-design --test pixel_design_e2e top_bar_save`
- `cargo test -p tench-pixel-design`
