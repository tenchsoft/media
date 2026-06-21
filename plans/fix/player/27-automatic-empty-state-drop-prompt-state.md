# Automatic Empty State Drop Prompt State

## Source Plan
- `plans/player/automatic-empty-state-drop-prompt-work-plan.md`

## Gap Analysis
The existing E2E harness starts from `PlayerState::example()`, which already has media loaded, so it never verifies the launch/no-media empty prompt. The test only asserts that the generic `player.automatic.empty_state_drop_prompt` node exists. See `apps/player/src-tauri/tests/plan_ui_e2e.rs:10` and `apps/player/src-tauri/tests/plan_ui_e2e.rs:139`.

The empty prompt is painted as text when `has_media == false`, but there is no dedicated automation selector for the visible prompt and the automatic status node is emitted regardless of whether the prompt is active. Automation cannot assert that the prompt appears only in empty state or disappears after media loads. See `apps/player/src-tauri/src/ui/paint_video.rs:26` and `apps/player/src-tauri/src/ui/app.rs:2282`.

There is no coverage for resize or side-panel layout changes while the player is empty. The prompt position is derived from the video surface center, but tests do not assert that it remains correctly framed after layout changes. See `apps/player/src-tauri/src/ui/paint_video.rs:146`.

## Plan Requirements Not Met
- Tests must verify the empty prompt appears on launch with `PlayerState::new()`.
- Tests must verify loading media sets `has_media` before the first frame and removes the empty prompt.
- Tests must verify the empty prompt remains correctly positioned after resize or side-panel layout changes.
- Automation must expose active empty-prompt state or a dedicated empty-prompt selector.

## Required Test Shape
- Start a harness with `PlayerState::new()`, capture the UI, and assert `has_media == false`, the empty prompt selector/value is present, and the capture is nonblank.
- Load media through a deterministic test path, capture before any video frame arrives, and assert `has_media == true` and the empty prompt selector is absent.
- Resize or open a side panel while empty and assert the prompt remains visible inside the video surface.

## Required Changes
- Add a dedicated `player.video.empty_prompt` automation node or make `player.automatic.empty_state_drop_prompt` report active state.
- Add no-media and load-media empty-state E2E coverage.
- Add layout-change assertions for the empty prompt.

## Verification
- `cargo test -p tench-player --test plan_ui_e2e automatic_empty_state_drop_prompt`
- `cargo test -p tench-player`
