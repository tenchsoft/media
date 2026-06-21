# Templates Left Panel Tab State

## Source Plan

- `plans/composer/templates-left-panel-tab-button-work-plan.md`

## Gap Analysis

The Templates tab implementation registers `SelectLeftTab(Templates)` and renders content from `state.left_tab`, but current E2E coverage only clicks Templates once and asserts a template row is present. It does not verify active tab state, highlighted styling, idempotence, or unrelated state preservation. See `apps/composer/src-tauri/src/ui/left_panel.rs:41`, `apps/composer/src-tauri/src/ui/state.rs:546`, and `apps/composer/src-tauri/tests/plan_ui_e2e.rs:204`.

The tests do not cover clicking Templates after importing media or after selecting templates, and they do not verify that timeline selection, playhead, mode, and export settings remain unchanged.

The compact-width tab hit region scenario is untested.

## Plan Requirements Not Met

- Clicking Templates from another left tab must be covered by UI automation and must assert that only left-panel content changes.
- Clicking Templates while it is already active must be tested for idempotence.
- Clicking Templates after import/template interactions must be tested to preserve existing project data.
- Compact-width Templates tab hit-region behavior must be tested.

## Required Test Shape

- Add a Composer UI automation test that clicks another left tab, then clicks `composer.left.templates`, and asserts template rows are visible while mode, playhead, selected clip, and export settings are unchanged.
- Click `composer.left.templates` while Templates is already active and assert no unrelated state changes.
- Import a media item, select a template, return to Templates, and assert both imported media and selected template/project data remain intact.
- Resize the left panel to a compact width, click the Templates tab's bounds, and assert only Templates is selected.

## Required Changes

- Add the missing Templates tab scenario tests.
- Expose active left tab, playhead, selected clip, mode, and export settings through automation if current state access is insufficient.
- Adjust compact-width tab hit regions only if the new test exposes overlap.

## Verification

- `cargo test -p tench-composer --test plan_ui_e2e templates_left_tab`
- `cargo test -p tench-composer`
