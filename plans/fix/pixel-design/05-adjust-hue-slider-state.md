# Hue Adjustment Slider State

## Source Plan

- `plans/pixel-design/adjust-hue-slider-control-work-plan.md`

## Gap Analysis

The Hue row updates `adjust_values.hue`, but `apply_adjust_filter(4)` applies `Filter::Sharpen` using `adjust_values.sharpness` instead of applying Hue. See `apps/pixel-design/src-tauri/src/ui/state.rs:318` and `apps/pixel-design/src-tauri/src/ui/state.rs:1419`.

`pixel-core::Filter` has no Hue adjustment variant, so there is no backend filter that can apply Hue to active layer pixels. See `crates/pixel-core/src/filter.rs:7`.

The Hue adjustment is only addressable by row index (`pd.adjust.slider.4`) rather than a semantic Hue control id. The hit test maps clicks in the shared slider list to an index from y-position, so tests cannot prove that only Hue bounds trigger Hue behavior independent of row ordering. See `apps/pixel-design/src-tauri/src/ui/mod.rs:537`.

Hue adjustments mutate adjustment values without pushing history for the displayed value, and undo snapshots do not include `adjust_values.hue`. See `apps/pixel-design/src-tauri/src/ui/state.rs:1242`.

The current E2E coverage clicks every adjustment slider once and asserts only that each value increases. It does not verify Hue-specific pixels, flattened output refresh, min/max clamps, immediate acknowledgment state, or undo restoration. See `apps/pixel-design/src-tauri/tests/pixel_design_e2e.rs:622`.

## Plan Requirements Not Met

- Hue must map to a real backend Hue filter instead of Sharpen.
- Hue must have a semantic hit region or automation id that proves only the Hue slider controls Hue.
- Hue adjustments must participate in destructive adjustment history.
- Undo after Hue must restore both pixels and the displayed Hue value.
- Tests must cover Hue min/max clamp, immediate UI acknowledgment, flattened output refresh, and pixel changes.

## Required Test Shape

- Add a Pixel Design UI automation test that activates the Hue slider by semantic selector and asserts `adjust_values.hue` changes by the expected amount.
- Verify the active layer pixels and flattened output change with a Hue-specific color shift.
- Repeatedly decrease and increase Hue and assert clamp behavior at the documented min/max.
- Apply Hue, undo, and assert both layer pixels and displayed Hue value restore together.
- Assert no other adjustment value, especially Sharpness, changes when the Hue slider is activated.

## Required Changes

- Add a Hue filter variant and implementation in `pixel-core`.
- Map adjustment index 4 to the Hue filter using `adjust_values.hue`.
- Add semantic adjustment slider selectors such as `pd.adjust.hue`.
- Push adjustment history for Hue and store enough adjustment state for undo/redo to restore the displayed value.
- Add Hue-specific E2E and state tests.

## Verification

- `cargo test -p tench-pixel-design --test pixel_design_e2e adjust_hue`
- `cargo test -p tench-pixel-design`
