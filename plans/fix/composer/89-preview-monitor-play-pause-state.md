# Preview Monitor Play Pause State

## Source Plan

- `plans/composer/preview-monitor-play-pause-control-work-plan.md`

## Gap Analysis

The preview monitor implementation toggles playback through the preview hit test, but the current E2E coverage clicks `composer.preview.play_pause` once and only asserts that the capture changed. It does not assert playing state, shuttle direction, speed, preview label text, the second-click pause path, outside-click behavior, or subtitle-editor focus behavior. See `apps/composer/src-tauri/src/ui/mod.rs:837`, `apps/composer/src-tauri/src/ui/state.rs:648`, and `apps/composer/src-tauri/tests/plan_ui_e2e.rs:131`.

The automation node exposes the preview monitor as one large button, but tests do not use explicit inside/outside point checks to prove the hit-test boundary contract. See `apps/composer/src-tauri/src/ui/mod.rs:1194`.

## Plan Requirements Not Met

- Clicking preview while paused must be tested to assert forward playback state and label.
- Clicking preview while playing must be tested to assert paused state and label.
- Clicking outside the preview monitor must be tested to leave transport unchanged.
- Clicking preview while the subtitle editor is focused must be tested against the explicit focus contract.

## Required Test Shape

- Add a Composer UI automation test that clicks inside the preview monitor and asserts `is_playing == true`, `shuttle_direction == 1`, `shuttle_speed == 1.0`, and the preview label shows forward playback.
- Click the preview monitor again and assert `is_playing == false`, `shuttle_direction == 0`, and the preview label returns to `Paused`.
- Click a point just outside the preview monitor bounds and assert transport state is unchanged.
- Focus the subtitle editor, type text, click the preview monitor, and assert the subtitle text is not lost while playback behavior follows the product contract.

## Required Changes

- Add the missing preview monitor scenario tests.
- Expose preview label text or transport state through automation if current state access is not enough.
- Adjust focus handling only if the subtitle-focused scenario exposes unexpected text loss.

## Verification

- `cargo test -p tench-composer --test plan_ui_e2e preview_play_pause`
- `cargo test -p tench-composer`
