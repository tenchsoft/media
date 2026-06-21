# I In Point Shortcut State

## Source Plan

- `plans/composer/i-in-point-shortcut-control-work-plan.md`

## Gap Analysis

The global `i` shortcut only checks that Control is not pressed. Alt-modified `i` can still set the in point, and focused search fields can fall through to the global shortcut when the character event has modifiers that the search editor does not consume. See `apps/composer/src-tauri/src/ui/mod.rs:891` and `apps/composer/src-tauri/src/ui/mod.rs:1058`.

The shortcut matches only lowercase `"i"`. If platform keyboard events report shifted `I` as an uppercase character, pressing the physical I key with Shift will not follow the same deterministic shortcut path. See `apps/composer/src-tauri/src/ui/mod.rs:1058`.

The current keyboard E2E test presses `"i"` and only asserts that the capture changed. It does not assert `state.in_point`, the preview/timeline IN marker, text-focus precedence in the subtitle editor, or deterministic behavior after repeated presses. See `apps/composer/src-tauri/tests/plan_ui_e2e.rs:377`.

## Plan Requirements Not Met

- The global in-point shortcut must ignore non-text global shortcut cases with Alt or other non-supported modifiers, not just Control.
- Focused text inputs must consume appropriate text-editing keys before the global in-point shortcut can run.
- Pressing the I key should be normalized consistently for lowercase and uppercase character events where appropriate.
- Tests must verify the actual in-point state and visible IN marker, not just a changed capture.
- Repeated shortcut presses must be tested for deterministic state and valid frame clamping.

## Required Test Shape

- Add a Composer UI automation test that presses `i` without modifiers and asserts `state.in_point == current_frame`.
- Assert the preview or timeline exposes an IN marker after the shortcut.
- Focus the subtitle editor, press `i`, and assert subtitle text receives the character while `state.in_point` remains unchanged.
- Press Alt+`i` and assert it does not run the global in-point shortcut.
- Press `i` repeatedly at different frames and assert the stored in point remains within the valid timeline range.

## Required Changes

- Normalize the I-key shortcut through a helper that respects Control, Alt, and text focus consistently.
- Reuse that helper for any visible in-point action if one is added.
- Add automation state or selectors for the IN marker so tests can assert the visible result directly.

## Verification

- `cargo test -p tench-composer --test plan_ui_e2e in_point_shortcut`
- `cargo test -p tench-composer`
