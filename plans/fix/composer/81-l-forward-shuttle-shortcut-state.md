# L Forward Shuttle Shortcut State

## Source Plan

- `plans/composer/l-forward-shuttle-shortcut-control-work-plan.md`

## Gap Analysis

The global `l` shortcut does not check modifiers before invoking forward shuttle. Ctrl+L, Alt+L, or modified text-field events can fall through to the global shortcut path. See `apps/composer/src-tauri/src/ui/mod.rs:995`.

Focused subtitle input consumes plain character input, but modified character events are not consumed and can still reach the global `l` shortcut. See `apps/composer/src-tauri/src/ui/mod.rs:910`.

Forward shuttle updates playback state and preview label through `shuttle_forward`, but it does not set a user notice for the visible state change. See `apps/composer/src-tauri/src/ui/state.rs:685` and `apps/composer/src-tauri/src/ui/mod.rs:995`.

The current keyboard E2E test presses `"l"` and only asserts that the capture changed. It does not assert forward direction, speed acceleration, preview label text, modifier handling, subtitle-editor precedence, or repeated-press clamping. See `apps/composer/src-tauri/tests/plan_ui_e2e.rs:377`.

## Plan Requirements Not Met

- The global L shortcut must respect modifier keys before invoking forward shuttle.
- Text-focused controls must prevent modified L events from triggering the global shortcut unless that shortcut is explicitly allowed.
- Forward shuttle must show a notice or equivalent user-visible status update required by the shortcut contract.
- Tests must verify shuttle state and preview label, not only a changed capture.
- Repeated L presses must be tested for deterministic acceleration and clamp behavior.

## Required Test Shape

- Add a Composer UI automation test that presses `l` and asserts `shuttle_direction == 1`, `is_playing == true`, and the preview label shows forward shuttle state.
- Press `l` repeatedly and assert the speed sequence clamps at the maximum supported forward speed.
- Press Ctrl+L and Alt+L and assert they do not run the global forward shuttle shortcut.
- Focus the subtitle editor, press `l`, and assert text editing takes precedence while shuttle state remains unchanged.
- Assert the shortcut produces the required notice or status node.

## Required Changes

- Normalize L shortcut routing through a helper that rejects unsupported modifiers and honors text focus consistently.
- Add or expose a stable automation value for the preview shuttle label.
- Set the required notice or status when forward shuttle starts or accelerates.

## Verification

- `cargo test -p tench-composer --test plan_ui_e2e forward_shuttle_shortcut`
- `cargo test -p tench-composer`
