# Bottom Aspect Mode Button State

## Source Plan
- `plans/player/bottom-aspect-mode-button-work-plan.md`

## Gap Analysis
The visible Aspect button label is drawn from `aspect_mode.label()`, but automation labels come from the generic `ClickAction` debug label, so tests cannot assert the displayed label changes from Fit to Fill, 1:1, 16:9, 4:3, and back to Fit. See `apps/player/src-tauri/src/ui/paint_controls.rs:453` and `apps/player/src-tauri/src/ui/app.rs:2544`.

The existing E2E clicks `player.controls.aspect` once and only asserts that `aspect_mode` changed. It does not verify the full cycle, label changes, toast text, current-frame redraw while paused, or side-panel-aware video geometry. See `apps/player/src-tauri/tests/plan_ui_e2e.rs:211`.

There is no targeted visual or geometry test proving `paint_video_frame()` recomputes the draw rect after aspect changes. This is especially unverified for paused playback and open drawer/AI panel layouts. See `apps/player/src-tauri/src/ui/paint_video.rs:49`.

## Plan Requirements Not Met
- Automation must expose the current visible Aspect button label or aspect mode.
- Tests must verify the full aspect cycle returns to Fit.
- Tests must verify toast text for each aspect mode.
- Tests must verify the current frame redraws while paused.
- Tests must verify aspect geometry respects side-panel width.

## Required Test Shape
- Click `player.controls.aspect` through Fit, Fill, 1:1, 16:9, 4:3, and back to Fit, asserting `aspect_mode`, visible label/value, and toast after each click.
- With a deterministic video frame and paused playback, change aspect and assert the frame capture changes according to the new draw rect.
- Open a drawer or AI panel, change aspect, and assert the computed frame rect remains inside the reduced video surface.

## Required Changes
- Expose current aspect mode or visible label through `player.controls.aspect`.
- Add deterministic frame geometry assertions for aspect changes.
- Extend `plan_ui_e2e` Aspect coverage for full cycle, toast, paused redraw, and side-panel layout.

## Verification
- `cargo test -p tench-player --test plan_ui_e2e bottom_aspect_mode`
- `cargo test -p tench-player`
