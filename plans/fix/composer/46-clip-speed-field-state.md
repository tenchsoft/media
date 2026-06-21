# Clip Speed Field State Fix Plan

## Source Plan

- `plans/composer/clip-speed-field-control-work-plan.md`

## Gap Analysis

The clip speed field is not an editable numeric field. Clicking it immediately
increments speed by `0.1`, with no edit mode, confirm, cancel, validation, or
undo snapshot.

## Plan Requirements Not Met

- Activating the speed field does not enter numeric edit mode.
- There is no text-entry path for a user-provided speed value.
- There is no cancel path that preserves the old speed.
- Confirming a speed change is not implemented separately from activation.
- Speed changes do not push undo before mutation.
- Numeric validation is missing for invalid, zero, negative, non-finite, or
  out-of-range speed values.
- There is no test asserting the selected clip's speed value changes after
  clicking `composer.clip.speed`.
- There is no test that undo restores the previous speed.
- There is no test that no selected clip keeps the speed field absent/inactive
  without starting invisible edit state.

## Code Review

- `apps/composer/src-tauri/src/ui/right_panel.rs:149` renders the current clip
  speed.
- `apps/composer/src-tauri/src/ui/right_panel.rs:150` registers the field as
  `SetClipSpeed(clip.id, clip.speed + 0.1)`.
- `apps/composer/src-tauri/src/ui/mod.rs:377` handles `SetClipSpeed`.
- `apps/composer/src-tauri/src/ui/mod.rs:378` resolves the clip id before
  mutation.
- `apps/composer/src-tauri/src/ui/mod.rs:379` mutates `clip.speed` directly
  without `push_undo` or validation.
- Existing fix `plans/fix/composer/05-clip-inspector-state.md` covers the broad
  state-assertion gap, not numeric edit, cancel, undo, or validation.

## Test Review

- `apps/composer/src-tauri/tests/plan_ui_e2e.rs:272` clicks
  `composer.clip.speed`.
- `apps/composer/src-tauri/tests/plan_ui_e2e.rs:273` only asserts capture
  change, not speed value, edit state, validation, or undo.

## Required Test Shape

- Select a clip, activate `composer.clip.speed`, and assert a numeric edit state
  or editable automation node appears.
- Type a valid speed, confirm it, and assert only the selected clip speed
  changes.
- Cancel an in-progress speed edit and assert the old speed remains visible.
- Confirm invalid speed values and assert no mutation plus validation notices.
- After a confirmed speed change, run undo and assert the previous speed is
  restored.
- Clear selection and assert the speed field is absent or disabled without
  hidden edit state.

## Required Changes

- Add inspector edit state for speed draft text and commit/cancel actions.
- Move speed mutation into an undo-aware state helper that parses and validates
  numeric input.
- Expose edit mode, draft value, committed speed, and validation notice through
  automation.

## Verification

- `cargo test -p tench-composer clip_speed_field`
- `cargo test -p tench-composer composer_plan_inspector_render_queue_ai_and_context_menu_use_real_events_ui_e2e`
- `cargo test -p tench-ui-test ui_automation`
- `git diff --check`
