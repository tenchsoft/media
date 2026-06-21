# Playlist Add Files Button State

## Source Plan
- `plans/player/playlist-add-files-button-work-plan.md`

## Gap Analysis
The no-app test injection from `plans/fix/player/01-playlist-add-files.md` is partially implemented, but `ClickAction::AddToPlaylist` calls `open_file_dialog()` without requesting repaint after the test path mutates playlist/toast state. See `apps/player/src-tauri/src/ui/app.rs:1059` and `apps/player/src-tauri/src/ui/app.rs:183`.

The native file dialog uses `pick_file`, so the visible Add Files button can select only one file in the real UI even though the plan and test injection support multiple selected files. See `apps/player/src-tauri/src/ui/app.rs:208`.

The current E2E injects one file after removing a playlist row, but asserts the playlist length remains equal to the post-remove count. It does not verify the selected file was actually added, that multiple files are handled, that the toast is shown, or that unrelated playback state remains unchanged. See `apps/player/src-tauri/tests/plan_ui_e2e.rs:260`.

Dialog cancel, missing dialog sender, no-app-handle without injected files, and invalid/unreadable selected paths still have no documented user-visible result.

## Plan Requirements Not Met
- Add Files must request repaint after playlist/toast state changes.
- Native Add Files must support multiple selected files or document single-file behavior.
- Tests must verify injected files are added to the playlist with expected titles and paths.
- Tests must cover repeated activation with different file selections.
- Tests must verify cancel and dialog setup failure without silent no-op.
- Tests must verify adding files does not change current media path, playback time, paused state, or selected playlist index unless playlist rules explicitly say so.

## Required Test Shape
- Inject one file, click `player.playlist.add_files`, and assert playlist length increases by one with the expected entry.
- Inject multiple files and assert all selected files are appended in order.
- Assert the toast text reports the add result and the capture includes a repaint.
- Repeat with a second injection and assert deterministic append behavior.
- Simulate cancel, missing sender, and no injected files in no-app mode and assert documented results.
- Snapshot playback state before add and assert unrelated fields remain unchanged.

## Required Changes
- Request repaint after Add Files test/native state mutation.
- Use a multi-file native picker for Add Files if multiple selection is required.
- Add deterministic dialog/test hooks for cancel and failure paths.
- Correct and extend the playlist add-files E2E assertions.

## Verification
- `cargo test -p tench-player --test plan_ui_e2e playlist_add_files`
- `cargo test -p tench-player`
