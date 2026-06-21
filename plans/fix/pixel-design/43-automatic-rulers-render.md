# Automatic Rulers Render

## Source Plan
- `plans/pixel-design/automatic-rulers-render-work-plan.md`

## Gap Analysis
Ruler rendering draws horizontal and vertical ruler backgrounds, but it only draws horizontal ticks and labels. There is no vertical tick/label loop even though the plan requires horizontal and vertical rulers around the viewport. See `apps/pixel-design/src-tauri/src/ui/canvas.rs:147`.

Tick spacing is not derived from the actual document dimensions and zoom state. `paint_rulers` computes zoom from `doc.width()` and `state_document_width(doc, viewport)`, where `state_document_width` derives a synthetic width from viewport size, so the threshold can be disconnected from the document zoom value. See `apps/pixel-design/src-tauri/src/ui/canvas.rs:160` and `apps/pixel-design/src-tauri/src/ui/canvas.rs:197`.

The `pd.auto.rulers` automation node exposes no enabled state, tick step, zoom, document rect, or vertical/horizontal tick counts. See `apps/pixel-design/src-tauri/src/ui/mod.rs:1432`.

The current E2E coverage toggles rulers and only asserts `pd.auto.rulers` is present. It does not verify tick density below/above thresholds, vertical ticks, resize behavior, alternate paths, persona switches, or capture pixels. See `apps/pixel-design/src-tauri/tests/pixel_design_e2e.rs:509`.

## Plan Requirements Not Met
- Vertical ruler ticks and labels must render.
- Ruler tick spacing must derive from canonical document rect/dimensions and zoom state.
- Automation metadata must expose ruler enabled state, tick step, zoom, document rect, and tick counts.
- Tests must verify tick density changes below and above threshold values.
- Tests must verify rulers remain correct after resize and persona switches.
- Tests must verify equivalent state changes from alternate paths produce identical ruler output.

## Required Test Shape
- Add a Pixel Design UI automation test that enables rulers and samples horizontal and vertical tick pixels from the capture.
- Set zoom below and above threshold values and assert ruler metadata plus sampled tick density changes as expected.
- Resize the viewport and assert tick positions stay aligned to the document rect.
- Switch personas and assert ruler metadata and capture remain correct.
- Use capture assertions to verify PNG validity, size, nonblank output, and expected visual changes.

## Required Changes
- Add vertical ruler tick and label rendering.
- Rework ruler spacing to use canonical document dimensions and zoom.
- Expose ruler tick metadata through automation.
- Add rulers render E2E and state tests.

## Verification
- `cargo test -p tench-pixel-design --test pixel_design_e2e automatic_rulers_render`
- `cargo test -p tench-pixel-design`
