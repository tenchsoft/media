# Track Header Mute Button State

## Source Plan

- `plans/composer/track-header-mute-button-work-plan.md`

## Gap Analysis

The M button toggles `track.muted` and renders header button/text styling from that boolean, but current E2E coverage only asserts the boolean changed after one click. It does not verify active styling, click-again behavior, multi-track isolation, or Audio inspector synchronization. See `apps/composer/src-tauri/src/ui/timeline_panel.rs:169`, `apps/composer/src-tauri/src/ui/mod.rs:299`, and `apps/composer/src-tauri/tests/plan_ui_e2e.rs:156`.

The Audio inspector reads the same `track.muted` state, but tests only assert the muted control exists and do not verify it stays synchronized after toggling the track header M button. See `apps/composer/src-tauri/src/ui/right_panel.rs:303` and `apps/composer/src-tauri/tests/plan_ui_e2e.rs:286`.

The muted-track audio/output rule is not covered by tests.

## Plan Requirements Not Met

- Clicking M again must be tested to restore `track.muted == false`.
- Muted and unmuted header styling must be asserted through automation.
- Muting one track among multiple tracks must be tested to leave other tracks unchanged.
- The Audio inspector muted value must be tested to stay synchronized with the track header button.
- Muted-track audio/output behavior must be defined and tested if mute affects playback or export.

## Required Test Shape

- Add a Composer UI automation test that clicks `composer.track.mute`, asserts `track.muted == true`, and asserts the M button active state.
- Click M again and assert `track.muted == false` plus inactive styling.
- Add a second track, mute one track, and assert the other track remains unmuted.
- Open the Audio inspector, toggle the header M button, and assert `composer.track.muted` shows the same state.
- If mute affects playback/export, verify muted output behavior in the relevant render or preview state.

## Required Changes

- Add the missing track mute scenario tests.
- Expose track mute active styling and Audio inspector muted value through automation if current selectors are insufficient.
- Define and apply muted-track output behavior where playback/export consumes track audio.

## Verification

- `cargo test -p tench-composer --test plan_ui_e2e track_mute`
- `cargo test -p tench-composer`
