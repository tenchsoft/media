# Properties Panel Tab Button State

## Source Plan
- `plans/pixel-design/edit-panel-properties-tab-button-work-plan.md`

## Gap Analysis
Properties tab buttons are exposed as generic automation buttons without selected or active-state metadata. The renderer draws active tab styling from `panel_tab`, but tests cannot directly verify that the Properties tab button is highlighted through the UI tree. See `apps/pixel-design/src-tauri/src/ui/mod.rs:1055` and `apps/pixel-design/src-tauri/src/ui/layers.rs:31`.

The current E2E coverage clicks Properties once and asserts property controls are present, then interacts with those controls. It does not assert `panel_tab == PanelTab::Properties`, active tab styling, absence of Layers/History-only controls, clicking Properties from History, repeated-click stability, unsaved document data preservation, or persona switch/return behavior. See `apps/pixel-design/src-tauri/tests/pixel_design_e2e.rs:308`.

## Plan Requirements Not Met
- Properties tab automation must expose active/selected tab state or equivalent metadata.
- Tests must verify clicking Properties from each other Edit panel tab shows only Properties-specific content.
- Tests must verify repeated Properties clicks do not reset active document, active layer, or history state.
- Tests must verify switching personas and returning to Edit preserves or intentionally restores the selected tab state.

## Required Test Shape
- Click Layers, then Properties, and assert `panel_tab`, selected tab metadata, Properties controls present, and Layers/History-only controls absent.
- Click History, then Properties, and assert the same Properties-specific state.
- Create unsaved document state, click Properties repeatedly, and assert document layers, active layer, dirty state, history length, and history index remain stable.
- Switch to another persona and back to Edit, then assert the product-defined tab persistence/default behavior.

## Required Changes
- Expose selected state for Edit panel tab automation nodes.
- Add Properties tab E2E assertions for active styling, visible content, repeated-click stability, data preservation, and persona return behavior.

## Verification
- `cargo test -p tench-pixel-design --test pixel_design_e2e edit_panel_properties_tab`
- `cargo test -p tench-pixel-design`
