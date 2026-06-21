# Transitions Search Field State

## Source Plan

- `plans/composer/transitions-search-field-control-work-plan.md`

## Gap Analysis

Transitions search routes keyboard character input, Escape, and Backspace, but IME commit events are not appended to `transitions_search`. See `apps/composer/src-tauri/src/ui/mod.rs:865`.

The search field renders placeholder and value text, but it does not render a focused state from `input_focus == TransitionsSearch`; the border is always drawn with the default border color. See `apps/composer/src-tauri/src/ui/left_panel.rs:416`.

Switching left tabs does not clear or reconcile search input focus, so typing after a tab switch can keep targeting the previous search field until another focus action occurs. See `apps/composer/src-tauri/src/ui/mod.rs:231` and `apps/composer/src-tauri/src/ui/state.rs:546`.

The current E2E coverage types `"wipe"` and asserts Wipe remains, but it does not assert non-matching transitions disappear, clear the search, test mixed case, test IME input, or verify effects search remains independent. See `apps/composer/src-tauri/tests/plan_ui_e2e.rs:242`.

## Plan Requirements Not Met

- IME commit input must route to `transitions_search` while the transitions search field is focused.
- The search field must render a focused state independently from effects search.
- Clearing the search must be tested to restore all transitions.
- Mixed-case filtering must be tested to remain case-insensitive.
- Tab-switch persistence and no-leak behavior between transitions and effects search must be defined and tested.

## Required Test Shape

- Add a Composer UI automation test that focuses `composer.transitions.search`, types a partial transition name, and asserts matching rows are present while non-matching rows are absent.
- Press Backspace or clear the field and assert all transition rows return.
- Type mixed-case text such as `WiPe` and assert filtering remains case-insensitive.
- Dispatch an IME commit while focused and assert `transitions_search` updates.
- Switch to Effects and back, then assert the chosen persistence contract and that effects search text is not modified by transitions input.

## Required Changes

- Route IME commit events into `transitions_search` when `input_focus == TransitionsSearch`.
- Render focused styling for the transitions search field from `input_focus`.
- Clear or preserve search focus on tab switches according to an explicit contract.
- Add absent-selector assertions for filtered-out transitions.

## Verification

- `cargo test -p tench-composer --test plan_ui_e2e transitions_search`
- `cargo test -p tench-composer`
