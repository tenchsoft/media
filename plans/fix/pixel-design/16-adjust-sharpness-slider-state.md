# Sharpness Adjustment Slider State

## Source Plan
- `plans/pixel-design/adjust-sharpness-slider-control-work-plan.md`

## Gap Analysis
The Sharpness row updates `adjust_values.sharpness` and maps index `5` to `Filter::Sharpen`, but the handler only treats the slider as a click target that nudges by `-5` or `+5`. There is no adjust-slider drag path that updates sharpness continuously from pointer movement. See `apps/pixel-design/src-tauri/src/ui/mod.rs:537` and `apps/pixel-design/src-tauri/src/ui/state.rs:303`.

The backend `Filter::Sharpen` amount is ignored when the filter is applied, so different sharpness values, including negative values, do not produce parameter-specific output. See `crates/pixel-core/src/filter.rs:49`.

Applying the sharpness filter mutates the active layer without pushing history or setting a status message. Undo after a sharpness adjustment therefore is not covered by the same history behavior expected for destructive adjustments. See `apps/pixel-design/src-tauri/src/ui/state.rs:1405`.

The current E2E coverage presses each adjust slider once and only asserts that the stored value increased. It does not verify sharpness pixels, flattened output refresh, status text, min/max clamp behavior, drag behavior, undo restoration, or displayed value restoration. See `apps/pixel-design/src-tauri/tests/pixel_design_e2e.rs:622`.

The automation node for the slider uses a generic label such as `Slider 5` and does not expose the Sharpness label or current value, making UI-level displayed-value assertions weaker than the plan requires. See `apps/pixel-design/src-tauri/src/ui/mod.rs:1242`.

## Plan Requirements Not Met
- Dragging the Sharpness slider must update sharpness deterministically, not only single-click nudges.
- Sharpness adjustments must produce output that corresponds to the displayed sharpness amount.
- Sharpness adjustments must push history consistently with destructive image changes.
- Sharpness adjustments must set an immediate status or equivalent acknowledgment.
- Tests must verify sharpness-specific pixel changes and flattened output refresh.
- Tests must verify repeated decrease and increase clamp at the documented minimum and maximum.
- Undo after sharpness adjustment must restore both active layer pixels and the displayed sharpness value.
- Automation metadata must expose enough label/value state to verify the displayed Sharpness value.

## Required Test Shape
- Add a Pixel Design UI automation test that presses `pd.adjust.slider.5` on both sides and asserts sharpness value, status text, history advance, and parameter-sensitive pixel changes.
- Drag `pd.adjust.slider.5` and assert the value and pixels update from the drag, not only from click.
- Repeatedly decrease and increase `pd.adjust.slider.5` and assert `-100` and `100` clamps.
- Undo after sharpness adjustment and assert the active layer pixels and UI-exposed sharpness value restore together.
- Assert the flattened capture changes after sharpness is applied.

## Required Changes
- Add adjust-slider drag handling for Sharpness or define the control as click-only and update the plan-facing automation semantics accordingly.
- Make `Filter::Sharpen` respect its amount or define separate behavior for negative and positive sharpness values.
- Push history for destructive sharpness adjustments and include adjust value restoration in undo.
- Set a status message when Sharpness changes.
- Expose Sharpness label and current value through automation metadata.
- Add sharpness-specific E2E, state, and pixel-core tests.

## Verification
- `cargo test -p tench-pixel-design --test pixel_design_e2e adjust_sharpness_slider`
- `cargo test -p tench-pixel-core filter`
- `cargo test -p tench-pixel-design`
