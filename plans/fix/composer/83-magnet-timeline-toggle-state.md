# Magnet Timeline Toggle State

## Source Plan

- `plans/composer/magnet-timeline-toggle-button-work-plan.md`

## Gap Analysis

The magnet toggle flips `state.magnetic`, but timeline editing paths do not use that boolean. Clip move and trim finalization call `snap_position`, and `snap_position` checks `state.snap` rather than `state.magnetic`, so enabling magnet has no editing effect. See `apps/composer/src-tauri/src/ui/mod.rs:593` and `apps/composer/src-tauri/src/ui/state.rs:1126`.

The current E2E coverage clicks `composer.timeline.magnet` once and asserts the boolean changed plus a capture difference. It does not click the toggle back off, verify that clip movement uses magnetic behavior when enabled, verify behavior stops when disabled, or confirm playback state remains unchanged during a toggle. See `apps/composer/src-tauri/tests/plan_ui_e2e.rs:150`.

## Plan Requirements Not Met

- The magnetic editing boolean must be applied in a concrete editing path such as clip move, trim, snapping, or ripple deletion.
- Turning magnet off must disable that magnetic editing behavior.
- Tests must prove the toggle changes subsequent timeline operations, not just `state.magnetic`.
- Toggling magnet during playback must preserve transport state.
- Toggle behavior while no drag is active must be tested to ensure no clip movement occurs.

## Required Test Shape

- Add a Composer UI automation test that enables `composer.timeline.magnet`, performs a timeline move or trim near a magnetic boundary, and asserts the resulting frame uses magnetic behavior.
- Disable magnet, repeat the same operation, and assert the result no longer uses magnetic behavior.
- Toggle magnet while playback is active and assert `is_playing`, shuttle direction, and current transport state are unchanged.
- Toggle magnet when no drag is active and assert clip positions remain unchanged.

## Required Changes

- Define how `state.magnetic` differs from or composes with `state.snap`.
- Apply `state.magnetic` in the selected timeline editing path.
- Expose enough automation state to compare clip positions before and after magnet-sensitive operations.

## Verification

- `cargo test -p tench-composer --test plan_ui_e2e magnet_toggle`
- `cargo test -p tench-composer`
