# Timeline Splitter Drag State

## Source Plan

- `plans/composer/timeline-splitter-drag-control-work-plan.md`

## Gap Analysis

Primary pointer down dispatches registered click regions before it checks splitter hit zones. The timeline splitter works when no click region overlaps it, but the splitter is not guaranteed to win before click handling as required. See `apps/composer/src-tauri/src/ui/mod.rs:805` and `apps/composer/src-tauri/src/ui/mod.rs:811`.

The timeline splitter has an automation node and clamp logic, but current E2E coverage only asserts the selector exists. There is no timeline-splitter drag test for height changes, vertical clamp behavior, release cleanup, or overlay recalculation. See `apps/composer/src-tauri/src/ui/mod.rs:455`, `apps/composer/src-tauri/src/ui/mod.rs:554`, and `apps/composer/src-tauri/src/ui/mod.rs:1231`.

## Plan Requirements Not Met

- Splitter hit zones must be prioritized before generic click handling when a pointer down starts inside the timeline splitter zone.
- Timeline splitter drag tests must assert the actual `timeline_h` change.
- Dragging beyond the minimum and maximum bounds must be tested for clamping without panel overlap.
- Pointer release must be tested to clear splitter drag state and keep the final layout stable.
- Overlay geometry must be verified after resizing while overlays are open.

## Required Test Shape

- Add a Composer UI automation test that starts a drag inside `composer.splitter.timeline` and asserts `timeline_h` changes during the drag.
- Drag far up and far down and assert the height clamps to the supported range.
- Release the pointer and assert `state.drag` and `drag_start_pos` are cleared.
- Open a render queue or AI overlay, resize the timeline, and assert overlay selectors remain present with recalculated bounds.

## Required Changes

- Move splitter hit-zone detection ahead of generic click dispatch for primary pointer down, or add an explicit priority check that prevents overlapping click regions from stealing splitter drags.
- Expose enough state or automation values to assert timeline height and active drag state.
- Add layout assertions for overlay bounds after timeline splitter resize.

## Verification

- `cargo test -p tench-composer --test plan_ui_e2e timeline_splitter`
- `cargo test -p tench-composer`
