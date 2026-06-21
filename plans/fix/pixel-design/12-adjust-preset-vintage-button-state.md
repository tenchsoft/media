# Vintage Adjust Preset Button State

## Source Plan
- `plans/pixel-design/adjust-preset-vintage-button-work-plan.md`

## Gap Analysis
Clicking Vintage toggles `active_adjust`, but `apply_adjust_preset("Vintage")` is called on every click. A second click can turn the active indicator off while applying the Vintage filter again and pushing another history snapshot. See `apps/pixel-design/src-tauri/src/ui/mod.rs:523` and `apps/pixel-design/src-tauri/src/ui/state.rs:1385`.

The current E2E coverage clicks every preset and only asserts `active_adjust.is_some()`. It does not verify Vintage pixels, flattened output refresh, status text, second-click behavior, or undo restoration. See `apps/pixel-design/src-tauri/tests/pixel_design_e2e.rs:609`.

## Plan Requirements Not Met
- Repeated Vintage clicks must have a defined behavior that does not silently reapply a destructive filter while showing inactive state.
- Tests must verify Vintage-specific pixel changes and flattened output refresh.
- Undo after Vintage must be tested to restore previous layer pixels.
- Tests must verify active indicator behavior for first and second clicks.

## Required Test Shape
- Add a Pixel Design UI automation test that clicks `pd.adjust.vintage` and asserts active state, status text, history advance, and sepia-style pixel changes.
- Click `pd.adjust.vintage` again and assert the defined repeated-click behavior without corrupting history or pixels.
- Undo after applying Vintage and assert the active layer pixels restore.
- Assert no unrelated preset is active after Vintage is selected.

## Required Changes
- Define whether repeated preset clicks toggle off without reapplying or reapply while staying active.
- Gate `apply_adjust_preset` and history pushes to match that behavior.
- Add Vintage-specific E2E and state tests.

## Verification
- `cargo test -p tench-pixel-design --test pixel_design_e2e adjust_preset_vintage`
- `cargo test -p tench-pixel-design`
