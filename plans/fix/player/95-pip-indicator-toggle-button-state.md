# Picture In Picture Indicator Toggle Button State

## Source Plan
- `plans/player/pip-indicator-toggle-button-work-plan.md`

## Gap Analysis
The current E2E enables PiP with the `i` key, clicks `player.pip.indicator`, and asserts the indicator disappears. It does not assert `pip_mode == false`, the `Picture-in-picture off` toast, repeated reopen/close behavior, Escape-after-close behavior, or unrelated playback invariants. See `apps/player/src-tauri/tests/plan_ui_e2e.rs:636`.

The keyboard path that enables PiP toggles `pip_mode` without showing the same toast as `ClickAction::TogglePip`, so the user-visible acknowledgement differs between keyboard and indicator click paths. See `apps/player/src-tauri/src/ui/app.rs:2166` and `apps/player/src-tauri/src/ui/app.rs:1141`.

There is no platform PiP abstraction or result state, so tests can only verify the in-app indicator flag, not whether a platform picture-in-picture mode actually changed.

## Plan Requirements Not Met
- Tests must verify clicking the indicator sets `pip_mode == false`.
- Tests must verify the off toast is shown after indicator activation.
- Keyboard and click PiP toggles must have consistent user-visible acknowledgement.
- Tests must verify repeated enable/indicator-close cycles are deterministic.
- Tests must verify Escape after indicator close does not leave PiP/modal state half-open.
- Tests must verify indicator close does not change media path, playback time, paused state, playlist, or selected playlist index.
- PiP platform state must be observable or explicitly documented as in-app-only.

## Required Test Shape
- Enable PiP, click `player.pip.indicator`, and assert `pip_mode == false`, indicator absent, and off toast.
- Enable and close PiP a second time and assert the same state transition.
- Press Escape after closing and assert `pip_mode` remains false and no modal/panel flag changes unexpectedly.
- Snapshot unrelated playback state before indicator click and assert it remains unchanged.
- If platform PiP is supported, use a test hook to assert the platform mode toggles off.

## Required Changes
- Align keyboard PiP toggle acknowledgement with `ClickAction::TogglePip` or document the difference.
- Expose PiP active/platform state through automation if platform PiP exists.
- Extend PiP indicator E2E coverage for state, toast, repeat activation, Escape, and unrelated state invariants.

## Verification
- `cargo test -p tench-player --test plan_ui_e2e pip_indicator_toggle`
- `cargo test -p tench-player`
