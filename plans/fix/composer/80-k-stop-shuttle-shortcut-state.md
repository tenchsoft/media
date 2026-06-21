# K Stop Shuttle Shortcut State

## Source Plan

- `plans/composer/k-stop-shuttle-shortcut-control-work-plan.md`

## Gap Analysis

The global `k` shortcut does not check modifiers before invoking shuttle stop. Ctrl+K, Alt+K, or modified text-field events can fall through to the global shortcut path. See `apps/composer/src-tauri/src/ui/mod.rs:991`.

Focused subtitle input consumes plain character input, but modified character events are not consumed and can still reach the global `k` shortcut. See `apps/composer/src-tauri/src/ui/mod.rs:910`.

`shuttle_stop` resets playback state, but the shortcut does not set a user notice or explicit status for the stop action or for the already-paused no-op case. See `apps/composer/src-tauri/src/ui/state.rs:699` and `apps/composer/src-tauri/src/ui/mod.rs:991`.

The current keyboard E2E test presses `"k"` and only asserts that the capture changed. It does not assert stopped state, preview label returning to `Paused`, modifier handling, subtitle-editor precedence, or repeated-stop determinism. See `apps/composer/src-tauri/tests/plan_ui_e2e.rs:377`.

## Plan Requirements Not Met

- The global K shortcut must respect modifier keys before invoking shuttle stop.
- Text-focused controls must prevent modified K events from triggering the global shortcut unless that shortcut is explicitly allowed.
- Stopping shuttle playback must show the required notice or equivalent user-visible status, including a clear no-op path when already stopped.
- Tests must verify stopped shuttle state and preview label, not only a changed capture.
- Repeated K presses must be tested for deterministic stopped state.

## Required Test Shape

- Start shuttle playback, press `k`, and assert `shuttle_direction == 0`, `is_playing == false`, speed reset, and the preview label is `Paused`.
- Press `k` again while already paused and assert deterministic state plus the required no-op/status behavior.
- Press Ctrl+K and Alt+K and assert they do not run the global stop shortcut.
- Focus the subtitle editor, press `k`, and assert text editing takes precedence while shuttle state remains unchanged.
- Assert the shortcut produces the required notice or status node.

## Required Changes

- Normalize K shortcut routing through a helper that rejects unsupported modifiers and honors text focus consistently.
- Add or expose a stable automation value for the preview shuttle label.
- Set the required notice or status when shuttle stops or when the stop command is a no-op.

## Verification

- `cargo test -p tench-composer --test plan_ui_e2e stop_shuttle_shortcut`
- `cargo test -p tench-composer`
