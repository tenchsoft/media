# Plus Zoom In Shortcut State

## Source Plan

- `plans/composer/plus-zoom-in-shortcut-control-work-plan.md`

## Gap Analysis

The global Plus/Equals shortcut does not distinguish supported modifier combinations from unsupported ones. Ctrl+Equals, Alt+Equals, or modified text-field events can fall through to the global zoom-in path. See `apps/composer/src-tauri/src/ui/mod.rs:1045`.

Focused subtitle input consumes plain character input, but modified character events are not consumed and can still reach the global Plus/Equals shortcut. See `apps/composer/src-tauri/src/ui/mod.rs:910`.

The shortcut mutates `state.zoom` directly and repaints, but it does not set a user notice or explicit no-op/status when the maximum zoom clamp is reached. See `apps/composer/src-tauri/src/ui/mod.rs:1045`.

The current keyboard E2E test presses `"+"` and only asserts that the capture changed. It does not cover `"="`, assert the zoom value increased by exactly 10, verify clamp behavior at the maximum, or test modifier and subtitle-editor precedence. See `apps/composer/src-tauri/tests/plan_ui_e2e.rs:377`.

## Plan Requirements Not Met

- The global Plus/Equals shortcut must respect modifier keys while still allowing the platform's normal Plus key event shape.
- Text-focused controls must prevent modified Plus/Equals events from triggering the global shortcut unless that shortcut is explicitly allowed.
- Zoom-in changes must show the required notice or equivalent user-visible status, including the maximum-clamp no-op case.
- Tests must verify the actual zoom value and visible zoom label, not only a changed capture.
- Repeated Plus/Equals presses must be tested for deterministic 10-point steps and maximum clamp.

## Required Test Shape

- Add a Composer UI automation test that presses Plus and asserts `state.zoom` increases by `10.0` and the visible zoom label updates.
- Press Equals and assert it follows the same zoom-in behavior.
- Press Plus or Equals repeatedly until the maximum and assert it clamps at `200.0`.
- Press again at the maximum and assert deterministic no-op/status behavior.
- Focus the subtitle editor, press Plus/Equals with unsupported modifiers, and assert text editing or modifier handling takes precedence while zoom remains unchanged.

## Required Changes

- Normalize Plus/Equals shortcut routing through a helper that rejects unsupported modifiers and honors text focus consistently.
- Use a shared zoom-in state method if a visible zoom-in control is added or enabled.
- Set the required notice or status when zoom increases or hits the maximum.
- Expose the zoom label or value through automation for direct assertions.

## Verification

- `cargo test -p tench-composer --test plan_ui_e2e zoom_in_shortcut`
- `cargo test -p tench-composer`
