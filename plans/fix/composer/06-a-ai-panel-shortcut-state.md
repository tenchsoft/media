# A AI Panel Shortcut State Fix Plan

## Source Plan

- `plans/composer/a-ai-panel-shortcut-control-work-plan.md`

## Gap Analysis

The `A` key toggles the Composer AI panel, but it does not fully satisfy the
shortcut contract because modifier handling, shared state routing, notices, and
text-focus precedence are incomplete.

## Plan Requirements Not Met

- The `A` shortcut does not guard modifier keys, so modified combinations such
  as Ctrl+A can still toggle the AI panel.
- The shortcut and `composer.quick.ai` button both mutate `show_ai_panel`
  directly instead of sharing one state method.
- Toggling the AI panel does not set a user-visible notice.
- Text-focus precedence is incomplete for modified keypresses while subtitle or
  search fields are focused.
- Tests do not verify open/close state, repeated shortcut behavior, modifier
  suppression, or subtitle-editor precedence.
- Automation does not expose whether `composer.quick.ai` or the AI panel is
  open or closed.

## Code Review

- `apps/composer/src-tauri/src/ui/mod.rs:1053` toggles `show_ai_panel` for
  `LogicalKey::Character("a")` without checking control, shift, or alt.
- `apps/composer/src-tauri/src/ui/mod.rs:264` toggles `show_ai_panel` directly
  for `ClickAction::ToggleAiPanel`.
- `apps/composer/src-tauri/src/ui/mod.rs:911` gives subtitle text input
  precedence for plain characters, but modified character keys can fall through
  to global shortcuts.
- `apps/composer/src-tauri/src/ui/timeline_panel.rs:639` registers the visible
  AI button, but the click action does not route through a shared toggle helper.

## Test Review

- `apps/composer/src-tauri/tests/plan_ui_e2e.rs:325` clicks
  `composer.quick.ai` and asserts AI feature buttons, but does not assert state
  or close behavior.
- `apps/composer/src-tauri/tests/plan_ui_e2e.rs:377` includes `a` in a shortcut
  smoke loop and only asserts the capture changed.
- There is no test for Ctrl+A/Alt+A/Shift+A suppression, subtitle-editor focus,
  or rapid repeated `A` presses.

## Required Test Shape

- Add an E2E test that presses `A`, asserts the AI panel opens, presses `A`
  again, and asserts it closes.
- Click `composer.quick.ai` and assert it uses the same state transition and
  notice behavior as the shortcut.
- Focus `composer.subtitle.editor`, press plain `a`, and assert the subtitle text
  changes while `show_ai_panel` remains unchanged.
- Press modified `A` combinations and assert they do not trigger the plain
  shortcut unless explicitly assigned.
- Repeat `A` rapidly and assert the final open/closed state is deterministic.

## Required Changes

- Add a shared `toggle_ai_panel` state method that updates `show_ai_panel`,
  clears or preserves conflicting panel state by policy, and sets a notice.
- Route both `ClickAction::ToggleAiPanel` and the `A` shortcut through that
  method.
- Require the plain `A` shortcut to have no control, alt, or command modifier.
  Decide whether shift should be accepted and test that decision.
- Ensure focused text inputs consume relevant text and modified keypresses before
  global shortcuts run.
- Expose AI panel open/closed state through automation.

## Verification

- `cargo test -p tench-composer a_ai_panel_shortcut`
- `cargo test -p tench-composer composer_plan_inspector_render_queue_ai_and_context_menu_use_real_events_ui_e2e`
- `cargo test -p tench-ui-test ui_automation`
- `git diff --check`
