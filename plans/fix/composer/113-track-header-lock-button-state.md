# Track Header Lock Button State

## Source Plan

- `plans/composer/track-header-lock-button-work-plan.md`

## Gap Analysis

The L button toggles `track.locked` and renders header button styling from that boolean, but clip move, trim, drop, and delete paths do not check locked tracks before mutating timeline data. See `apps/composer/src-tauri/src/ui/timeline_panel.rs:223`, `apps/composer/src-tauri/src/ui/mod.rs:304`, and `apps/composer/src-tauri/src/ui/state.rs:917`.

Paste partially honors locking by selecting the first unlocked track and reporting `No unlocked track`, but that behavior is not covered by E2E. See `apps/composer/src-tauri/src/ui/state.rs:1002`.

The current E2E coverage clicks `composer.track.lock` once and asserts the boolean changed. It does not click again, verify editing is blocked, verify notices, or isolate one locked track among many. See `apps/composer/src-tauri/tests/plan_ui_e2e.rs:163`.

## Plan Requirements Not Met

- Move, trim, drop, paste, and delete paths must consistently honor `track.locked` where applicable.
- Moving a clip onto a locked track must be blocked with a clear notice.
- Unlocking a track must restore editing availability.
- Paste with all tracks locked must be tested to report `No unlocked track`.
- Locking one track must be tested to leave other tracks editable.

## Required Test Shape

- Add a Composer UI automation test that locks a track, attempts to move a clip onto it, and asserts the move is blocked plus a notice is shown.
- Attempt trim and delete on a locked track and assert the configured locked-track behavior.
- Unlock the track and assert the same edit becomes available.
- Lock all tracks, paste a clip, and assert `No unlocked track` with no duplicate clip.
- Add another track, lock only one, and assert edits on the unlocked track still work.

## Required Changes

- Add locked-track guards to clip move, trim, media drop, delete, and any other timeline mutation paths that should respect locks.
- Keep paste behavior aligned with those guards and add tests for all-tracks-locked.
- Expose track locked state, edit notices, and clip positions through automation.

## Verification

- `cargo test -p tench-composer --test plan_ui_e2e track_lock`
- `cargo test -p tench-composer`
