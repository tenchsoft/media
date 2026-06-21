# URL Modal Play Button State

## Source Plan
- `plans/player/url-modal-play-button-work-plan.md`

## Gap Analysis
The current E2E types a URL and clicks `player.url.play`, but it only asserts the Play selector is absent afterward. It does not verify the submitted URL became the current media path/title, that the modal focus was cleared, or that playback/loading state changed as expected. See `apps/player/src-tauri/tests/plan_ui_e2e.rs:616`.

`SubmitUrl` trims the draft and calls `load_media_for_action` for non-empty input, then closes the modal. In the no-backend fallback used by headless tests, `load_media_for_action` calls `open_media`, which marks media loaded but leaves `is_playing = false`, so the required "playback begins loading the URL" behavior is not covered by a deterministic state assertion. See `apps/player/src-tauri/src/ui/app.rs:1128`, `apps/player/src-tauri/src/ui/app.rs:313`, and `apps/player/src-tauri/src/ui/state.rs:860`.

The Play path does not clear `url_input_text`, and the Ctrl+L reopen path toggles the modal directly. After submitting a URL, the stale submitted draft can still be visible on a keyboard reopen unless URL modal open/close state is centralized. See `apps/player/src-tauri/src/ui/app.rs:1133` and `apps/player/src-tauri/src/ui/app.rs:2120`.

## Plan Requirements Not Met
- Tests must assert clicking `player.url.play` submits the trimmed typed URL into current media state.
- Tests must verify URL Play closes the modal and clears URL focus after one input.
- Tests must verify the backend/no-backend loading state expected after URL Play, including `has_media`, media title/path, toast, and playing/loading flag.
- Tests must cover repeated URL Play after reopening the modal and assert the second URL replaces the first deterministically.
- Tests must verify stale submitted draft text is not exposed after reopening and that Escape after the play path leaves the modal fully closed.

## Required Test Shape
- Open URL modal, type a URL with surrounding whitespace, click `player.url.play`, and assert the stored path/title uses the trimmed URL.
- Assert `url_input_open == false`, `url_input_focused == false`, URL selectors are absent, and media state reflects a loaded URL.
- Reopen the modal, submit a different URL, and assert media state updates to the second URL without retaining the first draft.
- Press Escape after URL Play and assert no URL modal selectors return and focus remains cleared.
- Run the test in the no-backend harness with an explicit assertion for the expected fallback loading/playing state.

## Required Changes
- Define and implement the no-backend URL Play state contract so tests can distinguish "loaded for playback" from a closed modal with no media change.
- Clear or reset URL draft state after successful submit, or guarantee every reopen path resets it before rendering.
- Add focused URL Play state coverage to `plan_ui_e2e` or a dedicated URL modal E2E test.

## Verification
- `cargo test -p tench-player --test plan_ui_e2e url_modal_play_button`
- `cargo test -p tench-player`
