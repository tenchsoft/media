# Fade Transition Row State

## Source Plan

- `plans/composer/fade-transition-row-button-work-plan.md`

## Gap Analysis

Clicking a fade transition row only sets a notice and does not arm a transition drag/apply source. The transition rows register `ApplyTransition(t)`, but the handler does not write drag state or assignment state. See `apps/composer/src-tauri/src/ui/left_panel.rs:443` and `apps/composer/src-tauri/src/ui/mod.rs:243`.

`DragKind::Transition` exists, and clips have `transition_in` / `transition_out` fields, but the fade row path does not use them to target a clip edge or update core timeline data. See `apps/composer/src-tauri/src/ui/state.rs:197` and `crates/composer-core/src/timeline.rs:97`.

Fade rows are not uniquely addressable by transition type in automation. `FadeIn` and `FadeOut` both map to `composer.transition.fade`, so filtered-row clicks cannot prove that the correct fade type is referenced. See `apps/composer/src-tauri/src/ui/mod.rs:1435`.

The current E2E coverage asserts that `composer.transition.fade` exists, but it filters for `wipe` and clicks `composer.transition.wipe`. It does not drag fade to a clip in-edge or out-edge, and it does not assert transition overlays or timeline state. See `apps/composer/src-tauri/tests/plan_ui_e2e.rs:231`.

## Plan Requirements Not Met

- Clicking the fade transition row with a selected clip must arm a transition apply source instead of only showing a notice.
- Dragging fade to a clip in-edge must update `transition_in` and repaint the timeline overlay.
- Dragging fade to a clip out-edge must update `transition_out` and repaint the timeline overlay.
- Filtered fade rows must retain distinct transition identity, including `FadeIn` versus `FadeOut` where both are exposed.
- Transition assignment must push undo state.

## Required Test Shape

- Add a Composer UI automation test that selects a clip, clicks a fade transition row, and asserts the transition apply state is armed.
- Drag fade to a clip in-edge and assert the clip's `transition_in` is set and the overlay is present.
- Drag fade to a clip out-edge and assert the clip's `transition_out` is set and the overlay is present.
- Filter transitions to a fade-specific query, click the visible row, and assert the assigned transition type matches the selected fade row.
- Assert undo restores the previous clip transition state.

## Required Changes

- Give each concrete transition row a stable, distinct selector when multiple `TransitionType` variants share a broad family label.
- Route fade row activation into `DragKind::Transition` or an equivalent armed transition state.
- Add edge hit testing for transition drops and update `transition_in` / `transition_out` through core timeline structures.
- Push undo and repaint the timeline after transition assignment.

## Verification

- `cargo test -p tench-composer --test plan_ui_e2e fade_transition`
- `cargo test -p tench-composer`
