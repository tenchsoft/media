# O Out Point Shortcut State

## Source Plan

- `plans/composer/o-out-point-shortcut-control-work-plan.md`

## Gap Analysis

The global `o` shortcut does not check modifiers before setting the out point. Ctrl+O, Alt+O, or modified text-field events can fall through to the global shortcut path. See `apps/composer/src-tauri/src/ui/mod.rs:1066`.

Focused subtitle input consumes plain character input, but modified character events are not consumed and can still reach the global `o` shortcut. See `apps/composer/src-tauri/src/ui/mod.rs:910`.

The shortcut matches only lowercase `"o"`. If platform keyboard events report shifted `O` as an uppercase character, pressing the physical O key with Shift will not follow the same deterministic shortcut path. See `apps/composer/src-tauri/src/ui/mod.rs:1066`.

The current keyboard E2E test presses `"o"` and only asserts that the capture changed. It does not assert `state.out_point`, the preview/timeline OUT marker, text-focus precedence in the subtitle editor, or deterministic behavior after repeated presses. See `apps/composer/src-tauri/tests/plan_ui_e2e.rs:377`.

## Plan Requirements Not Met

- The global out-point shortcut must ignore unsupported modifier combinations instead of accepting every `o` character event.
- Focused text inputs must consume appropriate text-editing keys before the global out-point shortcut can run.
- Pressing the O key should be normalized consistently for lowercase and uppercase character events where appropriate.
- Tests must verify the actual out-point state and visible OUT marker, not just a changed capture.
- Repeated shortcut presses must be tested for deterministic state and valid frame clamping.

## Required Test Shape

- Add a Composer UI automation test that presses `o` without modifiers and asserts `state.out_point == current_frame`.
- Assert the preview or timeline exposes an OUT marker after the shortcut.
- Focus the subtitle editor, press `o`, and assert subtitle text receives the character while `state.out_point` remains unchanged.
- Press Ctrl+O and Alt+O and assert they do not run the global out-point shortcut unless explicitly intended.
- Press `o` repeatedly at different frames and assert the stored out point remains within the valid timeline range.

## Required Changes

- Normalize the O-key shortcut through a helper that respects Control, Alt, and text focus consistently.
- Reuse that helper for any visible out-point action if one is added.
- Add automation state or selectors for the OUT marker so tests can assert the visible result directly.

## Verification

- `cargo test -p tench-composer --test plan_ui_e2e out_point_shortcut`
- `cargo test -p tench-composer`
