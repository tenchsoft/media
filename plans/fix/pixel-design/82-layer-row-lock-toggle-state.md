# Layer Row Lock Toggle State

## Source Plan
- `plans/pixel-design/layer-row-lock-toggle-work-plan.md`

## Gap Analysis
The lock toggle flips `layer.locked` and updates status, but the automation node is labeled only `Layer N Lock` and does not expose the current locked/unlocked value. Tests cannot verify the row lock indicator through stable UI metadata. See `apps/pixel-design/src-tauri/src/ui/mod.rs:1088` and `apps/pixel-design/src-tauri/src/ui/layers.rs:142`.

Destructive tools do not consult `layer.locked` before mutating active-layer pixels or offsets. Brush/Eraser, Fill, Move, Gradient, Shape, and Text all write through `active_layer_mut()` without a lock guard. See `apps/pixel-design/src-tauri/src/ui/state.rs:826`, `apps/pixel-design/src-tauri/src/ui/state.rs:921`, `apps/pixel-design/src-tauri/src/ui/state.rs:954`, `apps/pixel-design/src-tauri/src/ui/state.rs:1035`, `apps/pixel-design/src-tauri/src/ui/state.rs:1068`, and `apps/pixel-design/src-tauri/src/ui/state.rs:1124`.

Toggling lock does not mark the document dirty or create history, so lock metadata may not be persisted or undoable according to the document history model. See `apps/pixel-design/src-tauri/src/ui/state.rs:690`.

The current E2E coverage clicks one lock toggle and only asserts the flag is true. It does not verify status text, lock indicator rendering, non-active row behavior, Brush/Move blocking while locked, edit allowance after unlock, dirty state, history behavior, or flattened-canvas stability. See `apps/pixel-design/src-tauri/tests/pixel_design_e2e.rs:290`.

## Plan Requirements Not Met
- Layer lock automation must expose the current locked/unlocked state.
- All destructive edit tools must refuse or clearly block locked-layer mutation.
- Lock metadata must participate in dirty/history behavior or have an explicit non-undoable rule.
- Tests must verify Brush and Move are blocked on locked layers.
- Tests must verify unlocking allows the same Brush and Move edits.
- Tests must verify toggling lock on a non-active row does not change active layer selection.

## Required Test Shape
- Toggle lock on the active row and assert locked state, automation value, status, dirty/history behavior, and row indicator.
- With the layer locked, attempt Brush and Move and assert pixels/offsets, history, dirty state, status, flattened output, and capture follow the blocked-action rule.
- Unlock the layer, repeat Brush and Move, and assert the edits are allowed.
- Add a second layer, toggle lock on the non-active row, and assert active layer selection remains unchanged unless product rules say otherwise.

## Required Changes
- Expose locked/unlocked state in `pd.layer.lock.{idx}` automation nodes.
- Add locked-layer guards to all destructive active-layer edit paths.
- Define and implement dirty/history behavior for lock metadata.
- Add Layer Lock E2E tests for indicator state, active/non-active rows, blocked edits, unlocked edits, history, dirty state, and flattened output.

## Verification
- `cargo test -p tench-pixel-design --test pixel_design_e2e layer_row_lock_toggle`
- `cargo test -p tench-pixel-design`
