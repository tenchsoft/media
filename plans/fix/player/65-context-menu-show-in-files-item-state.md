# Context Menu Show In Files Item State

## Source Plan
- `plans/player/context-menu-show-in-files-item-work-plan.md`

## Gap Analysis
The test path shows a toast for the current `media_path`, but the handler does not request repaint after setting that toast. The user-visible acknowledgement can therefore be missed by capture-based automation. See `apps/player/src-tauri/src/ui/app.rs:831`.

The native path spawns `show_in_file_manager` and ignores the result. Tests cannot verify the current media path was revealed, and runtime failures are not surfaced or logged through UI state. See `apps/player/src-tauri/src/ui/app.rs:838`.

When `media_path` is `None`, activation silently does nothing. The existing fix noted this path, but the current implementation still has no actionable result for it. See `plans/fix/player/06-context-show-in-files.md`.

The current E2E clicks `player.context.show_in_files` and only checks that some toast exists. It does not assert menu closure, exact path, one-command behavior, platform-call routing, or no unrelated context command. See `apps/player/src-tauri/tests/plan_ui_e2e.rs:566`.

## Plan Requirements Not Met
- Show-in-files must have an observable platform-call result or test hook.
- The handler must request repaint after user-visible state changes.
- Missing media path must produce a documented toast/state instead of silent no-op.
- Tests must verify the context menu closes after activation.
- Tests must verify exactly one reveal command targets the current media path.
- Tests must verify no neighboring context item command changes playback, screenshot, fullscreen, aspect, repeat, or shuffle state.

## Required Test Shape
- With a current `media_path`, click `player.context.show_in_files` and assert the reveal hook receives exactly that path.
- Assert the toast text contains the current path or the documented success/failure message.
- Assert context-menu selectors are absent after activation.
- Clear `media_path`, click again, and assert the documented no-media result.
- Snapshot unrelated context-controlled state and assert it remains unchanged.

## Required Changes
- Add a testable file-manager reveal abstraction or command spy.
- Surface platform reveal failures through toast/state and log context.
- Request repaint after test or UI state changes in `ShowInFiles`.
- Extend context-menu show-in-files coverage with success, no-media, failure, menu-closure, and one-command assertions.

## Verification
- `cargo test -p tench-player --test plan_ui_e2e context_menu_show_in_files`
- `cargo test -p tench-player`
