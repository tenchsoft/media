# Media Left Panel Tab State

## Source Plan

- `plans/composer/media-left-panel-tab-button-work-plan.md`

## Gap Analysis

The Media tab implementation registers `SelectLeftTab(Media)` and renders content from `state.left_tab`, but the current E2E coverage only asserts that `composer.left.media` exists. The left-panel flow clicks Templates, Effects, and Transitions, but it never returns to Media or asserts Media-tab idempotence. See `apps/composer/src-tauri/src/ui/left_panel.rs:41`, `apps/composer/src-tauri/src/ui/state.rs:546`, and `apps/composer/src-tauri/tests/plan_ui_e2e.rs:200`.

The tests do not cover clicking Media after importing media or selecting a template, and they do not verify that timeline selection, playhead, mode, and export settings remain unchanged.

The compact-width tab hit region scenario is untested.

## Plan Requirements Not Met

- Clicking Media from another left tab must be covered by UI automation and must assert that only left-panel content changes.
- Clicking Media while it is already active must be tested for idempotence.
- Clicking Media after import/template interactions must be tested to preserve existing project data.
- Compact-width Media tab hit-region behavior must be tested.

## Required Test Shape

- Add a Composer UI automation test that clicks Templates, then clicks `composer.left.media`, and asserts media-bin content is visible while mode, playhead, selected clip, and export settings are unchanged.
- Click `composer.left.media` while Media is already active and assert no unrelated state changes.
- Import a media item, select a template, return to Media, and assert both imported media and template/project data remain intact.
- Resize the left panel to a compact width, click the Media tab's bounds, and assert only Media is selected.

## Required Changes

- Add the missing Media tab scenario tests.
- Expose any missing automation state needed to assert playhead, selected clip, mode, export settings, and active left tab.
- Adjust compact-width tab hit regions only if the new test exposes overlap.

## Verification

- `cargo test -p tench-composer --test plan_ui_e2e media_left_tab`
- `cargo test -p tench-composer`
