# HDR Adjust Preset Button State

## Source Plan
- `plans/pixel-design/adjust-preset-hdr-button-work-plan.md`

## Gap Analysis
Clicking HDR toggles `active_adjust`, but `apply_adjust_preset("HDR")` is called on every click. A second click can turn the active indicator off while applying the HDR filter again and pushing another history snapshot. See `apps/pixel-design/src-tauri/src/ui/mod.rs:523` and `apps/pixel-design/src-tauri/src/ui/state.rs:1385`.

The current E2E coverage clicks every preset and only asserts `active_adjust.is_some()`. It does not verify HDR pixels, flattened output refresh, status text, second-click behavior, or undo restoration. See `apps/pixel-design/src-tauri/tests/pixel_design_e2e.rs:609`.

## Plan Requirements Not Met
- Repeated HDR clicks must have a defined behavior that does not silently reapply a destructive filter while showing inactive state.
- Tests must verify HDR-specific pixel changes and flattened output refresh.
- Undo after HDR must be tested to restore previous layer pixels.
- Tests must verify active indicator behavior for first and second clicks.

## Required Test Shape
- Add a Pixel Design UI automation test that clicks `pd.adjust.hdr` and asserts active state, status text, history advance, and HDR-style pixel changes.
- Click `pd.adjust.hdr` again and assert the defined repeated-click behavior without corrupting history or pixels.
- Undo after applying HDR and assert the active layer pixels restore.
- Assert no unrelated preset is active after HDR is selected.

## Required Changes
- Define whether repeated preset clicks toggle off without reapplying or reapply while staying active.
- Gate `apply_adjust_preset` and history pushes to match that behavior.
- Add HDR-specific E2E and state tests.

## Verification
- `cargo test -p tench-pixel-design --test pixel_design_e2e adjust_preset_hdr`
- `cargo test -p tench-pixel-design`
