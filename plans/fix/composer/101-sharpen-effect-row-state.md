# Sharpen Effect Row State

## Source Plan

- `plans/composer/sharpen-effect-row-button-work-plan.md`

## Gap Analysis

Effect rows register concrete `ApplyEffect(effect)` actions, but clicking Sharpen with no selected clip only sets a notice and does not arm Sharpen as an active apply or drag source. See `apps/composer/src-tauri/src/ui/left_panel.rs:361` and `apps/composer/src-tauri/src/ui/mod.rs:235`.

`DragKind::Effect` exists, but no effect row path starts that drag state, so Sharpen cannot be dragged from the effects list onto a timeline clip. See `apps/composer/src-tauri/src/ui/state.rs:195`.

The selected-clip apply path pushes undo, creates an effect, appends the effect id, and sets a notice, but current E2E coverage only filters and clicks Blur. Sharpen is only checked for selector presence; it is not applied, dragged, or verified after filtering. See `apps/composer/src-tauri/src/ui/state.rs:899` and `apps/composer/src-tauri/tests/plan_ui_e2e.rs:209`.

## Plan Requirements Not Met

- Clicking Sharpen with no selected clip must arm a Sharpen apply/drop source or expose clear selectable state without mutating clips.
- Dragging Sharpen onto a timeline clip must append a Sharpen effect id to the target clip.
- Sharpen application must update inspector or clip visual state in a testable way.
- Filtered Sharpen row clicks must be tested to map to `VideoEffectType::Sharpen`.

## Required Test Shape

- Add a Composer UI automation test that clicks `composer.effect.sharpen` with no selected clip and asserts no clip mutation plus the required armed/feedback state.
- Select a clip, click `composer.effect.sharpen`, and assert an added effect with type Sharpen is attached to that clip.
- Drag `composer.effect.sharpen` onto a timeline clip and assert the clip receives the Sharpen effect.
- Filter effects to Sharpen, click the filtered row, and assert the original Sharpen effect type is applied rather than a filtered index.
- Assert undo removes the Sharpen effect.

## Required Changes

- Add selected/armed effect state or start `DragKind::Effect { effect_type: VideoEffectType::Sharpen }` from the Sharpen row.
- Add effect drop handling from the effects list onto timeline clips.
- Expose effect type attachments and selected/armed effect state through automation.
- Add Sharpen-specific UI automation coverage.

## Verification

- `cargo test -p tench-composer --test plan_ui_e2e sharpen_effect`
- `cargo test -p tench-composer`
