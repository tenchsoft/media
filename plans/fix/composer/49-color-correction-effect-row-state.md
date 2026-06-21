# Color Correction Effect Row State Fix Plan

## Source Plan

- `plans/composer/color-correction-effect-row-button-work-plan.md`

## Gap Analysis

The Color Correction selector is an alias for `VideoEffectType::ColorBalance`,
but the visible effect label and filter identity remain `Color Balance`. The row
also shares the same missing active drag/apply-source behavior as the other
effect rows.

## Plan Requirements Not Met

- Filtering by `color correction` does not match the visible effect because the
  underlying label is `Color Balance`.
- Clicking Color Correction without a selected clip does not arm it as the
  active effect/apply source for a later drop.
- Dragging Color Correction onto a timeline clip is not implemented.
- `DragKind::Effect` exists but is not used by the effects list or timeline
  drop handling.
- There is no test that clicking `composer.effect.color_correction` with a
  selected clip creates the intended color-correction effect, appends its id,
  pushes undo, and shows a notice.
- There is no test that clicking Color Correction without a selected clip leaves
  clips unchanged while giving clear feedback.
- There is no test that filtering the effects list by either `color correction`
  or `color balance` maps to the same effect identity.
- There is no automation state for the active/armed effect row, drag source, or
  clip effect list.

## Code Review

- `crates/composer-core/src/effect.rs:49` labels `VideoEffectType::ColorBalance`
  as `Color Balance`.
- `apps/composer/src-tauri/src/ui/left_panel.rs:361` filters effects by their
  labels only.
- `apps/composer/src-tauri/src/ui/left_panel.rs:378` registers each visible
  effect row with `ClickAction::ApplyEffect(effect)`.
- `apps/composer/src-tauri/src/ui/mod.rs:1425` derives effect automation ids
  from the effect label.
- `apps/composer/src-tauri/src/ui/mod.rs:1430` adds
  `composer.effect.color_correction` as an alias for Color Balance.
- `apps/composer/src-tauri/src/ui/state.rs:195` defines `DragKind::Effect`, but
  no inspected UI path sets or consumes it.

## Test Review

- `apps/composer/src-tauri/tests/plan_ui_e2e.rs:214` asserts
  `composer.effect.color_correction` is present.
- Existing E2E does not click Color Correction, assert effect ids, assert the
  created effect kind, or test filtered identity.

## Required Test Shape

- Select a clip, click `composer.effect.color_correction`, and assert the
  project contains the intended color-correction effect id attached to that
  clip.
- Assert undo stack length changes and the notice names the effect clearly.
- Clear clip selection, click Color Correction, and assert no clip effect ids
  mutate while the UI exposes an armed/selected effect state or clear no-target
  notice.
- Filter the list by `color correction` and `color balance`, click the visible
  row, and assert both paths apply the same effect type.
- Drag the Color Correction row onto a specific timeline clip and assert only
  that clip receives the effect.

## Required Changes

- Add a user-facing alias/search term for Color Correction or rename the effect
  label to match the product terminology.
- Add active effect source state or wire `DragKind::Effect` into effects-row
  pointer handling.
- Add timeline drop handling that applies the dragged effect to the target clip
  through the same undo-aware backend method.
- Expose active effect and clip effect ids through automation for selector-based
  assertions.

## Verification

- `cargo test -p tench-composer color_correction_effect_row`
- `cargo test -p tench-composer composer_plan_left_panel_templates_effects_transitions_and_subtitles_use_real_events_ui_e2e`
- `cargo test -p tench-ui-test ui_automation`
- `git diff --check`
