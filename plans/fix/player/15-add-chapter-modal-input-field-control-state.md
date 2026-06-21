# Add Chapter Modal Input Field Control State

## Source Plan
- `plans/player/add-chapter-modal-input-field-control-work-plan.md`

## Gap Analysis
The Add Chapter input automation node is emitted through the generic click-region path, which always sets `value: None` and `focused: false`. The control can receive focus in `PlayerState`, but automation cannot observe the focused state or typed draft value required by the plan. See `apps/player/src-tauri/src/ui/app.rs:2217` and `apps/player/src-tauri/src/ui/app.rs:2576`.

The current E2E uses `type_text(&mut harness, "player.add_chapter.input", "New marker")`, then immediately clicks Add. It does not assert that `chapter_name_input_focused` became true, that `chapter_name_input` contains the typed text before submission, or that other text focus flags were cleared. See `apps/player/src-tauri/tests/plan_ui_e2e.rs:317`.

The stale draft and Escape cleanup scenarios are not covered for the input field path. Opening the modal clears the draft and focuses the field, and Escape clears modal focus, but tests do not assert those states after reopen or after pressing Escape from a focused chapter input. See `apps/player/src-tauri/src/ui/app.rs:1195`, `apps/player/src-tauri/src/ui/app.rs:1338`, and `apps/player/src-tauri/src/ui/app.rs:1975`.

## Plan Requirements Not Met
- `player.add_chapter.input` automation must expose the current input value.
- `player.add_chapter.input` automation must expose focused state when the field is active.
- Tests must verify clicking the input focuses only the chapter name field.
- Tests must verify typed text updates `chapter_name_input` before Add is clicked.
- Tests must verify reopening clears stale draft text and Escape clears modal/input focus.

## Required Test Shape
- Open the Add Chapter modal, click `player.add_chapter.input`, and assert `chapter_name_input_focused == true` while AI, URL, and subtitle-search focus flags are false.
- Type text and assert both `PlayerState.chapter_name_input` and the textbox automation `value` match the draft.
- Close or submit the modal, reopen it, and assert the draft starts empty and the field focus state is deterministic.
- Press Escape while the chapter input is focused and assert modal nodes are absent and `chapter_name_input_focused == false`.

## Required Changes
- Special-case text input automation nodes so `player.add_chapter.input` reports `value` and `focused` from `PlayerState`.
- Extend `plan_ui_e2e` Add Chapter input coverage for focus, typed value, stale draft cleanup, and Escape cleanup.

## Verification
- `cargo test -p tench-player --test plan_ui_e2e add_chapter`
- `cargo test -p tench-player`
