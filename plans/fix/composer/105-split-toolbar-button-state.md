# Split Toolbar Button State

## Source Plan

- `plans/composer/split-toolbar-button-work-plan.md`

## Gap Analysis

The toolbar visually disables Split when no clip is selected, but it still registers `ClickAction::SplitAtPlayhead` for the Split button. The no-selection guard happens inside `split_at_playhead` after dispatch rather than preventing disabled activation at the toolbar action boundary. See `apps/composer/src-tauri/src/ui/toolbar.rs:84`, `apps/composer/src-tauri/src/ui/toolbar.rs:110`, and `apps/composer/src-tauri/src/ui/state.rs:757`.

The split state method resolves the selected clip, pushes undo, rolls back on core failure, selects the right clip on success, and sets notices, but the toolbar Split control is not covered by E2E. Current tests only assert `composer.toolbar.split` exists. See `apps/composer/src-tauri/src/ui/state.rs:768`, `crates/composer-core/src/timeline.rs:395`, and `apps/composer/src-tauri/tests/plan_ui_e2e.rs:81`.

The edge-split error, no-selection no-op, undo pollution, and ripple-on independence scenarios are untested.

## Plan Requirements Not Met

- Disabled Split activation must be guarded before dispatch or covered by a test proving it cannot mutate state or pollute undo.
- Splitting a selected clip at the middle must be tested to produce two clips and select the new right clip.
- Split-at-edge failure must be tested to show the backend notice and avoid zero-length clips.
- Split with no selected clip must be tested to avoid phantom mutation and undo snapshot pollution.
- Split while ripple is enabled must be tested to remain independent from ripple deletion semantics.

## Required Test Shape

- Add a Composer UI automation test that selects a clip, moves the playhead to the middle, clicks `composer.toolbar.split`, and asserts two clips exist with the right clip selected.
- Clear clip selection, click `composer.toolbar.split`, and assert no clip count, selection, or undo-stack mutation occurs.
- Set playhead to a clip edge, click Split, and assert the actionable error notice with no zero-length clip.
- Enable ripple, split a clip, and assert split behavior is unchanged by ripple mode.
- Assert the toolbar Split button reports disabled state when no clip is selected.

## Required Changes

- Avoid registering `SplitAtPlayhead` for the toolbar Split button while disabled, or add an explicit disabled-action guard before dispatch.
- Expose clip count, selected clip id, undo depth, and toolbar enabled state through automation if current state helpers are insufficient.
- Add Split toolbar E2E coverage for success, no-op, error, and ripple-on scenarios.

## Verification

- `cargo test -p tench-composer --test plan_ui_e2e split_toolbar`
- `cargo test -p tench-composer`
