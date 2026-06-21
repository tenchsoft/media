# Film Adjust Preset Button State

## Source Plan

- `plans/pixel-design/adjust-preset-film-button-work-plan.md`

## Gap Analysis

Clicking Film toggles `active_adjust`, but `apply_adjust_preset("Film")` is called on every click. A second click can turn the active indicator off while applying the Film filter again and pushing another history snapshot. See `apps/pixel-design/src-tauri/src/ui/mod.rs:523` and `apps/pixel-design/src-tauri/src/ui/state.rs:1385`.

The current E2E coverage clicks every preset and only asserts `active_adjust.is_some()`. It does not verify Film pixels, flattened output refresh, status text, second-click behavior, or undo restoration. See `apps/pixel-design/src-tauri/tests/pixel_design_e2e.rs:609`.

## Plan Requirements Not Met

- Repeated Film clicks must have a defined behavior that does not silently reapply a destructive filter while showing inactive state.
- Tests must verify Film-specific pixel changes and flattened output refresh.
- Undo after Film must be tested to restore previous layer pixels.
- Tests must verify active indicator behavior for first and second clicks.

## Required Test Shape

- Add a Pixel Design UI automation test that clicks `pd.adjust.film` and asserts active state, status text, history advance, and Film-style pixel changes.
- Click `pd.adjust.film` again and assert the defined repeated-click behavior without corrupting history or pixels.
- Undo after applying Film and assert the active layer pixels restore.
- Assert no unrelated preset is active after Film is selected.

## Required Changes

- Define whether repeated preset clicks toggle off without reapplying or reapply while staying active.
- Gate `apply_adjust_preset` and history pushes to match that behavior.
- Add Film-specific E2E and state tests.

## Verification

- `cargo test -p tench-pixel-design --test pixel_design_e2e adjust_preset_film`
- `cargo test -p tench-pixel-design`
