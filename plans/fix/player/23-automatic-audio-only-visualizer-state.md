# Automatic Audio Only Visualizer State

## Source Plan
- `plans/player/automatic-audio-only-visualizer-work-plan.md`

## Gap Analysis
`MediaEvent::AudioLevels` updates `PlayerState.audio_levels`, but there is no targeted test that injects audio-only media state plus level updates and verifies the visualizer bars render or change. The current E2E only asserts that the generic `player.automatic.audio_visualizer` status node exists. See `apps/player/src-tauri/src/ui/app.rs:529` and `apps/player/src-tauri/tests/plan_ui_e2e.rs:128`.

The automatic visualizer status node is always emitted with no value describing whether audio-only mode is active, what level count was rendered, or whether the levels changed. Automation therefore cannot assert that the visualizer is derived from canonical `PlayerState.audio_levels` and `media_info.resolution`. See `apps/player/src-tauri/src/ui/app.rs:2282`.

There is no coverage for resize or side-panel layout changes after audio levels arrive. The renderer derives bar geometry from the video rect, but tests do not verify that the visualizer remains visible and correctly framed when the AI panel or drawer changes the media surface. See `apps/player/src-tauri/src/ui/paint_video.rs:99`.

## Plan Requirements Not Met
- Tests must verify audio-only media renders visualizer bars from backend audio levels.
- Tests must verify audio level updates change the visual capture without user input.
- Tests must verify the visualizer remains correct after resize or side-panel layout changes.
- Automation must expose enough audio visualizer state to assert active audio-only mode and rendered level count.

## Required Test Shape
- Inject `MediaEvent::AudioOnly` and `MediaEvent::AudioLevels(vec![...])`, capture the UI, and assert the capture is nonblank and changes when levels change.
- Assert `media_info.resolution == "Audio only"` and `audio_levels` contains the injected levels before visual assertions.
- Open a drawer or AI panel and capture again, asserting the visualizer remains within the adjusted video surface and `player.automatic.audio_visualizer` reports active state.

## Required Changes
- Add test hooks or harness utilities to inject audio-only and audio-level backend events.
- Expose audio visualizer automation value such as active state, level count, or resolution.
- Extend `plan_ui_e2e` or targeted UI tests to cover audio-only rendering, automatic level updates, and layout changes.

## Verification
- `cargo test -p tench-player --test plan_ui_e2e automatic_audio_visualizer`
- `cargo test -p tench-player`
