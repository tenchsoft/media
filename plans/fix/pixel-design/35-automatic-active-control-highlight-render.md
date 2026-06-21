# Automatic Active Control Highlight Render

## Source Plan
- `plans/pixel-design/automatic-active-control-highlight-render-work-plan.md`

## Gap Analysis
Renderers derive most highlight colors from canonical state, but automation metadata does not expose selected/active state for the individual controls. Buttons are created with no value and fixed `focused = false`, so tests cannot directly verify active persona, tool, AI tool, panel tab, layer row, brush preset, or adjust preset highlight state through the UI tree. See `apps/pixel-design/src-tauri/src/ui/mod.rs:1501`.

The aggregate `pd.auto.active_control_highlight` node only reports `active_tool.label()`. It does not represent `persona`, `expanded_ai`, `panel_tab`, active layer, `brush_preset`, or `active_adjust`, even though those groups are part of this automatic render requirement. See `apps/pixel-design/src-tauri/src/ui/mod.rs:1369`.

The current E2E coverage clicks controls and asserts underlying state fields, but it does not verify visual highlight output, exactly-one active item per mutually exclusive group, identical results across alternate user paths, or persistence after persona switches/resizes. See `apps/pixel-design/src-tauri/tests/pixel_design_e2e.rs:183`.

## Plan Requirements Not Met
- Automation metadata must expose selected/active state for controls that render active highlights.
- `pd.auto.active_control_highlight` must cover every highlighted state group, not only `active_tool`.
- Tests must verify exactly one active item per mutually exclusive group.
- Tests must verify the same selected state produces identical highlight output through alternate paths.
- Tests must verify highlights remain correct after persona changes and viewport resize.
- Tests must verify visual captures change when highlights move and remain nonblank/valid.

## Required Test Shape
- Add a Pixel Design UI automation test that switches persona, tool, AI tool, panel tab, layer row, brush preset, and adjust preset controls and asserts selected/active metadata for each group.
- Assert exactly one selected node exists in each mutually exclusive group after rapid switching.
- Select the same AI tool through strip and panel paths and assert the same selected metadata and capture result.
- Change viewport size after selecting controls and assert selected metadata and visual capture remain correct.
- Use `CaptureAssertions` helpers to verify captures are valid, nonblank, and changed when the active highlight moves.

## Required Changes
- Extend automation nodes for active-highlight controls with selected/active state or value metadata.
- Expand `pd.auto.active_control_highlight` to summarize all active-highlight groups.
- Add active-highlight render E2E tests covering state, automation metadata, and visual captures.

## Verification
- `cargo test -p tench-pixel-design --test pixel_design_e2e automatic_active_control_highlight`
- `cargo test -p tench-pixel-design`
