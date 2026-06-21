# Automatic Seek Hover Thumbnail Preview State

## Source Plan
- `plans/player/automatic-seek-hover-thumbnail-preview-work-plan.md`

## Gap Analysis
The current E2E only asserts that the generic `player.automatic.seek_hover_thumbnail` node exists. It does not move the pointer over the seekbar, assert `seek_hover_pos`, verify thumbnail generation, confirm the time preview changes, or ensure leaving the seekbar clears hover state. See `apps/player/src-tauri/tests/plan_ui_e2e.rs:128`.

The automatic seek-hover node is always emitted with no value for hover ratio, preview time, or thumbnail availability. Automation cannot assert that the preview is derived from the current seekbar geometry and backend thumbnail data. See `apps/player/src-tauri/src/ui/app.rs:2282`.

There is no coverage proving hover does not seek. The pointer-move path updates `seek_hover_pos` and may generate a thumbnail, but tests do not assert that `current_time` remains unchanged until click or drag. See `apps/player/src-tauri/src/ui/app.rs:1841`.

The renderer only draws the preview when both `seek_hover_pos` and `seek_thumbnail` are present, but there is no test for backend thumbnail generation failure or stale thumbnail clearing when a new hover position cannot produce a thumbnail. See `apps/player/src-tauri/src/ui/paint_controls.rs:232`.

## Plan Requirements Not Met
- Tests must verify hover over several seekbar positions updates preview position and time.
- Tests must verify hover does not seek until click or drag.
- Tests must verify leaving the seekbar clears `seek_hover_pos` and `seek_thumbnail`.
- Tests must verify preview geometry remains correct after resize or side-panel layout changes.
- Automation must expose hover ratio, preview time, or thumbnail availability.

## Required Test Shape
- Move the pointer to multiple seekbar positions and assert `seek_hover_pos`, preview time, and visual capture change while `current_time` stays unchanged.
- Use a fake backend thumbnail result and assert `seek_thumbnail` becomes present and the preview renders.
- Move outside the seekbar and assert `seek_hover_pos == None`, `seek_thumbnail == None`, and preview selectors/values are absent.
- Open a drawer or resize, hover again, and assert the ratio maps to the resized seekbar geometry.

## Required Changes
- Expose seek-hover ratio, preview time, and thumbnail-present state in `player.automatic.seek_hover_thumbnail`.
- Add fake backend thumbnail generation hooks for deterministic tests.
- Extend `plan_ui_e2e` or targeted UI tests for hover movement, no-seek behavior, leave cleanup, thumbnail rendering, and layout changes.

## Verification
- `cargo test -p tench-player --test plan_ui_e2e automatic_seek_hover_thumbnail`
- `cargo test -p tench-player`
