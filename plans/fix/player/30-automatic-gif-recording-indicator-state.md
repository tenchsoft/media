# Automatic GIF Recording Indicator State

## Source Plan
- `plans/player/automatic-gif-recording-indicator-work-plan.md`

## Gap Analysis
The current E2E starts and stops GIF recording, but it only asserts the modal Start/Stop button state. It does not verify that the red recording indicator appears on the video surface while recording and disappears after stop. See `apps/player/src-tauri/tests/plan_ui_e2e.rs:581`.

The automatic GIF recording indicator node is always emitted with no active/inactive value, so automation cannot distinguish recording from idle state through `player.automatic.gif_recording_indicator`. See `apps/player/src-tauri/src/ui/app.rs:2304`.

The plan requires the indicator to stay over the current video rect after layout changes, but there is no test that starts recording, opens a side panel or resizes, and verifies the REC dot remains correctly placed. See `apps/player/src-tauri/src/ui/paint_video.rs:236`.

## Plan Requirements Not Met
- Tests must verify the REC indicator appears only while GIF recording is active.
- Tests must verify both recording start paths produce the same indicator state.
- Tests must verify the indicator remains correctly positioned after resize or side-panel layout changes.
- Automation must expose whether the GIF recording indicator is active.

## Required Test Shape
- Click `player.gif_modal.start`, assert `gif_recording == true`, assert `player.automatic.gif_recording_indicator` reports active state, and compare capture against the idle state.
- Click `player.gif_modal.stop`, assert `gif_recording == false`, active indicator state is false, and capture changes.
- Start recording through the GIF options path and assert the same indicator state.
- Open a drawer or AI panel while recording and assert the indicator bounds remain inside the adjusted video surface.

## Required Changes
- Expose active indicator state or dedicated `player.video.gif_rec_indicator` automation only while recording.
- Extend `plan_ui_e2e` GIF coverage for indicator visible/hidden state, alternate start path, and layout-change placement.

## Verification
- `cargo test -p tench-player --test plan_ui_e2e automatic_gif_recording_indicator`
- `cargo test -p tench-player`
