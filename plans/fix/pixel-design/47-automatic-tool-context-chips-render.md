# Automatic Tool Context Chips Render

## Source Plan
- `plans/pixel-design/automatic-tool-context-chips-render-work-plan.md`

## Gap Analysis
The top bar renders context chips from `active_tool.context_options(brush_size, brush_opacity, brush_hardness)` and takes the first three options. However, the automation node `pd.auto.tool_context_chips` only exposes the active tool label, not the chip labels or values that are actually rendered. See `apps/pixel-design/src-tauri/src/ui/toolbar.rs:82` and `apps/pixel-design/src-tauri/src/ui/mod.rs:1376`.

The current E2E coverage asserts the context-chip node exists and checks underlying state changes for active tools and brush settings. It does not verify rendered chip text for Brush, Eraser, Text, Fill, Move, or updates after changing brush size/opacity/hardness through UI and keyboard paths. See `apps/pixel-design/src-tauri/tests/pixel_design_e2e.rs:178` and `apps/pixel-design/src-tauri/tests/pixel_design_e2e.rs:314`.

## Plan Requirements Not Met
- Automation metadata must expose the rendered context chip labels and values.
- Tests must verify chips for Brush, Eraser, Text, Fill, and Move.
- Tests must verify brush setting changes update the chip text immediately.
- Tests must verify equivalent active-tool changes through toolbar and keyboard paths produce identical chip output.
- Tests must verify chips remain correct after persona switches and viewport resize.
- Visual captures must verify chip rendering remains visible and non-overlapping.

## Required Test Shape
- Add a Pixel Design UI automation test that selects Brush, Eraser, Text, Fill, and Move and asserts the exact chip values exposed by `pd.auto.tool_context_chips`.
- Change brush size, opacity, and hardness through properties controls and keyboard shortcuts, then assert chip values update.
- Select the same tool through click and shortcut paths and assert identical chip metadata.
- Switch personas and resize the viewport, then assert chip metadata and capture remain correct.
- Use capture assertions to verify valid, nonblank output and visible chip changes.

## Required Changes
- Expose context chip labels/values through automation metadata or child nodes.
- Add tool context chip render E2E tests covering tool switches, setting changes, alternate paths, and resize.

## Verification
- `cargo test -p tench-pixel-design --test pixel_design_e2e automatic_tool_context_chips_render`
- `cargo test -p tench-pixel-design`
