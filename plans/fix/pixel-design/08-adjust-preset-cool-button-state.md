# Cool Adjust Preset Button State

## Source Plan

- `plans/pixel-design/adjust-preset-cool-button-work-plan.md`

## Gap Analysis

Clicking Cool toggles `active_adjust`, but `apply_adjust_preset("Cool")` is called on every click. A second click can turn the active indicator off while applying the Cool filter again and pushing another history snapshot. See `apps/pixel-design/src-tauri/src/ui/mod.rs:523` and `apps/pixel-design/src-tauri/src/ui/state.rs:1385`.

The current E2E coverage clicks every preset and only asserts `active_adjust.is_some()`. It does not verify Cool pixels, flattened output refresh, status text, second-click behavior, or undo restoration. See `apps/pixel-design/src-tauri/tests/pixel_design_e2e.rs:609`.

## Plan Requirements Not Met

- Repeated Cool clicks must have a defined behavior that does not silently reapply a destructive filter while showing inactive state.
- Tests must verify Cool-specific pixel changes and flattened output refresh.
- Undo after Cool must be tested to restore previous layer pixels.
- Tests must verify active indicator behavior for first and second clicks.

## Required Test Shape

- Add a Pixel Design UI automation test that clicks `pd.adjust.cool` and asserts active state, status text, history advance, and cooler pixel temperature changes.
- Click `pd.adjust.cool` again and assert the defined repeated-click behavior without corrupting history or pixels.
- Undo after applying Cool and assert the active layer pixels restore.
- Assert no unrelated preset is active after Cool is selected.

## Required Changes

- Define whether repeated preset clicks toggle off without reapplying or reapply while staying active.
- Gate `apply_adjust_preset` and history pushes to match that behavior.
- Add Cool-specific E2E and state tests.

## Verification

- `cargo test -p tench-pixel-design --test pixel_design_e2e adjust_preset_cool`
- `cargo test -p tench-pixel-design`
