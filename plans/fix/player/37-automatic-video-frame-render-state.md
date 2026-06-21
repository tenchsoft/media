# Automatic Video Frame Render State

## Source Plan
- `plans/player/automatic-video-frame-render-work-plan.md`

## Gap Analysis
The current E2E only asserts that the generic `player.automatic.video_frame` node exists and that clicking the Aspect button changes `aspect_mode`. It does not inject a backend `VideoFrame`, assert `video_frame`/`video_dims`, or verify that pixels render in the video surface. See `apps/player/src-tauri/tests/plan_ui_e2e.rs:128` and `apps/player/src-tauri/tests/plan_ui_e2e.rs:211`.

There is no coverage that aspect mode, drawer/AI panel reservation, and resize recompute the frame draw rect correctly. `paint_video_frame()` calculates draw dimensions from `aspect_mode` and the current video rect, but tests do not assert fit/fill/original/ratio behavior or that the frame remains inside the media surface. See `apps/player/src-tauri/src/ui/paint_video.rs:38`.

The automatic video-frame status node is always emitted with no value for frame dimensions, aspect mode, or computed draw rect. Automation cannot assert that rendering is derived from the current backend frame and canonical layout state. See `apps/player/src-tauri/src/ui/app.rs:2282`.

## Plan Requirements Not Met
- Tests must verify backend `VideoFrame` events update `video_frame`, `video_dims`, and rendered pixels.
- Tests must verify frame draw rect changes correctly for each aspect mode.
- Tests must verify frame layout remains correct after resize and side-panel changes.
- Automation must expose frame dimensions, aspect mode, or computed draw rect for deterministic assertions.

## Required Test Shape
- Inject a deterministic `VideoFrame` event, capture the UI, and assert nonblank rendered pixels in the expected video surface.
- Cycle aspect modes and assert computed draw rect or capture geometry changes according to Fit, Fill, Original, 16:9, and 4:3.
- Open a drawer or AI panel and resize the viewport, then assert the frame remains inside the adjusted video rect without overlapping the panel or controls.

## Required Changes
- Add deterministic video-frame event injection or fake backend hooks.
- Expose video frame dimensions/aspect/draw-rect state in `player.automatic.video_frame`.
- Extend `plan_ui_e2e` or targeted UI tests for frame rendering, aspect modes, resize, and side-panel layout.

## Verification
- `cargo test -p tench-player --test plan_ui_e2e automatic_video_frame_render`
- `cargo test -p tench-player`
