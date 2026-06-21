# Snap Timeline Toggle State

## Source Plan

- `plans/composer/snap-timeline-toggle-button-work-plan.md`

## Gap Analysis

The snap toggle flips `state.snap` and `snap_position` uses that boolean, but current E2E coverage only clicks `composer.timeline.snap` once and asserts the boolean changed plus a capture difference. It does not test toggling back off or verify that subsequent timeline operations actually use or stop using snap behavior. See `apps/composer/src-tauri/src/ui/mod.rs:294`, `apps/composer/src-tauri/src/ui/state.rs:1126`, and `apps/composer/src-tauri/tests/plan_ui_e2e.rs:140`.

The no-drag and playback-preservation scenarios are untested.

## Plan Requirements Not Met

- Turning snap on must be tested to affect a concrete editing operation such as clip move or trim.
- Turning snap off must be tested to stop applying snap behavior.
- Toggling snap while no drag is active must be tested to avoid moving clips.
- Toggling snap during playback must be tested to preserve transport state.
- Active/inactive visual state should be asserted through automation, not only a screenshot delta.

## Required Test Shape

- Add a Composer UI automation test that enables `composer.timeline.snap`, moves or trims a clip near a boundary, and asserts the resulting frame snaps to the boundary.
- Disable snap, repeat the same operation, and assert the frame does not snap.
- Toggle snap when no drag is active and assert clip positions remain unchanged.
- Start playback, click `composer.timeline.snap`, and assert transport state is unchanged.
- Assert the toggle's active/inactive state through a stable automation value or selector.

## Required Changes

- Add the missing snap toggle behavior tests.
- Expose clip positions and snap toggle active state through automation if current state access is insufficient.
- Adjust snap behavior only if the on/off operation tests expose an issue.

## Verification

- `cargo test -p tench-composer --test plan_ui_e2e snap_toggle`
- `cargo test -p tench-composer`
