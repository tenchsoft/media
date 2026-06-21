# Add Chapter Modal Add Button State

## Source Plan
- `plans/player/add-chapter-modal-add-button-work-plan.md`

## Gap Analysis
The existing E2E opens the Add Chapter modal, types `New marker`, clicks `player.add_chapter.add`, and only asserts that the Add button disappears. It does not verify that a chapter was added, that the title came from the draft text, that the timestamp matches the current playback time, that the toast confirms the action, or that focus state is cleared. See `apps/player/src-tauri/tests/plan_ui_e2e.rs:307`.

The plan requires deterministic behavior after reopening the modal and when using the default chapter name, but there is no test coverage for stale draft cleanup or empty-input Add behavior. `ShowAddChapterModal` clears the draft on open and `ConfirmAddChapter` has a default-name branch, but neither branch is asserted by the E2E. See `apps/player/src-tauri/src/ui/app.rs:1195` and `apps/player/src-tauri/src/ui/app.rs:1405`.

The Escape cleanup requirement is not covered after the Add button path. Keyboard Escape closes modals and clears chapter focus, but no test asserts that pressing Escape after Add does not leave `show_add_chapter_modal` or `chapter_name_input_focused` half-open. See `apps/player/src-tauri/src/ui/app.rs:1975`.

## Plan Requirements Not Met
- Tests must verify Add commits exactly one chapter with the typed name at the current playback time.
- Tests must verify Add with an empty input uses the default chapter name.
- Tests must verify the confirmation toast appears after Add.
- Tests must verify reopening the modal starts with a clean draft and focused input state.
- Tests must verify Escape after the Add path leaves the modal and focus state closed.

## Required Test Shape
- Seek to a deterministic playback time, open `player.chapters.add`, type a title, click `player.add_chapter.add`, and assert chapter count, inserted title, inserted time, toast text, modal absence, and focus cleanup.
- Reopen the modal, assert the draft is empty, click Add without typing, and assert the new chapter uses the expected default title.
- After a successful Add, send Escape and assert `show_add_chapter_modal == false`, `chapter_name_input_focused == false`, and no add-modal automation nodes remain.

## Required Changes
- Extend `plan_ui_e2e` Add Chapter coverage to inspect `PlayerState.chapters`, toast state, modal state, and focus fields after `player.add_chapter.add`.
- Add default-name, stale-draft, and post-Add Escape assertions for the Add Chapter modal flow.

## Verification
- `cargo test -p tench-player --test plan_ui_e2e add_chapter`
- `cargo test -p tench-player`
