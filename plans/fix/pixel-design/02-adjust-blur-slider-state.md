# Blur Adjustment Slider State

## Source Plan

- `plans/pixel-design/adjust-blur-slider-control-work-plan.md`

## Gap Analysis

The Blur adjustment is only addressable by row index (`pd.adjust.slider.6`) rather than a semantic Blur control id. The hit test maps any click in the shared slider list to an index from y-position, so tests cannot prove that only Blur bounds trigger Blur behavior independent of row ordering. See `apps/pixel-design/src-tauri/src/ui/mod.rs:537` and `apps/pixel-design/src-tauri/tests/pixel_design_e2e.rs:599`.

`apply_adjust_filter(6)` constructs a `GaussianBlur` from `adjust_values.blur`, but the backend filter ignores the radius argument and always applies a fixed box blur radius. See `apps/pixel-design/src-tauri/src/ui/state.rs:1425` and `crates/pixel-core/src/filter.rs:45`.

Blur adjustments mutate the active layer without pushing history, while presets push history before applying filters. Undo restores document snapshots only, and `adjust_values.blur` is not part of the document snapshot, so undo cannot restore pixels and displayed blur value together. See `apps/pixel-design/src-tauri/src/ui/state.rs:1397`, `apps/pixel-design/src-tauri/src/ui/state.rs:1430`, and `apps/pixel-design/src-tauri/src/ui/state.rs:1242`.

The current E2E coverage clicks every adjustment slider once and asserts only that each value increases. It does not verify Blur-specific pixels, flattened output refresh, min/max clamps, immediate acknowledgment state, or undo restoration. See `apps/pixel-design/src-tauri/tests/pixel_design_e2e.rs:622`.

## Plan Requirements Not Met

- Blur must have a semantic hit region or automation id that proves only the Blur slider controls Blur.
- Backend blur must honor the selected blur amount or document the fixed-radius behavior.
- Blur adjustments must participate in destructive adjustment history.
- Undo after Blur must restore both pixels and the displayed blur value.
- Tests must cover Blur min/max clamp, immediate UI acknowledgment, flattened output refresh, and pixel changes.

## Required Test Shape

- Add a Pixel Design UI automation test that activates the Blur slider by semantic selector and asserts `adjust_values.blur` changes by the expected amount.
- Verify the active layer pixels and flattened output change after Blur.
- Repeatedly decrease and increase Blur and assert clamp behavior at the documented min/max.
- Apply Blur, undo, and assert both layer pixels and displayed Blur value restore together.
- Assert no other adjustment value changes when the Blur slider is activated.

## Required Changes

- Add semantic adjustment slider selectors such as `pd.adjust.blur`.
- Make the blur backend respect the requested radius or explicitly map Blur values to supported backend strengths.
- Push adjustment history for Blur and store enough adjustment state for undo/redo to restore the displayed value.
- Add Blur-specific E2E and state tests.

## Verification

- `cargo test -p tench-pixel-design --test pixel_design_e2e adjust_blur`
- `cargo test -p tench-pixel-design`
