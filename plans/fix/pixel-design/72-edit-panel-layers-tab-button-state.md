# Layers Panel Tab Button State

## Source Plan
- `plans/pixel-design/edit-panel-layers-tab-button-work-plan.md`

## Gap Analysis
Layers tab buttons are exposed as generic automation buttons without selected or active-state metadata. The renderer draws active tab styling from `panel_tab`, but tests cannot directly verify the Layers tab button highlight through the UI tree. See `apps/pixel-design/src-tauri/src/ui/mod.rs:1055` and `apps/pixel-design/src-tauri/src/ui/layers.rs:31`.

The current E2E coverage asserts Layers controls are present in the default Edit panel, but it does not click Layers from Properties or History, assert `panel_tab == PanelTab::Layers`, verify active tab styling, verify absence of Properties/History-only controls, test repeated-click stability, or test persona switch/return behavior. See `apps/pixel-design/src-tauri/tests/pixel_design_e2e.rs:150` and `apps/pixel-design/src-tauri/tests/pixel_design_e2e.rs:268`.

## Plan Requirements Not Met
- Layers tab automation must expose active/selected tab state or equivalent metadata.
- Tests must verify clicking Layers from each other Edit panel tab shows only Layers-specific content.
- Tests must verify repeated Layers clicks do not reset active document, active layer, or history state.
- Tests must verify switching personas and returning to Edit preserves or intentionally restores the selected tab state.

## Required Test Shape
- Click Properties, then Layers, and assert `panel_tab`, selected tab metadata, Layers controls present, and Properties/History-only controls absent.
- Click History, then Layers, and assert the same Layers-specific state.
- Create unsaved document state, click Layers repeatedly, and assert document layers, active layer, dirty state, history length, and history index remain stable.
- Switch to another persona and back to Edit, then assert the product-defined tab persistence/default behavior.

## Required Changes
- Expose selected state for Edit panel tab automation nodes.
- Add Layers tab E2E assertions for active styling, visible content, repeated-click stability, data preservation, and persona return behavior.

## Verification
- `cargo test -p tench-pixel-design --test pixel_design_e2e edit_panel_layers_tab`
- `cargo test -p tench-pixel-design`
