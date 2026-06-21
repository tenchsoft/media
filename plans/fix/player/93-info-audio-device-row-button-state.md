# Info Audio Device Row Button State

## Source Plan
- `plans/player/info-audio-device-row-button-work-plan.md`

## Gap Analysis
`SelectAudioDevice` updates `selected_audio_device` and toast only when a backend exists. In headless UI tests or any no-backend state, activating a visible audio-device row produces no selected row and no user-visible acknowledgement. See `apps/player/src-tauri/src/ui/app.rs:1006`.

The current E2E asserts `player.info.audio_device.system_default` is present but never activates it. It does not verify backend device selection, active row highlight, toast text, first/middle/last row targeting, list refresh behavior, or unrelated playback invariants. See `apps/player/src-tauri/tests/plan_ui_e2e.rs:450`.

Backend audio-device errors are ignored because the handler does not surface a result from `set_audio_device`. Tests cannot distinguish a successful device switch from a failed platform call.

Automation exposes device rows by simple-name debug ID but does not expose selected/active state for assertions through the UI tree. See `apps/player/src-tauri/src/ui/app.rs:2426`.

Long audio-device lists have no exercised scroll-aware row-selection coverage, despite the plan requiring dynamic row hit testing with current scroll offset.

## Plan Requirements Not Met
- Selecting a visible audio device must update selected-device state or a documented failure state without requiring a real backend.
- Tests must verify backend device selection receives the selected device name.
- Tests must verify the row highlight/active state changes.
- Tests must verify the toast names the selected device.
- Tests must cover first, middle, and last audio-device rows.
- Tests must repeat selection after the device list changes and verify row targeting refreshes.
- Tests must verify selection does not change media path, playback time, paused state, playlist, or selected playlist index.
- Automation must expose selected state for audio-device rows.

## Required Test Shape
- Seed at least three audio devices, click each row, and assert `selected_audio_device`, active UI state, toast, and backend device call.
- Run the same click with no backend and assert the documented test/no-backend result.
- Replace the device list, click a rendered device row again, and assert the current row/device is targeted.
- Inject backend failure and assert an actionable toast/state without false success.
- Create an overflowing device list and verify visible-row hit testing remains correct.

## Required Changes
- Add a testable audio-device backend abstraction or command spy.
- Update no-backend behavior so visible row activation has deterministic state or a documented error.
- Surface backend audio-device selection failures.
- Expose active state for `player.info.audio_device.*` automation nodes.
- Add value-level Info drawer audio-device coverage to `plan_ui_e2e` or a focused Info drawer test.

## Verification
- `cargo test -p tench-player --test plan_ui_e2e info_audio_device_row`
- `cargo test -p tench-player`
