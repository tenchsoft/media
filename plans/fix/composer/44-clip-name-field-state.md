# Clip Name Field State Fix Plan

## Source Plan

- `plans/composer/clip-name-field-control-work-plan.md`

## Gap Analysis

The clip name field is not an editable field. Clicking it immediately appends
`*` to the current name through `SetClipName`, with no edit mode, confirm,
cancel, validation, or undo snapshot.

## Plan Requirements Not Met

- Activating the clip name field does not enter edit mode.
- There is no text-entry path for a user-provided clip name.
- There is no cancel path that preserves the old clip name.
- Confirming a name change is not implemented separately from activation.
- Clip name changes do not push undo before mutation.
- Clip name validation is missing for empty or invalid text.
- There is no test that no selected clip keeps the name field absent/inactive
  without starting invisible edit state.
- Existing fix `plans/fix/composer/05-clip-inspector-state.md` covers only the
  broad state-assertion gap, not edit mode, cancel, undo, or validation.

## Code Review

- `apps/composer/src-tauri/src/ui/right_panel.rs:141` renders the current clip
  name.
- `apps/composer/src-tauri/src/ui/right_panel.rs:142` registers the field as
  `SetClipName(clip.id, format!("{}*", clip.name))`.
- `apps/composer/src-tauri/src/ui/mod.rs:371` handles `SetClipName`.
- `apps/composer/src-tauri/src/ui/mod.rs:372` resolves the clip id before
  mutation.
- `apps/composer/src-tauri/src/ui/mod.rs:373` mutates `clip.name` directly
  without `push_undo` or validation.

## Test Review

- `apps/composer/src-tauri/tests/plan_ui_e2e.rs:264` asserts
  `composer.clip.name` is present after selecting a clip.
- `apps/composer/src-tauri/tests/plan_ui_e2e.rs:270` clicks the name field.
- `apps/composer/src-tauri/tests/plan_ui_e2e.rs:271` only asserts capture
  change, not edited value, cancel behavior, undo, or validation.

## Required Test Shape

- Select a clip, activate `composer.clip.name`, and assert a name edit state or
  editable automation node appears.
- Type a valid new name, confirm it, and assert only the selected clip name
  changes.
- Cancel an in-progress name edit and assert the original name remains visible.
- Confirm an empty or invalid name and assert no mutation plus an actionable
  validation notice.
- After a confirmed rename, run undo and assert the previous name is restored.
- Clear selection and assert the clip name field is absent or disabled without
  hidden edit state.

## Required Changes

- Add inspector edit state for clip name draft text and commit/cancel actions.
- Move name mutation into an undo-aware state helper that validates input and
  returns a result.
- Expose edit mode, draft text, committed clip name, and validation notice
  through automation.

## Verification

- `cargo test -p tench-composer clip_name_field`
- `cargo test -p tench-composer composer_plan_inspector_render_queue_ai_and_context_menu_use_real_events_ui_e2e`
- `cargo test -p tench-ui-test ui_automation`
- `git diff --check`
