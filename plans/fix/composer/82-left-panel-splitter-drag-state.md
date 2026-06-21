# Left Panel Splitter Drag State

## Source Plan

- `plans/composer/left-panel-splitter-drag-control-work-plan.md`

## Gap Analysis

Primary pointer down dispatches registered click regions before it checks splitter hit zones. The left splitter currently works when no click region overlaps it, but the splitter is not guaranteed to win before click handling as required. See `apps/composer/src-tauri/src/ui/mod.rs:805` and `apps/composer/src-tauri/src/ui/mod.rs:811`.

The current E2E coverage drags the left splitter once and only asserts that the capture changed. It does not assert the left panel width value, clamp behavior beyond the allowed range, drag-state clearing after release, or overlay recalculation while overlays are open. See `apps/composer/src-tauri/tests/plan_ui_e2e.rs:181`.

## Plan Requirements Not Met

- Splitter hit zones must be prioritized before generic click handling when a pointer down starts inside the splitter zone.
- Left splitter drag tests must assert the actual `left_panel_w` change, not just a capture change.
- Dragging beyond the minimum and maximum bounds must be tested for clamping without panel overlap.
- Pointer release must be tested to clear splitter drag state and keep the final layout stable.
- Overlay geometry must be verified after resizing while overlays are open.

## Required Test Shape

- Add a Composer UI automation test that starts a drag inside `composer.splitter.left` and asserts `left_panel_w` changes during the drag.
- Drag far left and far right and assert the width clamps to the supported range.
- Release the pointer and assert `state.drag` and `drag_start_pos` are cleared.
- Open a render queue or context overlay, resize the left panel, and assert overlay selectors remain present with recalculated bounds.

## Required Changes

- Move splitter hit-zone detection ahead of generic click dispatch for primary pointer down, or add an explicit priority check that prevents overlapping click regions from stealing splitter drags.
- Expose enough state or automation values to assert left panel width and active drag state.
- Add layout assertions for overlay bounds after left splitter resize.

## Verification

- `cargo test -p tench-composer --test plan_ui_e2e left_panel_splitter`
- `cargo test -p tench-composer`
