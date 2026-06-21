# Zoom Transition Row State

## Source Plan

- `plans/composer/zoom-transition-row-button-work-plan.md`

## Gap Analysis

Zoom is not a `TransitionType`. It is rendered as a synthetic row that dispatches `RunAiFeature("Transition Zoom")`, so clicking it queues a generic notice rather than arming or applying a transition. See `apps/composer/src-tauri/src/ui/left_panel.rs:462`, `apps/composer/src-tauri/src/ui/mod.rs:247`, and `crates/composer-core/src/transition.rs:7`.

Because Zoom is not represented in the transition model, it cannot be stored in `transition_in` or `transition_out`, dragged to a clip edge, or restored through undo as a normal transition assignment.

The current E2E coverage asserts that `composer.transition.zoom` exists, but it filters and clicks Wipe instead. Zoom is not clicked, dragged, or verified against timeline state. See `apps/composer/src-tauri/tests/plan_ui_e2e.rs:231`.

## Plan Requirements Not Met

- Zoom must be represented as a real transition type or a clearly supported transition workflow that can target clip edges.
- Clicking the zoom transition row with a selected clip must arm a transition apply source instead of running a generic AI feature action.
- Dragging zoom to a clip in-edge must update `transition_in` and repaint the timeline overlay.
- Dragging zoom to a clip out-edge must update `transition_out` and repaint the timeline overlay.
- Filtered zoom row clicks must retain stable transition identity.
- Transition assignment must push undo state.

## Required Test Shape

- Add a Composer UI automation test that selects a clip, clicks `composer.transition.zoom`, and asserts the transition apply state is armed.
- Drag zoom to a clip in-edge and assert the clip's `transition_in` is set and the overlay is present.
- Drag zoom to a clip out-edge and assert the clip's `transition_out` is set and the overlay is present.
- Filter transitions to `zoom`, click the visible row, and assert the assigned transition type or workflow id is Zoom.
- Assert undo restores the previous clip transition state.

## Required Changes

- Add Zoom to the transition model, or define an equivalent non-AI transition workflow that supports clip-edge assignment.
- Stop routing `composer.transition.zoom` through generic `RunAiFeature`.
- Route zoom row activation into `DragKind::Transition` or an equivalent armed transition state.
- Add edge hit testing for zoom drops and update `transition_in` / `transition_out` through core timeline structures.
- Push undo and repaint the timeline after transition assignment.

## Verification

- `cargo test -p tench-composer --test plan_ui_e2e zoom_transition`
- `cargo test -p tench-composer`
