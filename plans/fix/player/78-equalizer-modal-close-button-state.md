# Equalizer Modal Close Button State

## Source Plan
- `plans/player/equalizer-modal-close-button-work-plan.md`

## Gap Analysis
The current E2E clicks `player.equalizer.close` and only asserts the close selector is absent. It does not verify that modified EQ values remain in state, remain visible after reopening, or continue to be applied to the backend. See `apps/player/src-tauri/tests/plan_ui_e2e.rs:495`.

The EQ backend side effect is still a TODO for band and preset changes, so closing the modal cannot be proven to leave active backend EQ settings in place. See `apps/player/src-tauri/src/ui/app.rs:1154`.

Close with no media loaded is not covered. The close path should remain a UI-only state transition with no backend or media error, but no test sets `has_media = false` before opening and closing the equalizer.

## Plan Requirements Not Met
- Tests must verify changed EQ values remain after closing the modal.
- Tests must verify reopening the equalizer shows the active values.
- Tests must verify backend EQ settings remain applied after close.
- Tests must verify close is safe with no media loaded.

## Required Test Shape
- Change one or more bands, close the equalizer, and assert `eq_open == false` while `eq_bands` remains unchanged.
- Reopen the equalizer and assert visible band labels match the changed values.
- Use the EQ backend test hook from the band-button fixes and assert close does not reset or clear backend gains.
- Set no-media state, open and close the equalizer, and assert no error/toast regression and no unrelated playback mutation.

## Required Changes
- Add equalizer close persistence coverage to `plan_ui_e2e` or a focused modal test.
- Reuse the backend equalizer abstraction required by the band and preset fixes.
- Expose equalizer band label/value assertions through automation if needed.

## Verification
- `cargo test -p tench-player --test plan_ui_e2e equalizer_modal_close`
- `cargo test -p tench-player`
