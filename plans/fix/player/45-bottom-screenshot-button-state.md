# Bottom Screenshot Button State

## Source Plan
- `plans/player/bottom-screenshot-button-work-plan.md`

## Gap Analysis
The existing E2E only clicks Screenshot without a video frame and asserts the `No video frame` toast. It does not inject a visible frame, verify a PNG is written, decode the saved image, or assert the saved pixels match the current frame. See `apps/player/src-tauri/tests/plan_ui_e2e.rs:219`.

Screenshot output uses the runtime pictures directory and generated timestamp path, with no test-injected output directory or last-screenshot state. This makes success-path assertions non-deterministic and risks writing test artifacts outside the test temp area. See `crates/media-runtime/src/player.rs:105`.

The audio-only and invalid-frame scenarios are not covered. The implementation reports `No video frame to capture` for missing frame or zero dimensions and `Screenshot failed` for frame data mismatch, but tests do not verify those user-facing failure paths. See `apps/player/src-tauri/src/ui/app.rs:694`.

## Plan Requirements Not Met
- Tests must verify Screenshot with a visible frame writes a valid PNG.
- Tests must verify saved PNG pixels/dimensions match the current frame.
- Tests must verify audio-only media shows an actionable no-frame toast.
- Tests must verify no empty image is saved before any frame arrives.
- Screenshot output must be deterministic or injectable for tests.

## Required Test Shape
- Inject a deterministic `VideoFrame`, click `player.controls.screenshot`, assert a success toast with path, read the PNG from a temp output directory, decode it, and compare dimensions/pixels.
- Set audio-only media state with no `video_frame`, click Screenshot, and assert an actionable toast and no file creation.
- Inject mismatched frame data or zero dimensions and assert failure/no-frame toast with no saved image.

## Required Changes
- Add a test output directory or screenshot writer abstraction for deterministic screenshot tests.
- Expose the last screenshot path or result state for automation/test assertions.
- Extend `plan_ui_e2e` Screenshot coverage for success, audio-only, no-frame, and invalid-frame paths.

## Verification
- `cargo test -p tench-player --test plan_ui_e2e bottom_screenshot`
- `cargo test -p tench-player`
