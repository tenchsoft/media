# Right Panel Splitter Drag State

## Source Plan

- `plans/composer/right-panel-splitter-drag-control-work-plan.md`

## Gap Analysis

Primary pointer down dispatches registered click regions before it checks splitter hit zones. The right splitter works when no click region overlaps it, but the splitter is not guaranteed to win before click handling as required. See `apps/composer/src-tauri/src/ui/mod.rs:805` and `apps/composer/src-tauri/src/ui/mod.rs:811`.

The right splitter has an automation node and clamp logic, but current E2E coverage only asserts the selector exists. There is no right-panel drag test for width changes, clamp behavior, release cleanup, or overlay recalculation. See `apps/composer/src-tauri/src/ui/mod.rs:450`, `apps/composer/src-tauri/src/ui/mod.rs:551`, and `apps/composer/src-tauri/src/ui/mod.rs:1222`.

## Plan Requirements Not Met

- Splitter hit zones must be prioritized before generic click handling when a pointer down starts inside the right splitter zone.
- Right splitter drag tests must assert the actual `right_panel_w` change.
- Dragging beyond the minimum and maximum bounds must be tested for clamping without panel overlap.
- Pointer release must be tested to clear splitter drag state and keep the final layout stable.
- Overlay geometry must be verified after resizing while overlays are open.

## Required Test Shape

- Add a Composer UI automation test that starts a drag inside `composer.splitter.right` and asserts `right_panel_w` changes during the drag.
- Drag far left and far right and assert the width clamps to the supported range.
- Release the pointer and assert `state.drag` and `drag_start_pos` are cleared.
- Open a render queue or AI overlay, resize the right panel, and assert overlay selectors remain present with recalculated bounds.

## Required Changes

- Move splitter hit-zone detection ahead of generic click dispatch for primary pointer down, or add an explicit priority check that prevents overlapping click regions from stealing splitter drags.
- Expose enough state or automation values to assert right panel width and active drag state.
- Add layout assertions for overlay bounds after right splitter resize.

## Verification

- `cargo test -p tench-composer --test plan_ui_e2e right_panel_splitter`
- `cargo test -p tench-composer`
