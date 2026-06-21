# Subtitle Search Modal Input Field Control State

## Source Plan
- `plans/player/subtitle-search-modal-input-field-control-work-plan.md`

## Gap Analysis
The current E2E types into `player.subtitle_search.input`, but it does not assert that the input gained focus, that `subtitle_search_text` contains the typed text, that other text focus flags were cleared, or that the visible input value updated. See `apps/player/src-tauri/tests/plan_ui_e2e.rs:375`.

Automation exposes the subtitle search input as a textbox selector but does not expose its current value or focused state, so UI-level assertions for typed text and focus restoration are not possible. See `apps/player/src-tauri/src/ui/app.rs:2410`.

`ShowSubtitleSearch` clears the input text on open, while `CloseModal` leaves existing search text/results in state. The reopen and Escape behavior for stale search draft is not documented or tested. See `apps/player/src-tauri/src/ui/app.rs:1314` and `apps/player/src-tauri/src/ui/app.rs:1345`.

Typing in the input updates text but does not update search results until a separate search action occurs. That behavior is not documented in the input-control tests. See `apps/player/src-tauri/src/ui/app.rs:2038`.

## Plan Requirements Not Met
- Tests must verify activating the input sets `subtitle_search_focused == true`.
- Tests must verify typed characters update `subtitle_search_text`.
- Tests must verify visible input value through automation.
- Tests must verify focus exclusivity with AI, URL, and chapter-name inputs.
- Tests must verify reopen behavior for stale search draft/results.
- Tests must verify Escape after input use leaves modal and focus state consistent.
- Input/search-result update timing must be documented and tested.

## Required Test Shape
- Open search, click the input, type text, and assert focus flags, internal text, visible value, and no unrelated text focus.
- Press Backspace/Enter and assert the documented edit/search behavior.
- Close and reopen search and assert the input is cleared or restored according to product rules.
- Press Escape after typing and assert search modal/focus state is closed consistently.

## Required Changes
- Expose textbox value and focused state for `player.subtitle_search.input`.
- Add subtitle search input state coverage to `plan_ui_e2e` or a focused modal/input test.
- Document whether typing should search live or require Find/Enter.

## Verification
- `cargo test -p tench-player --test plan_ui_e2e subtitle_search_input`
- `cargo test -p tench-player`
