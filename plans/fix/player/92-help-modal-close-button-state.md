# Help Modal Close Button State

## Source Plan
- `plans/player/help-modal-close-button-work-plan.md`

## Gap Analysis
The current E2E verifies that `player.help.close` disappears after clicking the Help close button, but it does not assert focus restoration, unrelated playback invariants, reopen behavior, or Escape-after-close behavior. See `apps/player/src-tauri/tests/plan_ui_e2e.rs:632`.

Automation does not expose a meaningful focused state for playback controls after closing Help, so the plan requirement that keyboard focus returns to normal playback controls cannot be asserted. See `apps/player/src-tauri/src/ui/app.rs:1670`.

Help close uses the shared `CloseModal` action, which clears several modal and focus flags. There is no focused test proving Help close only commits the expected modal cleanup and does not disturb playback, drawer, GIF, EQ, URL, or subtitle modal state. See `apps/player/src-tauri/src/ui/app.rs:1345`.

## Plan Requirements Not Met
- Tests must verify focus returns to normal playback controls after Help close.
- Automation must expose enough focused/active state to assert focus restoration.
- Tests must verify reopening Help after close starts from a clean state.
- Tests must verify pressing Escape after Help close does not leave modal state half-open.
- Tests must verify Help close does not change playback time, paused state, media path, playlist, drawer, or unrelated modal flags.

## Required Test Shape
- Open Help, click `player.help.close`, and assert `help_open == false` plus normal playback focus/keyboard handling is restored.
- Press Space or another playback key after close and assert it reaches the playback control path.
- Reopen Help and assert only the Help modal state is present.
- Close Help, press Escape, and assert no modal flag reopens or remains half-open.
- Snapshot unrelated player state before Help close and assert it remains unchanged.

## Required Changes
- Expose focused/active playback-control state through automation.
- Add Help close focus and state-invariant coverage to `plan_ui_e2e` or a focused modal test.
- Document the intended shared `CloseModal` cleanup scope for Help close.

## Verification
- `cargo test -p tench-player --test plan_ui_e2e help_modal_close`
- `cargo test -p tench-player`
