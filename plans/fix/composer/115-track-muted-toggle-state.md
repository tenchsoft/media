# Track Muted Toggle State

## Source Plan

- `plans/composer/track-muted-toggle-control-work-plan.md`

## Gap Analysis

The Audio inspector renders volume, pan, and muted controls from `selected_track()`, but the current E2E coverage only asserts the muted selector exists. It clicks volume and pan, and never clicks `composer.track.muted` or asserts the resulting track state. See `apps/composer/src-tauri/src/ui/right_panel.rs:214`, `apps/composer/src-tauri/src/ui/right_panel.rs:303`, and `apps/composer/src-tauri/tests/plan_ui_e2e.rs:286`.

Volume and pan values are clamped when the UI constructs click actions, but the action handlers write incoming values directly. Programmatic or stale actions can bypass the supported range. See `apps/composer/src-tauri/src/ui/right_panel.rs:261`, `apps/composer/src-tauri/src/ui/right_panel.rs:299`, and `apps/composer/src-tauri/src/ui/mod.rs:389`.

The no-selected-track empty state and tab-switch synchronization with the track header mute button are untested.

## Plan Requirements Not Met

- Activating `composer.track.muted` must be tested to toggle the selected track's muted state and update the inspector value.
- No-selected-track behavior must be tested to preserve empty state and avoid mutation.
- Volume and pan handlers must clamp incoming values, not only UI-generated increments.
- Switching tabs after toggling mute must be tested to keep the track header M button synchronized.
- Tests must assert project track data, not only capture changes.

## Required Test Shape

- Add a Composer UI automation test that opens the Audio inspector, clicks `composer.track.muted`, and asserts the selected track's `muted` field plus visible value update.
- Clear clip/track selection, open the Audio inspector, and assert the empty state remains and no track mutates.
- Dispatch or simulate out-of-range volume and pan actions and assert values clamp to supported min/max.
- Toggle mute in the Audio inspector, switch back to the timeline/header view, and assert `composer.track.mute` reflects the same state.

## Required Changes

- Clamp `SetTrackVolume` and `SetTrackPan` in the dispatch handlers.
- Add muted-toggle E2E coverage for selected track, empty state, tab synchronization, and data assertions.
- Expose track audio values and muted visual state through automation if current state access is insufficient.

## Verification

- `cargo test -p tench-composer --test plan_ui_e2e track_muted_toggle`
- `cargo test -p tench-composer`
