# Subtitle Style Modal Close Button State

## Source Plan
- `plans/player/subtitle-style-modal-close-button-work-plan.md`

## Gap Analysis
The current E2E clicks `player.subtitle_style.close` but does not assert `subtitle_style_open == false`, close selector absence, value persistence, reopen labels, or no-subtitle-loaded behavior. See `apps/player/src-tauri/tests/plan_ui_e2e.rs:448`.

`CloseModal` leaves `subtitle_style` values intact, but there is no test proving this after several style changes or after closing and reopening the modal. See `apps/player/src-tauri/src/ui/app.rs:1345`.

Several subtitle style properties are not applied by the subtitle renderer, so closing the modal cannot currently guarantee that changed style remains active for future subtitle rendering. See `apps/player/src-tauri/src/ui/paint_video.rs:178`.

The close button uses shared `CloseModal`, which clears several unrelated modal/focus flags. There is no focused test proving Subtitle Style close only commits the expected cleanup scope.

## Plan Requirements Not Met
- Tests must verify the Subtitle Style modal closes after the Close button.
- Tests must verify style values remain unchanged after close.
- Tests must verify reopening shows displayed values matching the active style.
- Tests must verify changed style remains active in future subtitle rendering.
- Tests must verify closing with no subtitle loaded does not change playback state.
- Tests must verify close does not disturb unrelated modal/focus state beyond the documented `CloseModal` scope.

## Required Test Shape
- Change multiple style values, click Close, and assert `subtitle_style_open == false` and values remain in state.
- Reopen the style modal and assert visible labels/automation values match the changed state.
- Render subtitles after close and assert the overlay reflects the changed style.
- Clear subtitle text, close the modal, and assert playback/media state is unchanged.

## Required Changes
- Add Subtitle Style close persistence coverage to `plan_ui_e2e` or a focused modal test.
- Expose style values through automation labels or values.
- Apply all active subtitle style values in subtitle rendering.
- Document the shared `CloseModal` cleanup scope for Subtitle Style close.

## Verification
- `cargo test -p tench-player --test plan_ui_e2e subtitle_style_modal_close`
- `cargo test -p tench-player`
