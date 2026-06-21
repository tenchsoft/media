# Add Chapter Modal Cancel Button State

## Source Plan
- `plans/player/add-chapter-modal-cancel-button-work-plan.md`

## Gap Analysis
The existing E2E only asserts that `player.add_chapter.cancel` is present after opening the Add Chapter modal. It never activates Cancel, so it does not verify that the modal closes without adding a chapter or that focus state is cleaned up. See `apps/player/src-tauri/tests/plan_ui_e2e.rs:308`.

The Cancel button is registered as `ClickAction::CloseModal`, but no test covers this shared modal cleanup path from the Add Chapter modal after draft text has been entered. The plan requires stale focus and draft state to be handled consistently after cancellation and reopen. See `apps/player/src-tauri/src/ui/paint_overlays.rs:1006` and `apps/player/src-tauri/src/ui/app.rs:1345`.

The Escape-after-control scenario is not covered for the Cancel path. There is keyboard Escape cleanup for open modals, but no E2E asserts that pressing Escape after Cancel leaves the Add Chapter modal closed and focus flags cleared. See `apps/player/src-tauri/src/ui/app.rs:1975`.

## Plan Requirements Not Met
- Tests must activate `player.add_chapter.cancel` and verify no chapter is added.
- Tests must verify Cancel closes the modal and clears chapter input focus.
- Tests must verify draft text does not reappear after canceling and reopening the modal.
- Tests must verify Escape after Cancel does not leave the modal or focus state half-open.

## Required Test Shape
- Record chapter count, open `player.chapters.add`, type draft text, click `player.add_chapter.cancel`, and assert chapter count is unchanged, modal nodes are absent, and `chapter_name_input_focused == false`.
- Reopen the modal and assert `chapter_name_input` starts empty before any new typing.
- After Cancel, send Escape and assert `show_add_chapter_modal == false`, `chapter_name_input_focused == false`, and no add-modal automation nodes remain.

## Required Changes
- Extend `plan_ui_e2e` Add Chapter modal coverage to click Cancel after text entry.
- Add no-addition, modal absence, focus cleanup, stale-draft, and post-Cancel Escape assertions.

## Verification
- `cargo test -p tench-player --test plan_ui_e2e add_chapter`
- `cargo test -p tench-player`
