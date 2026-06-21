# URL Modal Input Field Control State

## Source Plan
- `plans/player/url-modal-input-field-control-work-plan.md`

## Gap Analysis
The current URL flow types into `player.url.input`, but it only continues to Play and never asserts that the input click focused the URL field, that typed characters populated `url_input_text`, or that AI/subtitle/chapter text fields lost focus. See `apps/player/src-tauri/tests/plan_ui_e2e.rs:616`.

The automation node for `player.url.input` is exposed as a textbox, but `push_player_node` always sets `value: None` and `focused: false`, so tests cannot verify the visible URL draft or focused state through the UI tree. See `apps/player/src-tauri/src/ui/app.rs:2554` and `apps/player/src-tauri/src/ui/app.rs:2577`.

The Ctrl+L reopen path toggles `url_input_open` directly instead of using the `OpenUrl` dispatch path. After a prior input/cancel/close sequence, this can leave stale draft text available on the next keyboard reopen unless URL modal cleanup is centralized. See `apps/player/src-tauri/src/ui/app.rs:1123` and `apps/player/src-tauri/src/ui/app.rs:2120`.

## Plan Requirements Not Met
- Tests must verify clicking `player.url.input` sets `url_input_focused == true` after one input.
- Tests must verify typing characters updates `url_input_text` exactly.
- Tests must verify URL input focus clears AI, subtitle search, and chapter-name input focus.
- Tests must verify Backspace and Escape after URL input focus update state deterministically.
- Tests must verify reopening the modal after URL input use does not expose stale draft or stale focus.
- Automation must expose enough textbox state to assert URL input focus and value from the UI tree.

## Required Test Shape
- Open URL modal, click `player.url.input`, and assert URL focus is true while other text focus flags are false.
- Type a URL string and assert `url_input_text` equals the typed text.
- Send Backspace and assert the text updates by one character.
- Send Escape and assert URL modal selectors are absent, `url_input_open == false`, and `url_input_focused == false`.
- Reopen the modal through Ctrl+L and the toolbar/open action path, then assert the URL input starts from the expected empty state.

## Required Changes
- Populate automation `value` and `focused` for `player.url.input`.
- Add focused URL input tests to `plan_ui_e2e` or a dedicated URL modal E2E test.
- Centralize URL modal open/close state handling so keyboard and click paths reset draft/focus consistently.

## Verification
- `cargo test -p tench-player --test plan_ui_e2e url_modal_input_field_control`
- `cargo test -p tench-player`
