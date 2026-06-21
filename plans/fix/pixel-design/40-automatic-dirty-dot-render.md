# Automatic Dirty Dot Render

## Source Plan
- `plans/pixel-design/automatic-dirty-dot-render-work-plan.md`

## Gap Analysis
The dirty dot render is derived from `state.document.dirty`, and save success clears that flag while save failure leaves it unchanged. However, several document-mutating operations do not set dirty or push history: layer visibility, layer lock, layer opacity, and adjust slider filters mutate document-visible state directly. See `apps/pixel-design/src-tauri/src/ui/toolbar.rs:58`, `apps/pixel-design/src-tauri/src/ui/state.rs:679`, `apps/pixel-design/src-tauri/src/ui/state.rs:774`, and `apps/pixel-design/src-tauri/src/ui/state.rs:1405`.

The Save toolbar flow without a file path only sets a "No file path" status, and current E2E coverage does not exercise a successful save that clears dirty or a failed save that preserves dirty. See `apps/pixel-design/src-tauri/src/ui/mod.rs:61` and `apps/pixel-design/src-tauri/tests/pixel_design_e2e.rs:350`.

The current E2E coverage verifies the dirty dot appears after a brush stroke, but it does not verify disappearance after successful save, preservation after failed save, dirty behavior for all document mutations, alternate mutation paths, persona switches, or resize. See `apps/pixel-design/src-tauri/tests/pixel_design_e2e.rs:379`.

## Plan Requirements Not Met
- Every document-mutating operation must set `document.dirty`, including visibility, lock, opacity, and adjust-filter changes.
- Save success must be tested to clear dirty and remove the dirty dot.
- Save failure must be tested to preserve dirty and keep the dirty dot visible.
- Tests must verify dirty dot lifecycle through multiple mutation paths.
- Tests must verify dirty dot state remains correct after persona switches and viewport resize.
- Automation must verify both presence and absence of `pd.auto.dirty_dot` after clean/dirty transitions.

## Required Test Shape
- Add a Pixel Design UI automation test that makes a brush edit, asserts `document.dirty` and `pd.auto.dirty_dot`, saves to a fixture path, and asserts dirty clears and the node disappears.
- Force a save failure after a dirty edit and assert dirty remains true, status reports failure, and `pd.auto.dirty_dot` remains present.
- Mutate layer visibility, layer lock, layer opacity, and adjust slider filters and assert each sets dirty.
- Trigger equivalent dirty mutations through UI and keyboard paths and assert identical dirty-dot output.
- Switch personas and resize after dirty and clean states and assert the dirty dot remains correct.

## Required Changes
- Mark dirty for all document-visible and persisted metadata mutations.
- Add save-success and save-failure test fixtures or dependency injection for save paths.
- Add dirty-dot lifecycle E2E and state tests.

## Verification
- `cargo test -p tench-pixel-design --test pixel_design_e2e automatic_dirty_dot_render`
- `cargo test -p tench-pixel-design`
