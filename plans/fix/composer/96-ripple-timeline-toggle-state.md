# Ripple Timeline Toggle State

## Source Plan

- `plans/composer/ripple-timeline-toggle-button-work-plan.md`

## Gap Analysis

The ripple toggle flips `state.ripple` and the delete path uses that boolean, but current E2E coverage only clicks `composer.timeline.ripple` once and asserts the boolean changed plus a capture difference. It does not test toggling back off or verify that subsequent timeline operations actually use or stop using ripple behavior. See `apps/composer/src-tauri/src/ui/mod.rs:294`, `apps/composer/src-tauri/src/ui/state.rs:802`, and `apps/composer/src-tauri/tests/plan_ui_e2e.rs:145`.

The no-drag and playback-preservation scenarios are untested.

## Plan Requirements Not Met

- Turning ripple on must be tested to affect a concrete editing operation such as delete.
- Turning ripple off must be tested to stop applying ripple behavior.
- Toggling ripple while no drag is active must be tested to avoid moving clips.
- Toggling ripple during playback must be tested to preserve transport state.
- Active/inactive visual state should be asserted through automation, not only a screenshot delta.

## Required Test Shape

- Add a Composer UI automation test that enables `composer.timeline.ripple`, deletes a selected clip, and asserts later clips close the gap.
- Disable ripple, repeat a delete operation, and assert later clips do not close the gap.
- Toggle ripple when no drag is active and assert clip positions remain unchanged.
- Start playback, click `composer.timeline.ripple`, and assert transport state is unchanged.
- Assert the toggle's active/inactive state through a stable automation value or selector.

## Required Changes

- Add the missing ripple toggle behavior tests.
- Expose clip positions and ripple toggle active state through automation if current state access is insufficient.
- Adjust ripple delete behavior only if the on/off operation tests expose an issue.

## Verification

- `cargo test -p tench-composer --test plan_ui_e2e ripple_toggle`
- `cargo test -p tench-composer`
