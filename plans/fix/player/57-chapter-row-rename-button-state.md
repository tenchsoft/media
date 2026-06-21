# Chapter Row Rename Button State

## Source Plan
- `plans/player/chapter-row-rename-button-work-plan.md`

## Gap Analysis
Clicking a rename button only sets `chapter_rename_idx` and `chapter_rename_text`; there is no rendered rename editor, input selector, confirm button, or cancel button tied to that state. The only chapter modal currently rendered is the add-chapter modal. See `apps/player/src-tauri/src/ui/app.rs:1185` and `apps/player/src-tauri/src/ui/paint_overlays.rs:937`.

`RenameChapter(idx)` stores the requested index even when the index is invalid, with an empty rename string. `ConfirmChapterRename` then clears the rename state and shows `Chapter renamed` even if no chapter was renamed. See `apps/player/src-tauri/src/ui/app.rs:1185` and `apps/player/src-tauri/src/ui/app.rs:1257`.

The current E2E clicks `player.chapter.rename.0` but does not assert that rename state opens, that the current title is prefilled, that editing changes the target row, or that playback state is unchanged. See `apps/player/src-tauri/tests/plan_ui_e2e.rs:307`.

There is no coverage for first, middle, and last rename buttons, nor for row targeting after import/add/delete changes the chapter list. Stale row indices would not be caught.

## Plan Requirements Not Met
- Rename activation must show an editable rename field prefilled with the clicked chapter title.
- Rename confirm must rename the targeted chapter only when the target index is valid.
- Invalid rename indices must not show a false success message.
- Tests must verify first, middle, and last row rename targeting.
- Tests must repeat rename after chapter-list mutation and verify row indices refresh.
- Tests must verify rename does not change media path, playback time, paused state, playlist, or selected playlist index.

## Required Test Shape
- Click a row rename selector and assert rename input, confirm, and cancel automation nodes are present.
- Assert the rename input value equals the current chapter title before typing.
- Type a new title, confirm, and assert only that row title changes.
- Cancel a rename and assert the chapter list is unchanged.
- Add/import/delete chapters, rename a displayed row again, and assert the current row index is targeted.
- Exercise an invalid-index path through a unit test and assert no false success toast is shown.

## Required Changes
- Render a chapter rename overlay or inline editor bound to `chapter_rename_idx` and `chapter_rename_text`.
- Expose stable automation nodes for rename input, confirm, and cancel.
- Guard `RenameChapter` and `ConfirmChapterRename` against invalid indices before setting success state.
- Add value-level rename coverage to `plan_ui_e2e` or a focused player UI automation test.

## Verification
- `cargo test -p tench-player --test plan_ui_e2e chapter_row_rename`
- `cargo test -p tench-player`
