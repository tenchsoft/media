# Automatic Canvas Layout Recompute

## Source Plan
- `plans/pixel-design/automatic-canvas-layout-recompute-work-plan.md`

## Gap Analysis
`canvas_document_rect` derives the document rect from viewport bounds, document dimensions, zoom, and viewport offset, and the same function is used by paint, pointer mapping, and automation. However, the `panels_visible` state toggled by `toggle_panels` is not used anywhere in layout, painting, or hit testing, so panel visibility changes cannot affect side-panel width or canvas framing. See `apps/pixel-design/src-tauri/src/ui/canvas.rs:7`, `apps/pixel-design/src-tauri/src/ui/state.rs:1473`, and `apps/pixel-design/src-tauri/src/ui/mod.rs:723`.

The `pd.auto.canvas_layout` automation node only exposes formatted document width and height. It does not expose document rect origin, viewport bounds, zoom, document dimensions, or viewport offset, making automatic layout and pointer-mapping assertions incomplete. See `apps/pixel-design/src-tauri/src/ui/mod.rs:1384`.

The current E2E coverage verifies zoom state changes and crop changes document width, but it does not assert recomputed canvas rect bounds, centered/framed layout, pointer mapping after zoom/crop/pan, panel toggle behavior, persona switches, alternate paths, or viewport resize behavior. See `apps/pixel-design/src-tauri/tests/pixel_design_e2e.rs:517` and `apps/pixel-design/src-tauri/tests/pixel_design_e2e.rs:487`.

## Plan Requirements Not Met
- Panel visibility or side-panel width changes must affect canvas layout consistently.
- Automation metadata must expose enough canvas layout data to verify rect position, size, zoom, viewport, and offsets.
- Tests must verify canvas remains centered/framed after resize, zoom, crop, panel toggle, and viewport offset changes.
- Tests must verify pointer mapping remains correct after layout changes.
- Tests must verify equivalent state changes from alternate paths produce identical layout.
- Tests must verify layout remains correct after persona switches and viewport resize.

## Required Test Shape
- Add a Pixel Design UI automation test that reads `pd.auto.canvas_layout` before and after resize, zoom, crop, pan, and panel toggle and asserts expected rect origin/size changes.
- Draw or crop at known fractional canvas points after zoom and resize, then assert document-space pixels or dimensions match expected pointer mapping.
- Trigger zoom through button, slider, scroll, and keyboard paths and assert equivalent layout output where the final zoom is the same.
- Switch personas after layout changes and assert the canvas rect and pointer mapping remain stable.
- Use capture assertions to verify the canvas stays valid, nonblank, and framed inside the viewport.

## Required Changes
- Wire `panels_visible` or side-panel width state into viewport/layout, painting, hit testing, and automation.
- Expand `pd.auto.canvas_layout` value or child nodes to include viewport rect, document rect, zoom, document dimensions, and viewport offset.
- Add canvas layout recompute E2E and state tests.

## Verification
- `cargo test -p tench-pixel-design --test pixel_design_e2e automatic_canvas_layout_recompute`
- `cargo test -p tench-pixel-design`
