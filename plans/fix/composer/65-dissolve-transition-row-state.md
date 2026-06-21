# Dissolve Transition Row State Fix Plan

## Source Plan

- `plans/composer/dissolve-transition-row-button-work-plan.md`

## Gap Analysis

The Dissolve row is rendered as `CrossDissolve`, but clicking it only sets a
selected notice. There is no armed transition source, drag/drop behavior, clip
edge detection, transition id creation, or assignment to `transition_in` or
`transition_out`.

## Plan Requirements Not Met

- Clicking Dissolve with a selected clip does not arm the transition for a clip
  edge.
- Dragging Dissolve to a clip in-edge is not implemented.
- Dragging Dissolve to a clip out-edge is not implemented.
- `DragKind::Transition` exists but is not used by transition rows or timeline
  drop handling.
- No backend helper creates a transition entry and assigns its id to the target
  clip edge.
- There is no test that clicking `composer.transition.dissolve` maps to
  `TransitionType::CrossDissolve`.
- There is no test that filtered transition rows retain the correct transition
  identity for Dissolve.
- Automation does not expose active transition source, target clip edge, or
  transition_in/transition_out state.

## Code Review

- `crates/composer-core/src/transition.rs:8` defines
  `TransitionType::CrossDissolve`.
- `crates/composer-core/src/transition.rs:34` labels it `Cross Dissolve`.
- `apps/composer/src-tauri/src/ui/left_panel.rs:442` filters transitions by
  label.
- `apps/composer/src-tauri/src/ui/left_panel.rs:459` registers each transition
  row with `ClickAction::ApplyTransition(t)`.
- `apps/composer/src-tauri/src/ui/mod.rs:243` handles transition clicks by
  setting a notice only.
- `apps/composer/src-tauri/src/ui/state.rs:198` defines
  `DragKind::Transition`, but no inspected UI path sets or consumes it.

## Test Review

- `apps/composer/src-tauri/tests/plan_ui_e2e.rs:234` asserts
  `composer.transition.dissolve` is present.
- Existing E2E filters to Wipe and clicks Wipe only.
- Existing tests do not assign any transition to a clip edge or assert timeline
  overlay state.

## Required Test Shape

- Click `composer.transition.dissolve` with a selected clip and assert an armed
  transition state is exposed without blindly mutating the clip.
- Drag Dissolve to a clip in-edge and assert `transition_in` is set through an
  undo-aware backend helper.
- Drag Dissolve to a clip out-edge and assert `transition_out` is set.
- Filter transitions to `dissolve`, click the row, and assert the active
  transition type is `TransitionType::CrossDissolve`.
- Assert timeline overlay rendering and automation state after transition
  assignment.

## Required Changes

- Add active transition source state or wire `DragKind::Transition` into
  transition-row pointer handling.
- Add timeline edge drop handling that creates or reuses transition data and
  assigns the resulting id to `transition_in` or `transition_out`.
- Expose active transition, target edge, transition ids, and overlay bounds
  through automation.

## Verification

- `cargo test -p tench-composer dissolve_transition_row`
- `cargo test -p tench-composer composer_plan_left_panel_templates_effects_transitions_and_subtitles_use_real_events_ui_e2e`
- `cargo test -p tench-ui-test ui_automation`
- `git diff --check`
