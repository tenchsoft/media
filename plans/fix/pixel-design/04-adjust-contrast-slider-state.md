# Contrast Adjustment Slider State

## Source Plan

- `plans/pixel-design/adjust-contrast-slider-control-work-plan.md`

## Gap Analysis

The Contrast adjustment is only addressable by row index (`pd.adjust.slider.1`) rather than a semantic Contrast control id. The hit test maps clicks in the shared slider list to an index from y-position, so tests cannot prove that only Contrast bounds trigger Contrast behavior independent of row ordering. See `apps/pixel-design/src-tauri/src/ui/mod.rs:537` and `apps/pixel-design/src-tauri/tests/pixel_design_e2e.rs:599`.

Contrast adjustments mutate the active layer without pushing history, while presets push history before applying filters. Undo restores document snapshots only, and `adjust_values.contrast` is not part of the document snapshot, so undo cannot restore pixels and displayed Contrast value together. See `apps/pixel-design/src-tauri/src/ui/state.rs:1397`, `apps/pixel-design/src-tauri/src/ui/state.rs:1410`, and `apps/pixel-design/src-tauri/src/ui/state.rs:1242`.

The current E2E coverage clicks every adjustment slider once and asserts only that each value increases. It does not verify Contrast-specific pixels, flattened output refresh, min/max clamps, immediate acknowledgment state, or undo restoration. See `apps/pixel-design/src-tauri/tests/pixel_design_e2e.rs:622`.

## Plan Requirements Not Met

- Contrast must have a semantic hit region or automation id that proves only the Contrast slider controls Contrast.
- Contrast adjustments must participate in destructive adjustment history.
- Undo after Contrast must restore both pixels and the displayed Contrast value.
- Tests must cover Contrast min/max clamp, immediate UI acknowledgment, flattened output refresh, and pixel changes.

## Required Test Shape

- Add a Pixel Design UI automation test that activates the Contrast slider by semantic selector and asserts `adjust_values.contrast` changes by the expected amount.
- Verify the active layer pixels and flattened output change after Contrast.
- Repeatedly decrease and increase Contrast and assert clamp behavior at the documented min/max.
- Apply Contrast, undo, and assert both layer pixels and displayed Contrast value restore together.
- Assert no other adjustment value changes when the Contrast slider is activated.

## Required Changes

- Add semantic adjustment slider selectors such as `pd.adjust.contrast`.
- Push adjustment history for Contrast and store enough adjustment state for undo/redo to restore the displayed value.
- Add Contrast-specific E2E and state tests.

## Verification

- `cargo test -p tench-pixel-design --test pixel_design_e2e adjust_contrast`
- `cargo test -p tench-pixel-design`
