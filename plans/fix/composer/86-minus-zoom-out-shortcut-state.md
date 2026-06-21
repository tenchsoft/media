# Minus Zoom Out Shortcut State

## Source Plan

- `plans/composer/minus-zoom-out-shortcut-control-work-plan.md`

## Gap Analysis

The global `-` shortcut does not check modifiers before changing timeline zoom. Ctrl+Minus, Alt+Minus, or modified text-field events can fall through to the global zoom-out path. See `apps/composer/src-tauri/src/ui/mod.rs:1049`.

Focused subtitle input consumes plain character input, but modified character events are not consumed and can still reach the global `-` shortcut. See `apps/composer/src-tauri/src/ui/mod.rs:910`.

The shortcut mutates `state.zoom` directly and repaints, but it does not set a user notice or explicit no-op/status when the minimum zoom clamp is reached. See `apps/composer/src-tauri/src/ui/mod.rs:1049`.

The current keyboard E2E test presses `"-"` and only asserts that the capture changed. It does not assert the zoom value decreased by exactly 10, clamp behavior at the minimum, modifier handling, or subtitle-editor precedence. See `apps/composer/src-tauri/tests/plan_ui_e2e.rs:377`.

## Plan Requirements Not Met

- The global Minus shortcut must respect modifier keys before invoking timeline zoom out.
- Text-focused controls must prevent modified Minus events from triggering the global shortcut unless that shortcut is explicitly allowed.
- Zoom-out changes must show the required notice or equivalent user-visible status, including the minimum-clamp no-op case.
- Tests must verify the actual zoom value and visible zoom label, not only a changed capture.
- Repeated Minus presses must be tested for deterministic 10-point steps and minimum clamp.

## Required Test Shape

- Add a Composer UI automation test that presses Minus and asserts `state.zoom` decreases by `10.0` and the visible zoom label updates.
- Press Minus repeatedly until the minimum and assert it clamps at `10.0`.
- Press Minus again at the minimum and assert deterministic no-op/status behavior.
- Press Ctrl+Minus and Alt+Minus and assert they do not run the global zoom-out shortcut unless explicitly intended.
- Focus the subtitle editor, press Minus, and assert text editing takes precedence while zoom remains unchanged.

## Required Changes

- Normalize Minus shortcut routing through a helper that rejects unsupported modifiers and honors text focus consistently.
- Use a shared zoom-out state method if a visible zoom-out control is added or enabled.
- Set the required notice or status when zoom decreases or hits the minimum.
- Expose the zoom label or value through automation for direct assertions.

## Verification

- `cargo test -p tench-composer --test plan_ui_e2e zoom_out_shortcut`
- `cargo test -p tench-composer`
