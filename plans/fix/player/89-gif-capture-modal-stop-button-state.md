# GIF Capture Modal Stop Button State

## Source Plan
- `plans/player/gif-capture-modal-stop-button-work-plan.md`

## Gap Analysis
The current E2E clicks `player.gif_modal.stop` and only asserts the Stop selector disappears. It does not assert `gif_recording == false`, `gif_state == "idle"`, modal closure, frame buffer drain, toast text, or recording indicator state. See `apps/player/src-tauri/tests/plan_ui_e2e.rs:592`.

The Stop path reports `GIF saved: <path>` before the background encoder completes and ignores encode failures. A failed encode can still show a success toast. See `apps/player/src-tauri/src/ui/app.rs:912`.

The no-frames Stop path is not asserted even though the current E2E likely reaches it. Tests do not verify the `No frames captured` toast or that no output file is created.

Stopping after real captured frames is not covered. The saved GIF path, file existence, decodability, frame dimensions, and cleanup are already related to `plans/fix/player/29-automatic-gif-recording-frame-capture-state.md`, but the modal Stop entry point still needs explicit coverage.

Escape-after-stop and reopen-after-stop behavior are not covered, so stale modal/recording state can survive unnoticed.

## Plan Requirements Not Met
- Tests must verify Stop sets `gif_recording == false`.
- Tests must verify Stop sets `gif_state` to the documented idle/final state.
- Tests must verify the no-frames toast and no-output behavior.
- Tests must verify saved GIF success only after encode succeeds.
- Tests must verify encode errors surface as failure state/toast.
- Tests must verify reopening and Escape after Stop do not leave half-open modal state.

## Required Test Shape
- Start recording with no frames, click Stop, and assert `gif_recording == false`, `gif_state == "idle"`, modal closed, indicator inactive, and `No frames captured` toast.
- Start recording with deterministic frames, click Stop, wait for encode completion, and assert a valid saved GIF file and success toast.
- Inject encode failure and assert an error toast/state without a success message.
- Reopen the GIF modal after Stop and assert controls reset to Start/Options/Close.
- Press Escape after Stop and assert all GIF modal flags remain closed.

## Required Changes
- Report GIF encode success/failure after the encoder result is known.
- Add deterministic GIF frame and output hooks for modal Stop tests.
- Extend GIF modal E2E coverage for no-frame stop, successful encode, encode failure, reopen, and Escape-after-stop cases.

## Verification
- `cargo test -p tench-player --test plan_ui_e2e gif_capture_modal_stop`
- `cargo test -p tench-player`
