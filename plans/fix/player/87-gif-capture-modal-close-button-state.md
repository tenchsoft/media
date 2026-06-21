# GIF Capture Modal Close Button State

## Source Plan
- `plans/player/gif-capture-modal-close-button-work-plan.md`

## Gap Analysis
The GIF modal Close button dispatches `ToggleGifCapture`, not a close-only action. If recording is active, closing the modal stops recording, starts encoding, and leaves `gif_state` as `encoding` without completion/error handling. This shared inconsistency is related to `plans/fix/player/41-bottom-gif-toggle-button-state.md`, but the modal Close entry point still needs its own coverage. See `apps/player/src-tauri/src/ui/paint_overlays.rs:109` and `apps/player/src-tauri/src/ui/app.rs:881`.

The current E2E clicks `player.gif_modal.close` only after stopping recording and cancelling options, then does not assert `gif_capture_open`, `gif_state`, `gif_recording`, frame buffer state, or stale options state. See `apps/player/src-tauri/tests/plan_ui_e2e.rs:602`.

Escape-after-close behavior is not covered. The keyboard path clears modal flags independently, but no test verifies the GIF modal is not left half-open after Close followed by Escape. See `apps/player/src-tauri/src/ui/app.rs:1981`.

## Plan Requirements Not Met
- GIF modal Close must have a documented close-only behavior when recording is active.
- Tests must verify Close sets `gif_capture_open == false`.
- Tests must verify Close leaves `gif_state` in `idle` or another documented non-recording state.
- Tests must verify Close clears or preserves GIF options state consistently.
- Tests must verify reopening after Close does not show stale modal state.
- Tests must verify Escape after Close does not leave the modal half-open.

## Required Test Shape
- Open the GIF modal, click Close, and assert `gif_capture_open == false`, `gif_recording == false`, and documented `gif_state`.
- Start recording, click Close, and assert the documented stop/close behavior, frame handling, toast, and final state.
- Open options, cancel or close, then close the GIF modal and assert `gif_options_open == false`.
- Reopen the GIF modal and assert Start/Options/Close state is clean.
- Press Escape after Close and assert all GIF modal flags remain closed.

## Required Changes
- Split GIF modal Close from bottom GIF toggle if close-only behavior should not stop/encode implicitly.
- Add completion/error handling or a documented final state for Close while recording.
- Extend GIF modal E2E coverage with close, recording-close, reopen, options cleanup, and Escape-after-close cases.

## Verification
- `cargo test -p tench-player --test plan_ui_e2e gif_capture_modal_close`
- `cargo test -p tench-player`
