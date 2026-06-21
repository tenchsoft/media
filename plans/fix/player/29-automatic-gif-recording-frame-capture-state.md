# Automatic GIF Recording Frame Capture State

## Source Plan
- `plans/player/automatic-gif-recording-frame-capture-work-plan.md`

## Gap Analysis
GIF recording appends downsampled frames on `MediaEvent::VideoFrame`, but the duration limit is not enforced from `PlayerState.gif_options.max_duration_secs`. The code keeps a rolling hardcoded cap of 300 frames instead of stopping or finalizing when the configured duration limit is reached. See `apps/player/src-tauri/src/ui/app.rs:366` and `apps/player/src-tauri/src/ui/state.rs:349`.

The current E2E only asserts that the generic `player.automatic.gif_frame_capture` node exists. It does not start recording, inject video frames, assert frame count in `gif_state`, stop recording, or validate that the saved GIF is readable. See `apps/player/src-tauri/tests/plan_ui_e2e.rs:136`.

The automatic GIF frame-capture status node is always emitted with no value for recording state, captured frame count, dimensions, or limit state. Automation cannot assert that frame capture is derived from current backend `VideoFrame` events and canonical recording state. See `apps/player/src-tauri/src/ui/app.rs:2282`.

## Plan Requirements Not Met
- GIF capture must enforce the configured duration limit instead of only applying a hardcoded rolling frame cap.
- Tests must verify `VideoFrame` events append downsampled frames while recording is active.
- Tests must verify `gif_state` includes the captured frame count.
- Tests must verify stopping after captured frames produces a valid saved GIF.
- Automation must expose recording frame count, dimensions, or active capture state.

## Required Test Shape
- Start GIF recording, inject several deterministic `VideoFrame` events, and assert `gif_state == "recording (N frames)"`, internal frame count, and downsampled dimensions.
- Configure a short max duration, inject enough frames to cross the limit, and assert recording stops or finalizes according to the configured limit.
- Stop recording, wait for encoding, and assert the saved GIF file exists and can be decoded.

## Required Changes
- Track elapsed recording duration using GIF options and stop/finalize at `max_duration_secs`.
- Expose GIF frame count, dimensions, and recording active state through `player.automatic.gif_frame_capture`.
- Add test hooks for deterministic video frame injection and GIF output verification.
- Extend `plan_ui_e2e` or targeted UI tests for frame append, duration limit, and saved GIF validity.

## Verification
- `cargo test -p tench-player --test plan_ui_e2e automatic_gif_frame_capture`
- `cargo test -p tench-player`
