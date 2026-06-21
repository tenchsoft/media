# S Snap Shortcut State

## Source Plan

- `plans/composer/s-snap-shortcut-control-work-plan.md`

## Gap Analysis

The global `s` shortcut only checks that Control is not pressed. Alt+S or modified text-field events can still toggle snap. See `apps/composer/src-tauri/src/ui/mod.rs:1003`.

Focused subtitle input consumes plain character input, but modified character events are not consumed and can still reach the global `s` shortcut. See `apps/composer/src-tauri/src/ui/mod.rs:910`.

The S shortcut and visible snap toggle both mutate `state.snap` directly through separate paths, with no shared state method to keep behavior aligned. See `apps/composer/src-tauri/src/ui/mod.rs:293` and `apps/composer/src-tauri/src/ui/mod.rs:1003`.

The current keyboard E2E test presses `"s"` and only asserts that the capture changed. It does not assert snap state, timeline snap button active styling, repeated-toggle determinism, subtitle-editor precedence, or that Ctrl+S does not also toggle snap. See `apps/composer/src-tauri/tests/plan_ui_e2e.rs:377`.

## Plan Requirements Not Met

- The global S shortcut must reject unsupported modifier combinations, not just Control.
- Focused text inputs must consume appropriate text-editing keys before the global snap shortcut can run.
- The shortcut and visible snap toggle must share a state method or equivalent behavior contract.
- Tests must verify the actual snap boolean and active toggle visual.
- Repeat presses must be tested for deterministic on/off behavior.

## Required Test Shape

- Add a Composer UI automation test that presses `s` without modifiers and asserts `state.snap` flips and `composer.timeline.snap` reflects the active state.
- Press `s` again and assert snap returns to its previous state.
- Press Ctrl+S and assert save behavior occurs without changing `state.snap`.
- Press Alt+S and assert it does not run the global snap shortcut unless explicitly intended.
- Focus the subtitle editor, press `s`, and assert text editing takes precedence while snap remains unchanged.

## Required Changes

- Add a shared `toggle_snap` state method used by both `ClickAction::ToggleSnap` and the S shortcut.
- Normalize S shortcut routing through a helper that rejects unsupported modifiers and honors text focus consistently.
- Expose snap active state through automation if screenshot assertions are insufficient.

## Verification

- `cargo test -p tench-composer --test plan_ui_e2e snap_shortcut`
- `cargo test -p tench-composer`
