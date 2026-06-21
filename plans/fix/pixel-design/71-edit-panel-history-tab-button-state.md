# History Panel Tab Button State

## Source Plan
- `plans/pixel-design/edit-panel-history-tab-button-work-plan.md`

## Gap Analysis
History tab buttons are exposed as generic automation buttons without selected or active-state metadata. The renderer does draw active tab styling from `panel_tab`, but tests cannot directly verify that the History tab button is highlighted through the UI tree. See `apps/pixel-design/src-tauri/src/ui/mod.rs:1055` and `apps/pixel-design/src-tauri/src/ui/layers.rs:31`.

The current E2E coverage clicks History once and asserts History controls are present. It does not assert `panel_tab == PanelTab::History`, active tab styling, absence of Layers/Properties-only controls, repeated-click stability, unsaved document data preservation, or persona switch/return behavior. See `apps/pixel-design/src-tauri/tests/pixel_design_e2e.rs:337`.

## Plan Requirements Not Met
- History tab automation must expose active/selected tab state or equivalent metadata.
- Tests must verify only History-specific content is visible after clicking History.
- Tests must verify repeated History clicks do not reset active document, active layer, or history state.
- Tests must verify switching personas and returning to Edit preserves or intentionally restores the selected tab state.

## Required Test Shape
- Click History from Layers and Properties, then assert `panel_tab`, selected tab metadata, History controls present, and Layers/Properties-only controls absent.
- Create unsaved document state, click History repeatedly, and assert document layers, active layer, dirty state, history length, and history index remain stable.
- Switch to another persona and back to Edit, then assert the product-defined tab persistence/default behavior.

## Required Changes
- Expose selected state for Edit panel tab automation nodes.
- Add History tab E2E assertions for active styling, visible content, repeated-click stability, data preservation, and persona return behavior.

## Verification
- `cargo test -p tench-pixel-design --test pixel_design_e2e edit_panel_history_tab`
- `cargo test -p tench-pixel-design`
