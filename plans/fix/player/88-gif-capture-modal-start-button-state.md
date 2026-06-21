# GIF Capture Modal Start Button State

## Source Plan
- `plans/player/gif-capture-modal-start-button-work-plan.md`

## Gap Analysis
The current E2E clicks `player.gif_modal.start` and only asserts the Stop selector appears. It does not assert `gif_recording == true`, `gif_state == "recording"`, frame buffer clearing, modal state, or recording indicator state. See `apps/player/src-tauri/tests/plan_ui_e2e.rs:590`.

`StartGifRecord` clears `gif_frames` and starts recording, but no test seeds stale frames first to prove the buffer is reset before a new recording. See `apps/player/src-tauri/src/ui/app.rs:906`.

Escape after starting GIF recording is not covered. The Escape path can close `gif_capture_open` through `close_all_panels()` while leaving `gif_recording` and `gif_state` unchanged, producing a hidden active recording state unless that behavior is documented and tested. See `apps/player/src-tauri/src/ui/app.rs:1979` and `apps/player/src-tauri/src/ui/state.rs:1197`.

Reopen-after-start behavior is not covered, so stale recording/modal state can survive across modal reopen without being caught.

## Plan Requirements Not Met
- Tests must verify Start sets `gif_recording == true`.
- Tests must verify Start sets `gif_state == "recording"`.
- Tests must verify Start clears stale frame buffer state.
- Tests must verify the modal reflects recording state beyond selector presence.
- Tests must verify reopening after Start handles recording state deterministically.
- Tests must verify Escape after Start does not leave an undocumented hidden recording state.

## Required Test Shape
- Seed stale GIF frames, open the modal, click Start, and assert `gif_recording`, `gif_state`, frame count, and Stop control state.
- Assert the recording indicator automation state becomes active.
- Press Escape after Start and assert either recording remains visible/documented or recording is stopped with a documented final state.
- Reopen the GIF modal after Start/Escape and assert Start/Stop controls match actual recording state.

## Required Changes
- Add test hooks for GIF frame buffer count and recording state if not already exposed.
- Decide and implement documented Escape behavior while GIF recording is active.
- Extend GIF modal E2E coverage for Start state, stale frame clearing, recording indicator, reopen, and Escape-after-start cases.

## Verification
- `cargo test -p tench-player --test plan_ui_e2e gif_capture_modal_start`
- `cargo test -p tench-player`
