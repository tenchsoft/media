# Effects Left Panel Tab State Fix Plan

## Source Plan

- `plans/composer/effects-left-panel-tab-button-work-plan.md`

## Gap Analysis

Effects tab click is covered enough to show the effects list, but the test does
not verify `left_tab` state, active styling, project-data invariants, or compact
panel hit behavior.

## Plan Requirements Not Met

- There is no assertion that clicking `composer.left.effects` sets
  `left_tab == LeftPanelTab::Effects`.
- There is no test that only left panel content changes after switching to
  Effects.
- There is no test that clicking Effects while already active preserves
  selection, playhead, mode, timeline, and export settings.
- There is no test that imported media and selected template data remain intact
  after clicking Effects.
- There is no compact panel width test proving the Effects tab hit region
  selects only that tab.
- Left tab hit regions use fixed widths and spacing without compact layout
  adaptation.
- Automation does not expose active left tab value or per-tab active state.

## Code Review

- `apps/composer/src-tauri/src/ui/left_panel.rs:44` creates fixed-width left tab
  hit regions.
- `apps/composer/src-tauri/src/ui/left_panel.rs:45` renders active styling from
  `state.left_tab`.
- `apps/composer/src-tauri/src/ui/left_panel.rs:64` registers each tab with
  `ClickAction::SelectLeftTab(tab)`.
- `apps/composer/src-tauri/src/ui/left_panel.rs:76` renders the effects list
  when `left_tab == Effects`.
- `apps/composer/src-tauri/src/ui/mod.rs:216` dispatches left tab clicks to
  `select_left_tab`.
- `apps/composer/src-tauri/src/ui/state.rs:546` stores active left tab in a
  single field.

## Test Review

- `apps/composer/src-tauri/tests/plan_ui_e2e.rs:209` clicks
  `composer.left.effects`.
- `apps/composer/src-tauri/tests/plan_ui_e2e.rs:211` through `:220` assert
  effects controls are present.
- Existing E2E does not assert `left_tab`, active styling, non-left-panel
  invariants, or compact panel behavior.

## Required Test Shape

- Click Effects from Media/Templates and assert `left_tab`,
  `composer.effects.search`, and effect rows.
- Snapshot selected clip, playhead, mode, timeline, media bin, templates, and
  export settings before and after switching to Effects.
- Click Effects again while active and assert no project or navigation state
  changes.
- Set a compact `left_panel_w`, click the Effects tab by selector, and assert it
  selects Effects without hitting adjacent tabs.

## Required Changes

- Expose active left tab and per-tab selected state through automation.
- Add compact-tab layout or overflow handling if fixed tab rectangles overlap
  or extend beyond the usable left panel width.
- Keep `select_left_tab` free of timeline, selection, playhead, mode, and export
  mutations.

## Verification

- `cargo test -p tench-composer effects_left_panel_tab`
- `cargo test -p tench-composer composer_plan_left_panel_templates_effects_transitions_and_subtitles_use_real_events_ui_e2e`
- `cargo test -p tench-ui-test ui_automation`
- `git diff --check`
