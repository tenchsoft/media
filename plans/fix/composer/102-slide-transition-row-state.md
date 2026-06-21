# Slide Transition Row State

## Source Plan

- `plans/composer/slide-transition-row-button-work-plan.md`

## Gap Analysis

Clicking a slide transition row only sets a notice and does not arm a transition drag/apply source. The transition rows register `ApplyTransition(t)`, but the handler does not write drag state or assignment state. See `apps/composer/src-tauri/src/ui/left_panel.rs:443` and `apps/composer/src-tauri/src/ui/mod.rs:243`.

`DragKind::Transition` exists, and clips have `transition_in` / `transition_out` fields, but the slide row path does not use them to target a clip edge or update core timeline data. See `apps/composer/src-tauri/src/ui/state.rs:197` and `crates/composer-core/src/timeline.rs:97`.

Slide rows are not uniquely addressable by transition type in automation. `SlideLeft` and `SlideRight` both map to `composer.transition.slide`, so filtered-row clicks cannot prove that the correct slide type is referenced. See `apps/composer/src-tauri/src/ui/mod.rs:1435`.

The current E2E coverage asserts that `composer.transition.slide` exists, but it filters for `wipe` and clicks `composer.transition.wipe`. It does not drag slide to a clip in-edge or out-edge, and it does not assert transition overlays or timeline state. See `apps/composer/src-tauri/tests/plan_ui_e2e.rs:231`.

## Plan Requirements Not Met

- Clicking the slide transition row with a selected clip must arm a transition apply source instead of only showing a notice.
- Dragging slide to a clip in-edge must update `transition_in` and repaint the timeline overlay.
- Dragging slide to a clip out-edge must update `transition_out` and repaint the timeline overlay.
- Filtered slide rows must retain distinct transition identity, including `SlideLeft` versus `SlideRight` where both are exposed.
- Transition assignment must push undo state.

## Required Test Shape

- Add a Composer UI automation test that selects a clip, clicks a slide transition row, and asserts the transition apply state is armed.
- Drag slide to a clip in-edge and assert the clip's `transition_in` is set and the overlay is present.
- Drag slide to a clip out-edge and assert the clip's `transition_out` is set and the overlay is present.
- Filter transitions to a slide-specific query, click the visible row, and assert the assigned transition type matches the selected slide row.
- Assert undo restores the previous clip transition state.

## Required Changes

- Give each concrete transition row a stable, distinct selector when multiple `TransitionType` variants share a broad family label.
- Route slide row activation into `DragKind::Transition` or an equivalent armed transition state.
- Add edge hit testing for transition drops and update `transition_in` / `transition_out` through core timeline structures.
- Push undo and repaint the timeline after transition assignment.

## Verification

- `cargo test -p tench-composer --test plan_ui_e2e slide_transition`
- `cargo test -p tench-composer`
