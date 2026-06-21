# Crop Effect Row State Fix Plan

## Source Plan

- `plans/composer/crop-effect-row-button-work-plan.md`

## Gap Analysis

The Crop row uses the shared typed effect click path, but it is not an active
drag/apply source and cannot be dropped onto a clip. Existing E2E only checks
that the Crop selector is present; it never applies Crop or asserts project
state.

## Plan Requirements Not Met

- Clicking Crop without a selected clip does not arm Crop as the active
  effect/apply source for a later drop.
- Dragging Crop onto a timeline clip is not implemented.
- `DragKind::Effect` exists but is not used by the effects list or timeline
  drop handling.
- There is no test that clicking Crop with a selected clip creates a Crop
  `Effect`, appends its id to the selected clip, pushes undo, and shows the
  expected notice.
- There is no test that clicking Crop without a selected clip leaves clips
  unchanged while giving clear feedback.
- There is no test that filtering the effects list to Crop still applies
  `VideoEffectType::Crop` rather than a filtered-row index.
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

- `apps/composer/src-tauri/tests/plan_ui_e2e.rs:218` asserts
  `composer.effect.crop` is present.
- Existing E2E coverage does not click Crop, assert `effect_ids`, assert the
  created effect kind, or test filtered identity.

## Required Test Shape

- Select a clip, click `composer.effect.crop`, and assert the project contains a
  Crop effect id attached to that clip.
- Assert undo stack length changes and the notice says `Crop added`.
- Clear clip selection, click Crop, and assert no clip effect ids mutate while
  the UI exposes an armed/selected effect state or clear no-target notice.
- Filter the list to `crop`, click the visible row, and assert the applied
  effect kind is `VideoEffectType::Crop`.
- Drag the Crop row onto a specific timeline clip and assert only that clip
  receives the effect.

## Required Changes

- Add active effect source state or wire `DragKind::Effect` into effects-row
  pointer handling.
- Add timeline drop handling that applies the dragged effect to the target clip
  through the same undo-aware backend method.
- Expose active effect and clip effect ids through automation for selector-based
  assertions.

## Verification

- `cargo test -p tench-composer crop_effect_row`
- `cargo test -p tench-composer composer_plan_left_panel_templates_effects_transitions_and_subtitles_use_real_events_ui_e2e`
- `cargo test -p tench-ui-test ui_automation`
- `git diff --check`
