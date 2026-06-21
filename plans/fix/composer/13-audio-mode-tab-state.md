# Audio Mode Tab State Fix Plan

## Source Plan

- `plans/composer/audio-mode-tab-button-work-plan.md`

## Gap Analysis

The top Audio mode tab is registered and can synchronize the inspector index
through `select_mode`, but the Audio mode user flow is not verified. E2E tests
only assert the selector exists.

## Plan Requirements Not Met

- E2E coverage does not click `composer.mode.audio`.
- Tests do not assert `mode == ComposerMode::Audio` and
  `active_inspector_tab == 2` after activation.
- Audio tab highlight and inspector content are not verified after clicking the
  top mode tab.
- No-selected-clip audio empty state is not tested.
- Playback continuation and muted-track representation are not tested.
- Automation does not expose active/selected state for mode tabs.

## Code Review

- `apps/composer/src-tauri/src/ui/toolbar.rs:60` registers mode tabs with
  `ClickAction::SelectMode(mode)`.
- `apps/composer/src-tauri/src/ui/state.rs:541` updates both `mode` and
  `active_inspector_tab` in `select_mode`.
- `apps/composer/src-tauri/src/ui/mod.rs:1398` exposes mode tab selectors, but
  not active state.

## Test Review

- `apps/composer/src-tauri/tests/plan_ui_e2e.rs:75` checks
  `composer.mode.audio` is present, but never clicks it.
- There is no UI test for audio controls or empty state after top mode
  activation.

## Required Test Shape

- Click `composer.mode.audio` and assert `mode`, `active_inspector_tab`, and
  inspector content all reflect Audio.
- Assert the top Audio mode node is active and the previous mode is inactive.
- Run the flow with no selected clip and verify the expected audio empty state.
- Start playback, click Audio, and assert playback/current frame continue
  unchanged.
- Mute a track, click Audio, and assert muted state remains represented.

## Required Changes

- Extend mode-tab automation with selected/active state.
- Add Audio mode E2E coverage for state sync, inspector rendering, and unrelated
  state preservation.
- Expose audio empty state and muted state through stable automation selectors.

## Verification

- `cargo test -p tench-composer audio_mode_tab`
- `cargo test -p tench-composer composer_plan_toolbar_modes_use_real_events_ui_e2e`
- `cargo test -p tench-ui-test ui_automation`
- `git diff --check`
