# J Reverse Shuttle Shortcut State

## Source Plan

- `plans/composer/j-reverse-shuttle-shortcut-control-work-plan.md`

## Gap Analysis

The global `j` shortcut does not check modifiers before invoking reverse shuttle. Ctrl+J, Alt+J, or modified text-field events can fall through to the global shortcut path. See `apps/composer/src-tauri/src/ui/mod.rs:987`.

Focused subtitle input consumes plain character input, but modified character events are not consumed and can still reach the global `j` shortcut. See `apps/composer/src-tauri/src/ui/mod.rs:910`.

Reverse shuttle updates playback state and preview label through `shuttle_reverse`, but it does not set a user notice for the visible state change. See `apps/composer/src-tauri/src/ui/state.rs:671` and `apps/composer/src-tauri/src/ui/mod.rs:987`.

The current keyboard E2E test presses `"j"` and only asserts that the capture changed. It does not assert reverse direction, speed acceleration, preview label text, modifier handling, subtitle-editor precedence, or repeated-press clamping. See `apps/composer/src-tauri/tests/plan_ui_e2e.rs:377`.

## Plan Requirements Not Met

- The global J shortcut must respect modifier keys before invoking reverse shuttle.
- Text-focused controls must prevent modified J events from triggering the global shortcut unless that shortcut is explicitly allowed.
- Reverse shuttle must show a notice or equivalent user-visible status update required by the shortcut contract.
- Tests must verify shuttle state and preview label, not only a changed capture.
- Repeated J presses must be tested for deterministic acceleration and clamp behavior.

## Required Test Shape

- Add a Composer UI automation test that presses `j` and asserts `shuttle_direction == -1`, `is_playing == true`, and the preview label shows reverse shuttle state.
- Press `j` repeatedly and assert the speed sequence clamps at the maximum supported reverse speed.
- Press Ctrl+J and Alt+J and assert they do not run the global reverse shuttle shortcut.
- Focus the subtitle editor, press `j`, and assert text editing takes precedence while shuttle state remains unchanged.
- Assert the shortcut produces the required notice or status node.

## Required Changes

- Normalize J shortcut routing through a helper that rejects unsupported modifiers and honors text focus consistently.
- Add or expose a stable automation value for the preview shuttle label.
- Set the required notice or status when reverse shuttle starts or accelerates.

## Verification

- `cargo test -p tench-composer --test plan_ui_e2e reverse_shuttle_shortcut`
- `cargo test -p tench-composer`
