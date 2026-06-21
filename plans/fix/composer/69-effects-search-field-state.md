# Effects Search Field State Fix Plan

## Source Plan

- `plans/composer/effects-search-field-control-work-plan.md`

## Gap Analysis

Effects search supports plain keyboard characters and label filtering, but IME
input and focused styling are missing. Current tests only prove that `blur`
remains visible after typing; they do not prove non-matching effects disappear
or that clear, uppercase, and tab-switch behavior follow a contract.

## Plan Requirements Not Met

- IME commit input is not routed to `effects_search`.
- The search field does not render focused styling from `input_focus`.
- There is no test that non-matching effects are absent after typing a partial
  name.
- There is no test that clearing search restores all effects in original order.
- There is no test that uppercase input filters case-insensitively.
- The switch-away-and-back persistence/reset contract is not documented or
  tested.
- Automation does not expose search focused state, search value, visible effect
  order, or placeholder state.

## Code Review

- `apps/composer/src-tauri/src/ui/left_panel.rs:338` renders placeholder text
  when `effects_search` is empty.
- `apps/composer/src-tauri/src/ui/left_panel.rs:348` styles the field from empty
  vs non-empty state, not focus state.
- `apps/composer/src-tauri/src/ui/left_panel.rs:358` registers
  `ClickAction::FocusEffectsSearch`.
- `apps/composer/src-tauri/src/ui/left_panel.rs:361` lowercases the query.
- `apps/composer/src-tauri/src/ui/left_panel.rs:363` filters
  `VideoEffectType::ALL` by lowercased label.
- `apps/composer/src-tauri/src/ui/mod.rs:227` sets
  `ComposerInputFocus::EffectsSearch`.
- `apps/composer/src-tauri/src/ui/mod.rs:891` routes plain keyboard characters
  to focused search fields.
- `apps/composer/src-tauri/src/ui/mod.rs:934` handles IME commit only for the
  subtitle editor path.

## Test Review

- `apps/composer/src-tauri/tests/plan_ui_e2e.rs:225` types `blur` into
  `composer.effects.search`.
- `apps/composer/src-tauri/tests/plan_ui_e2e.rs:226` asserts
  `composer.effect.blur` remains present.
- Existing tests do not assert non-matching effects are absent, search value,
  clear behavior, uppercase input, IME input, focus styling, or tab-switch
  behavior.

## Required Test Shape

- Focus Effects search, type `blu`, and assert only matching effects remain
  visible.
- Clear the search with Backspace or a dedicated clear action and assert all
  effects return in `VideoEffectType::ALL` order.
- Type uppercase `BLUR` and assert filtering is case-insensitive.
- Send an IME commit while Effects search is focused and assert
  `effects_search` updates.
- Switch away from Effects and back, then assert the documented persistence or
  reset behavior.
- Assert focused and placeholder automation values before and after focus/text.

## Required Changes

- Route IME commits to `effects_search` when `input_focus == EffectsSearch`.
- Render search focused styling from `input_focus`.
- Expose search value, focus state, placeholder state, and visible effect ids
  through automation.
- Document whether Effects search persists across left-tab switches.

## Verification

- `cargo test -p tench-composer effects_search_field`
- `cargo test -p tench-composer composer_plan_left_panel_templates_effects_transitions_and_subtitles_use_real_events_ui_e2e`
- `cargo test -p tench-ui-test ui_automation`
- `git diff --check`
