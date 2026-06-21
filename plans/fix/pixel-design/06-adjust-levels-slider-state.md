# Levels Adjustment Slider State

## Source Plan

- `plans/pixel-design/adjust-levels-slider-control-work-plan.md`

## Gap Analysis

The Levels row updates `adjust_values.levels`, but `apply_adjust_filter(7)` falls through to `None`, so no backend filter is applied to active layer pixels. See `apps/pixel-design/src-tauri/src/ui/state.rs:318` and `apps/pixel-design/src-tauri/src/ui/state.rs:1405`.

`pixel-core::Filter` has no Levels adjustment variant, so there is no backend filter that can apply Levels to the active layer preview. See `crates/pixel-core/src/filter.rs:7`.

The Levels adjustment is only addressable by row index (`pd.adjust.slider.7`) rather than a semantic Levels control id. The hit test maps clicks in the shared slider list to an index from y-position, so tests cannot prove that only Levels bounds trigger Levels behavior independent of row ordering. See `apps/pixel-design/src-tauri/src/ui/mod.rs:537`.

Levels adjustments mutate adjustment values without pushing history for the displayed value, and undo snapshots do not include `adjust_values.levels`. See `apps/pixel-design/src-tauri/src/ui/state.rs:1242`.

The current E2E coverage clicks every adjustment slider once and asserts only that each value increases. It does not verify Levels-specific pixels, flattened output refresh, min/max clamps, immediate acknowledgment state, or undo restoration. See `apps/pixel-design/src-tauri/tests/pixel_design_e2e.rs:622`.

## Plan Requirements Not Met

- Levels must map to a real backend Levels filter.
- Levels must have a semantic hit region or automation id that proves only the Levels slider controls Levels.
- Levels adjustments must participate in destructive adjustment history.
- Undo after Levels must restore both pixels and the displayed Levels value.
- Tests must cover Levels min/max clamp, immediate UI acknowledgment, flattened output refresh, and pixel changes.

## Required Test Shape

- Add a Pixel Design UI automation test that activates the Levels slider by semantic selector and asserts `adjust_values.levels` changes by the expected amount.
- Verify the active layer pixels and flattened output change with a Levels-specific adjustment.
- Repeatedly decrease and increase Levels and assert clamp behavior at the documented min/max.
- Apply Levels, undo, and assert both layer pixels and displayed Levels value restore together.
- Assert no other adjustment value changes when the Levels slider is activated.

## Required Changes

- Add a Levels filter variant and implementation in `pixel-core`.
- Map adjustment index 7 to the Levels filter using `adjust_values.levels`.
- Add semantic adjustment slider selectors such as `pd.adjust.levels`.
- Push adjustment history for Levels and store enough adjustment state for undo/redo to restore the displayed value.
- Add Levels-specific E2E and state tests.

## Verification

- `cargo test -p tench-pixel-design --test pixel_design_e2e adjust_levels`
- `cargo test -p tench-pixel-design`
