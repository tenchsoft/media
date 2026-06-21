# GIF Options Modal Start Recording Button State

## Source Plan
- `plans/player/gif-options-modal-start-recording-button-work-plan.md`

## Gap Analysis
The current E2E opens GIF options and asserts the Start selector is present, but it never clicks `player.gif_options.start`. Recording state, modal closure, frame buffer clearing, option usage, and repaint are unverified for this entry point. See `apps/player/src-tauri/tests/plan_ui_e2e.rs:596`.

The visible Start Recording button dispatches `StartGifRecording`, while a separate `StartGifWithOptions` action also exists and closes both the options modal and GIF capture modal. The two paths have different modal behavior, and the visible options path does not document whether the parent GIF capture modal should remain open. See `apps/player/src-tauri/src/ui/paint_overlays.rs:770`, `apps/player/src-tauri/src/ui/app.rs:1298`, and `apps/player/src-tauri/src/ui/app.rs:1398`.

Configured GIF options are not proven to affect recording or encoding. Duration enforcement is already captured in `plans/fix/player/29-automatic-gif-recording-frame-capture-state.md`, but this visible Start Recording entry point still needs tests that the current options are the ones used.

Escape-after-start and reopen-after-start behavior are not covered, so hidden active recording or stale options modal state can survive unnoticed.

## Plan Requirements Not Met
- Tests must click `player.gif_options.start`.
- Tests must verify recording starts with the configured GIF options.
- Tests must verify `gif_options_open == false` after activation.
- Tests must verify `gif_recording == true` and `gif_state == "recording"`.
- The visible options-start path must have documented parent GIF modal behavior.
- Tests must verify stale frame/options state is handled consistently after reopen.
- Tests must verify Escape after options-start does not leave a half-open modal state.

## Required Test Shape
- Set non-default GIF options, click `player.gif_options.start`, and assert the recording configuration uses those values.
- Assert `gif_options_open == false`, `gif_recording == true`, `gif_state == "recording"`, and stale frames are cleared.
- Assert whether `gif_capture_open` remains open or closes according to documented behavior.
- Reopen GIF options during or after recording and assert controls reflect the current state.
- Press Escape after options-start and assert modal flags and recording state remain documented and consistent.

## Required Changes
- Consolidate `StartGifRecording` and `StartGifWithOptions` semantics or document why they differ.
- Ensure the visible Start Recording path applies the configured GIF options to capture/encoding.
- Add GIF options Start Recording coverage to `plan_ui_e2e` or a focused GIF modal test.
- Expose active GIF recording configuration through state or automation for assertions.

## Verification
- `cargo test -p tench-player --test plan_ui_e2e gif_options_modal_start_recording`
- `cargo test -p tench-player`
