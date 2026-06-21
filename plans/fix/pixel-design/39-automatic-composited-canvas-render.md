# Automatic Composited Canvas Render

## Source Plan
- `plans/pixel-design/automatic-composited-canvas-render-work-plan.md`

## Gap Analysis
The app keeps `state.composited_image` and many UI handlers call `refresh_flattened` after document mutations, but there is no central dirty/invalidation mechanism and painting does not rebuild a stale cache before drawing. Document mutations outside those handler paths can leave `composited_image` stale because `paint_canvas_viewport` renders the cache whenever it is `Some`. See `apps/pixel-design/src-tauri/src/ui/mod.rs:53` and `apps/pixel-design/src-tauri/src/ui/canvas.rs:35`.

Open-file rendering is not exercised through the UI flow. The Open button only sets `pending_file_action = Some(FileAction::Open)`, while `load_image` is a separate method that refreshes the cache when called directly. See `apps/pixel-design/src-tauri/src/ui/mod.rs:236` and `apps/pixel-design/src-tauri/src/ui/mod.rs:40`.

The `pd.auto.composited_canvas` automation node only reports the image node bounds and no cache generation, layer count, document size, or source mutation information. See `apps/pixel-design/src-tauri/src/ui/mod.rs:1390`.

The current E2E coverage verifies a brush stroke changes a capture and that the composited canvas node is present. It does not verify composited pixels after layer reorder, opacity change, visibility change, crop, open/load, alternate mutation paths, persona switches, or resize. See `apps/pixel-design/src-tauri/tests/pixel_design_e2e.rs:373`.

## Plan Requirements Not Met
- Document mutations must centrally invalidate the composited cache or trigger a rebuild before paint.
- Open/load file flow must be tested to verify the canvas updates without manual refresh.
- Automation metadata must expose enough composited-cache state to verify rebuilds.
- Tests must verify canvas pixels after brush stroke, layer reorder, opacity change, visibility change, crop, and open/load.
- Tests must verify equivalent mutations through alternate paths produce identical composited output.
- Tests must verify composited canvas remains correct after persona switches and viewport resize.

## Required Test Shape
- Add a Pixel Design UI automation test that captures before/after pixels for brush stroke, layer reorder, opacity change, visibility toggle, crop, and image load.
- Assert `pd.auto.composited_canvas` metadata changes when the flattened cache is rebuilt and remains tied to the current document/layers.
- Exercise equivalent state changes through UI buttons and keyboard paths, then assert matching composited output.
- Switch personas and resize the viewport after mutations and assert composited canvas pixels remain correct.
- Use `CaptureAssertions` helpers to verify PNG validity, size, nonblank output, and expected visual changes.

## Required Changes
- Add a canonical composited-cache dirty flag or generation that document mutations update.
- Rebuild the flattened cache before painting or automation capture when it is dirty.
- Wire Open/load UI automation to a fixture-backed load path and verify cache refresh.
- Expose composited-cache metadata through automation.
- Add composited canvas render E2E and state tests.

## Verification
- `cargo test -p tench-pixel-design --test pixel_design_e2e automatic_composited_canvas_render`
- `cargo test -p tench-pixel-design`
