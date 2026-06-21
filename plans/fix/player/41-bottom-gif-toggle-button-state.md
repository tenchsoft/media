# Bottom GIF Toggle Button State

## Source Plan
- `plans/player/bottom-gif-toggle-button-work-plan.md`

## Gap Analysis
When `ToggleGifCapture` is used while recording, it sets `gif_recording = false`, moves `gif_state` to `encoding`, drains frames, starts a background encode, and then toggles `gif_capture_open`. It does not report success/failure, does not transition out of `encoding`, and can open the modal if recording was started from an options path with the modal closed. See `apps/player/src-tauri/src/ui/app.rs:881`.

The modal Stop path behaves differently from the bottom toggle path: it shows a saved/no-frames toast, closes the modal, and immediately sets `gif_state` back to `idle`. The two stop paths are not consistent. See `apps/player/src-tauri/src/ui/app.rs:912`.

The existing E2E opens the modal from `player.controls.gif`, starts recording, and stops with `player.gif_modal.stop`. It does not use the bottom GIF toggle while recording, assert `gif_capture_open`, `gif_state`, `gif_recording`, frame buffer state, or encoding result. See `apps/player/src-tauri/tests/plan_ui_e2e.rs:581`.

## Plan Requirements Not Met
- Bottom GIF toggle while recording must have a documented and deterministic modal state.
- Bottom GIF toggle while recording must transition encoding to saved, failed, or idle state.
- Bottom GIF toggle and modal Stop must use consistent stop/encode state handling.
- Tests must verify idle click opens the ready modal and recording click stops/encodes.
- Tests must verify closing the modal returns `gif_state` to idle or a documented state.

## Required Test Shape
- From idle, click `player.controls.gif` and assert `gif_capture_open == true`, `gif_state == "ready"`, and Start/Close controls are present.
- Start recording, then click `player.controls.gif` instead of modal Stop and assert `gif_recording == false`, frame buffer is drained or preserved as documented, modal state is deterministic, and encoding result/toast is observable.
- Compare bottom-toggle stop and modal Stop to ensure they produce the same documented state transitions.
- Close the modal and assert `gif_state` is idle or the documented non-idle state.

## Required Changes
- Unify GIF stop/encode handling between `ToggleGifCapture` and `StopGifRecord`.
- Add completion/error handling for background GIF encoding and update `gif_state`/toast.
- Define whether bottom toggle should close or keep closed the modal when stopping a recording.
- Extend `plan_ui_e2e` GIF coverage for bottom toggle idle, bottom toggle while recording, modal close, and state/toast assertions.

## Verification
- `cargo test -p tench-player --test plan_ui_e2e bottom_gif_toggle`
- `cargo test -p tench-player`
