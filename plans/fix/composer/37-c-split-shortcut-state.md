# C Split Shortcut State Fix Plan

## Source Plan

- `plans/composer/c-split-shortcut-control-work-plan.md`

## Gap Analysis

The `C` shortcut and Split toolbar button both call `split_at_playhead`, but the
shortcut does not guard modifier combinations and missing-target failure is
silent. Existing tests do not press `C` or click Split to assert the clip graph
changes.

## Plan Requirements Not Met

- Ctrl+C and Alt+C can reach the global split shortcut because the `C` handler
  has no modifier guard.
- Ctrl+C while the subtitle editor is focused can bypass text-editing behavior
  and invoke global split.
- Pressing `C` with no selected clip returns `false` without an actionable
  notice.
- The Split toolbar button is visually disabled with no selected clip, but its
  click region is still registered.
- There is no E2E test that pressing `C` splits the selected clip through the
  same state path as the Split toolbar button.
- There is no test that `C` with no selected clip shows a no-op message and
  leaves timeline data unchanged.
- There is no test that repeated `C` presses at the same frame remain
  deterministic and report invalid boundary attempts clearly.

## Code Review

- `apps/composer/src-tauri/src/ui/mod.rs:208` routes the Split toolbar action to
  `split_at_playhead`.
- `apps/composer/src-tauri/src/ui/mod.rs:999` routes `c == "c"` directly to
  `split_at_playhead` without modifier checks.
- `apps/composer/src-tauri/src/ui/state.rs:756` returns `false` immediately when
  no clip is selected.
- `apps/composer/src-tauri/src/ui/state.rs:780` reports timeline split errors
  as notices, but the missing-selection branch has no notice.
- `apps/composer/src-tauri/src/ui/toolbar.rs:84` computes disabled styling for
  Split when no clip is selected.
- `apps/composer/src-tauri/src/ui/toolbar.rs:117` registers the Split click
  region regardless of that disabled state.

## Test Review

- `apps/composer/src-tauri/tests/plan_ui_e2e.rs:81` only asserts
  `composer.toolbar.split` is present.
- Existing E2E coverage selects a clip for inspector tests, but does not press
  `C`, click Split, compare clip counts, verify selection of the right-side
  split clip, or test no-target behavior.

## Required Test Shape

- Select a clip, set the playhead inside its bounds, press `C`, and assert the
  track now has two clips with valid ranges and the right clip selected.
- Repeat the same split through `composer.toolbar.split` and assert equivalent
  state changes.
- Clear selection, press `C`, and assert no timeline mutation plus an actionable
  notice.
- Press Ctrl+C and Alt+C and assert no split occurs.
- Focus the subtitle editor, press plain `c`, and assert text editing takes
  precedence while clip state remains unchanged.
- Press `C` repeatedly at an invalid boundary and assert deterministic no-op
  state and notice text.

## Required Changes

- Add a shared split command wrapper that maps `split_at_playhead` results to
  user-facing notices for success, missing target, and invalid boundary cases.
- Use that wrapper from both the Split toolbar button and the `C` shortcut.
- Do not dispatch disabled toolbar click regions, or make disabled click
  dispatch produce the same no-op result.
- Reject disallowed modifiers before invoking the global `C` shortcut.

## Verification

- `cargo test -p tench-composer c_split_shortcut`
- `cargo test -p tench-composer composer_plan_inspector_render_queue_ai_and_context_menu_use_real_events_ui_e2e`
- `git diff --check`
