# Clip Reversed Toggle State Fix Plan

## Source Plan

- `plans/composer/clip-reversed-toggle-control-work-plan.md`

## Gap Analysis

The reversed toggle changes the selected clip and is covered by a basic state
assertion, but it mutates project data without pushing undo. No-selection and
other-clip invariants are also untested.

## Plan Requirements Not Met

- Reversed changes do not push undo before mutation.
- Undo does not restore the previous reversed value because no undo snapshot is
  recorded for this property change.
- There is no test that only the selected clip's reversed value changes when
  multiple clips exist.
- There is no test that the control is absent or inert with no selected clip.
- There is no automation value exposing the rendered `Yes` or `No` inspector
  state after toggling.

## Code Review

- `apps/composer/src-tauri/src/ui/right_panel.rs:153` renders the Reversed
  field.
- `apps/composer/src-tauri/src/ui/right_panel.rs:155` registers
  `ClickAction::ToggleClipReversed(clip.id)`.
- `apps/composer/src-tauri/src/ui/mod.rs:383` handles the toggle action.
- `apps/composer/src-tauri/src/ui/mod.rs:384` resolves the clip id before
  mutation.
- `apps/composer/src-tauri/src/ui/mod.rs:385` mutates `clip.reversed` directly
  without `push_undo`.

## Test Review

- `apps/composer/src-tauri/tests/plan_ui_e2e.rs:278` clicks
  `composer.clip.reversed`.
- `apps/composer/src-tauri/tests/plan_ui_e2e.rs:282` asserts the selected clip's
  reversed value changed.
- Existing coverage does not assert undo behavior, other-clip invariants,
  no-selection behavior, or rendered inspector value.

## Required Test Shape

- Select a clip, toggle Reversed, then undo and assert the previous value is
  restored.
- Create multiple clips, toggle Reversed on one selected clip, and assert all
  other clips retain their previous values.
- Clear selection and assert `composer.clip.reversed` is absent or disabled and
  no hidden mutation occurs.
- Assert the inspector automation value changes between `Yes` and `No` after
  toggling.

## Required Changes

- Move reversed mutation into an undo-aware state helper.
- Expose rendered inspector field values through automation.
- Keep the toggle scoped to the captured clip id resolved from the selected
  inspector state.

## Verification

- `cargo test -p tench-composer clip_reversed_toggle`
- `cargo test -p tench-composer composer_plan_inspector_render_queue_ai_and_context_menu_use_real_events_ui_e2e`
- `cargo test -p tench-ui-test ui_automation`
- `git diff --check`
