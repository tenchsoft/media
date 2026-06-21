# URL Modal Cancel Button State

## Source Plan
- `plans/player/url-modal-cancel-button-work-plan.md`

## Gap Analysis
The URL modal E2E opens the modal, types a URL, and clicks Play, but it never clicks `player.url.cancel`, verifies modal dismissal from the cancel control, or snapshots media/playback state to prove cancel does not commit the draft URL. See `apps/player/src-tauri/tests/plan_ui_e2e.rs:604`.

The Cancel button is rendered as a `CloseModal` click target, while the separate `CancelUrl` dispatch arm is incomplete and only flips `url_input_open`. Any future path that uses `CancelUrl` would leave `url_input_focused` intact and would not share the same cleanup rules as `CloseModal`. See `apps/player/src-tauri/src/ui/paint_overlays.rs:452` and `apps/player/src-tauri/src/ui/app.rs:1137`.

`CloseModal` and Escape clear URL focus, but neither clears `url_input_text`. Reopening with Ctrl+L toggles `url_input_open` directly instead of using the `OpenUrl` path that clears the draft, so text entered before cancel can reappear on a later keyboard reopen. See `apps/player/src-tauri/src/ui/app.rs:1345`, `apps/player/src-tauri/src/ui/app.rs:1985`, and `apps/player/src-tauri/src/ui/app.rs:2120`.

## Plan Requirements Not Met
- Tests must click `player.url.cancel` after entering draft text and assert the modal closes after one input.
- Tests must assert cancel leaves current media path, current URL/display label, playback state, and playback time unchanged.
- Tests must verify cancel clears URL focus and does not leave stale draft text visible after reopening the modal.
- Tests must verify Escape after the cancel path leaves the URL modal fully closed.
- URL cancel/close/open keyboard paths must share one cleanup/opening policy so draft and focus state are deterministic.

## Required Test Shape
- Open the URL modal, snapshot media/playback state, focus `player.url.input`, and type a draft URL.
- Click `player.url.cancel` and assert `player.url.input`, `player.url.play`, and `player.url.cancel` are absent.
- Assert `url_input_open == false`, `url_input_focused == false`, and media/playback snapshot fields are unchanged.
- Reopen with Ctrl+L and assert stale draft text is not exposed through state or automation value.
- Press Escape after the cancel path and assert the URL modal remains fully closed with URL focus cleared.

## Required Changes
- Route `CancelUrl`, URL `CloseModal`, Escape, and Ctrl+L open/close through shared URL modal helpers, or remove the unused `CancelUrl` arm if `CloseModal` is the only intended cancel action.
- Clear URL draft text on cancel/close or guarantee every reopen path resets it before the modal is visible.
- Add focused URL cancel state coverage to `plan_ui_e2e` or a dedicated URL modal E2E test.
- Expose the URL input value through automation if the stale draft assertion cannot be made from product state alone.

## Verification
- `cargo test -p tench-player --test plan_ui_e2e url_modal_cancel_button`
- `cargo test -p tench-player`
