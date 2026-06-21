# Brightness Adjustment Slider State

## Source Plan

- `plans/pixel-design/adjust-brightness-slider-control-work-plan.md`

## Gap Analysis

The Brightness adjustment is only addressable by row index (`pd.adjust.slider.0`) rather than a semantic Brightness control id. The hit test maps clicks in the shared slider list to an index from y-position, so tests cannot prove that only Brightness bounds trigger Brightness behavior independent of row ordering. See `apps/pixel-design/src-tauri/src/ui/mod.rs:537` and `apps/pixel-design/src-tauri/tests/pixel_design_e2e.rs:599`.

Brightness adjustments mutate the active layer without pushing history, while presets push history before applying filters. Undo restores document snapshots only, and `adjust_values.brightness` is not part of the document snapshot, so undo cannot restore pixels and displayed Brightness value together. See `apps/pixel-design/src-tauri/src/ui/state.rs:1397`, `apps/pixel-design/src-tauri/src/ui/state.rs:1407`, and `apps/pixel-design/src-tauri/src/ui/state.rs:1242`.

The current E2E coverage clicks every adjustment slider once and asserts only that each value increases. It does not verify Brightness-specific pixels, flattened output refresh, min/max clamps, immediate acknowledgment state, or undo restoration. See `apps/pixel-design/src-tauri/tests/pixel_design_e2e.rs:622`.

## Plan Requirements Not Met

- Brightness must have a semantic hit region or automation id that proves only the Brightness slider controls Brightness.
- Brightness adjustments must participate in destructive adjustment history.
- Undo after Brightness must restore both pixels and the displayed Brightness value.
- Tests must cover Brightness min/max clamp, immediate UI acknowledgment, flattened output refresh, and pixel changes.

## Required Test Shape

- Add a Pixel Design UI automation test that activates the Brightness slider by semantic selector and asserts `adjust_values.brightness` changes by the expected amount.
- Verify the active layer pixels and flattened output change after Brightness.
- Repeatedly decrease and increase Brightness and assert clamp behavior at the documented min/max.
- Apply Brightness, undo, and assert both layer pixels and displayed Brightness value restore together.
- Assert no other adjustment value changes when the Brightness slider is activated.

## Required Changes

- Add semantic adjustment slider selectors such as `pd.adjust.brightness`.
- Push adjustment history for Brightness and store enough adjustment state for undo/redo to restore the displayed value.
- Add Brightness-specific E2E and state tests.

## Verification

- `cargo test -p tench-pixel-design --test pixel_design_e2e adjust_brightness`
- `cargo test -p tench-pixel-design`
