# Context Menu Open File Item State

## Source Plan
- `plans/player/context-menu-open-file-item-work-plan.md`

## Gap Analysis
The test fallback for `open_file_dialog` appends injected paths to the playlist and shows a toast, while the native dialog result loads and plays the selected file through `process_dialog_results`. The context-menu open-file test therefore does not verify the same behavior as the real user flow. See `apps/player/src-tauri/src/ui/app.rs:183` and `apps/player/src-tauri/tests/plan_ui_e2e.rs:559`.

`handle_context_menu_action("open_file")` calls `open_file_dialog()` without requesting repaint after test-state mutation. The menu is cleared before dispatch, but the open-file branch has no `EventCtx` repaint path for the toast or playlist/media changes. See `apps/player/src-tauri/src/ui/app.rs:1492`.

The current E2E clicks `player.context.open_file` after injecting a file but does not assert menu closure, file loading, playlist changes, toast text, or that exactly one command ran. See `apps/player/src-tauri/tests/plan_ui_e2e.rs:562`.

Native dialog cancel, missing dialog sender, and no-app-handle/no-injected-file paths still silently do nothing, so the activation can produce no user-visible acknowledgement.

## Plan Requirements Not Met
- Context open-file test behavior must match the native open-media behavior.
- The context open-file branch must request repaint after state mutation.
- Tests must verify the menu closes after activating Open File.
- Tests must verify exactly one open-file command runs.
- Tests must cover cancel and dialog setup failure without silent no-op.
- Tests must verify no unrelated aspect, repeat, shuffle, or play/pause command runs.

## Required Test Shape
- Inject a file, click `player.context.open_file`, and assert the selected media path/title or documented playlist behavior changed exactly once.
- Assert context-menu selectors are absent after activation.
- Snapshot aspect, repeat, shuffle, and paused state before activation and assert no neighboring context command changed them.
- Run the action twice with different injected files and assert deterministic state transitions.
- Simulate cancel and dialog setup failure and assert a documented toast/state result.

## Required Changes
- Align the test fallback with native open-file semantics or document and test the playlist-only behavior explicitly.
- Request repaint after context-menu open-file state mutations.
- Extend context-menu open-file E2E coverage with menu closure, one-command, success, cancel, and failure assertions.

## Verification
- `cargo test -p tench-player --test plan_ui_e2e context_menu_open_file`
- `cargo test -p tench-player`
