# Saturation Adjustment Slider State

## Source Plan
- `plans/pixel-design/adjust-saturation-slider-control-work-plan.md`

## Gap Analysis
The Saturation row updates `adjust_values.saturation` and maps index `2` to `Filter::Saturation`, but the handler only treats the slider as a click target that nudges by `-5` or `+5`. There is no adjust-slider drag path that updates saturation continuously from pointer movement. See `apps/pixel-design/src-tauri/src/ui/mod.rs:537` and `apps/pixel-design/src-tauri/src/ui/state.rs:303`.

Applying the saturation filter mutates the active layer without pushing history or setting a status message. Undo after a saturation adjustment therefore is not covered by the same history behavior expected for destructive adjustments. See `apps/pixel-design/src-tauri/src/ui/state.rs:1405`.

The current E2E coverage presses each adjust slider once and only asserts that the stored value increased. It does not verify saturation pixels, flattened output refresh, status text, min/max clamp behavior, drag behavior, undo restoration, or displayed value restoration. See `apps/pixel-design/src-tauri/tests/pixel_design_e2e.rs:622`.

The automation node for the slider uses a generic label such as `Slider 2` and does not expose the Saturation label or current value, making UI-level displayed-value assertions weaker than the plan requires. See `apps/pixel-design/src-tauri/src/ui/mod.rs:1242`.

## Plan Requirements Not Met
- Dragging the Saturation slider must update saturation deterministically, not only single-click nudges.
- Saturation adjustments must push history consistently with destructive image changes.
- Saturation adjustments must set an immediate status or equivalent acknowledgment.
- Tests must verify saturation-specific pixel changes and flattened output refresh.
- Tests must verify repeated decrease and increase clamp at the documented minimum and maximum.
- Undo after saturation adjustment must restore both active layer pixels and the displayed saturation value.
- Automation metadata must expose enough label/value state to verify the displayed Saturation value.

## Required Test Shape
- Add a Pixel Design UI automation test that presses `pd.adjust.slider.2` on both sides and asserts saturation value, status text, history advance, and saturation pixel changes.
- Drag `pd.adjust.slider.2` and assert the value and pixels update from the drag, not only from click.
- Repeatedly decrease and increase `pd.adjust.slider.2` and assert `-100` and `100` clamps.
- Undo after saturation adjustment and assert the active layer pixels and UI-exposed saturation value restore together.
- Assert the flattened capture changes after saturation is applied.

## Required Changes
- Add adjust-slider drag handling for Saturation or define the control as click-only and update the plan-facing automation semantics accordingly.
- Push history for destructive saturation adjustments and include adjust value restoration in undo.
- Set a status message when Saturation changes.
- Expose Saturation label and current value through automation metadata.
- Add saturation-specific E2E and state tests.

## Verification
- `cargo test -p tench-pixel-design --test pixel_design_e2e adjust_saturation_slider`
- `cargo test -p tench-pixel-design`
