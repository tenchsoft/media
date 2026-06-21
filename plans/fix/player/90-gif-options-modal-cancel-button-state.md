# GIF Options Modal Cancel Button State

## Source Plan
- `plans/player/gif-options-modal-cancel-button-work-plan.md`

## Gap Analysis
The current E2E clicks `player.gif_options.cancel` but does not assert `gif_options_open == false`, `gif_recording == false`, `gif_state`, or whether the parent GIF capture modal remains in the documented state. See `apps/player/src-tauri/tests/plan_ui_e2e.rs:601`.

Cancel uses the shared `CloseModal` action, which closes several unrelated modal flags. There is no test proving GIF options cancellation only commits the expected GIF state and does not start recording or mutate GIF options. See `apps/player/src-tauri/src/ui/paint_overlays.rs:782` and `apps/player/src-tauri/src/ui/app.rs:1345`.

Reopen-after-cancel and Escape-after-cancel behavior are not covered, so stale options state or half-open overlay state can survive unnoticed.

## Plan Requirements Not Met
- Tests must verify Cancel closes the GIF options modal.
- Tests must verify Cancel does not start GIF recording.
- Tests must verify Cancel does not mutate GIF option values.
- Tests must verify the parent GIF capture modal state after Cancel is documented and stable.
- Tests must verify reopening options after Cancel shows clean option state.
- Tests must verify Escape after Cancel leaves GIF modal flags consistent.

## Required Test Shape
- Open GIF options, snapshot `gif_options`, click Cancel, and assert `gif_options_open == false`, `gif_recording == false`, and options are unchanged.
- Assert the UI tree no longer exposes `player.gif_options.start` or `player.gif_options.cancel`.
- Assert the parent GIF modal is either still open or closed according to documented behavior.
- Reopen GIF options and assert option labels match the saved values.
- Press Escape after Cancel and assert no GIF options/modal half-open state remains.

## Required Changes
- Add GIF options Cancel state assertions to `plan_ui_e2e` or a focused modal test.
- Document parent GIF modal behavior after options cancellation.
- Add automation values for GIF options if current labels are not assertable.

## Verification
- `cargo test -p tench-player --test plan_ui_e2e gif_options_modal_cancel`
- `cargo test -p tench-player`
