# Contrast Effect Row State Fix Plan

## Source Plan

- `plans/composer/contrast-effect-row-button-work-plan.md`

## Gap Analysis

The Contrast row uses the shared typed effect click path, but it is not an
active drag/apply source and cannot be dropped onto a clip. Existing E2E only
checks that the Contrast selector is present; it never applies Contrast or
asserts project state.

## Plan Requirements Not Met

- Clicking Contrast without a selected clip does not arm Contrast as the active
  effect/apply source for a later drop.
- Dragging Contrast onto a timeline clip is not implemented.
- `DragKind::Effect` exists but is not used by the effects list or timeline
  drop handling.
- There is no test that clicking Contrast with a selected clip creates a
  Contrast `Effect`, appends its id to the selected clip, pushes undo, and shows
  the expected notice.
- There is no test that clicking Contrast without a selected clip leaves clips
  unchanged while giving clear feedback.
- There is no test that filtering the effects list to Contrast still applies
  `VideoEffectType::Contrast` rather than a filtered-row index.
- There is no automation state for the active/armed effect row, drag source, or
  clip effect list.

## Code Review

- `apps/composer/src-tauri/src/ui/left_panel.rs:361` filters effects by label.
- `apps/composer/src-tauri/src/ui/left_panel.rs:378` registers each visible
  effect row with `ClickAction::ApplyEffect(effect)`.
- `apps/composer/src-tauri/src/ui/mod.rs:235` applies an effect only when
  `selected_clip_id` is present.
- `apps/composer/src-tauri/src/ui/mod.rs:240` only shows a selected notice when
  no clip is selected.
- `apps/composer/src-tauri/src/ui/state.rs:195` defines `DragKind::Effect`, but
  no inspected UI path sets or consumes it.
- `apps/composer/src-tauri/src/ui/state.rs:899` creates a video effect, pushes
  undo, appends the effect id to the target clip, and shows a notice.

## Test Review

- `apps/composer/src-tauri/tests/plan_ui_e2e.rs:216` asserts
  `composer.effect.contrast` is present.
- Existing E2E coverage does not click Contrast, assert `effect_ids`, assert the
  created effect kind, or test filtered identity.

## Required Test Shape

- Select a clip, click `composer.effect.contrast`, and assert the project
  contains a Contrast effect id attached to that clip.
- Assert undo stack length changes and the notice says `Contrast added`.
- Clear clip selection, click Contrast, and assert no clip effect ids mutate
  while the UI exposes an armed/selected effect state or clear no-target notice.
- Filter the list to `contrast`, click the visible row, and assert the applied
  effect kind is `VideoEffectType::Contrast`.
- Drag the Contrast row onto a specific timeline clip and assert only that clip
  receives the effect.

## Required Changes

- Add active effect source state or wire `DragKind::Effect` into effects-row
  pointer handling.
- Add timeline drop handling that applies the dragged effect to the target clip
  through the same undo-aware backend method.
- Expose active effect and clip effect ids through automation for selector-based
  assertions.

## Verification

- `cargo test -p tench-composer contrast_effect_row`
- `cargo test -p tench-composer composer_plan_left_panel_templates_effects_transitions_and_subtitles_use_real_events_ui_e2e`
- `cargo test -p tench-ui-test ui_automation`
- `git diff --check`
