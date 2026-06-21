# Automatic Checkerboard Transparency Render

## Source Plan
- `plans/pixel-design/automatic-checkerboard-transparency-render-work-plan.md`

## Gap Analysis
Canvas painting draws a checkerboard behind the viewport and draws a smaller checkerboard inside the document only when no composited image exists. Once a composited image exists, transparent document pixels rely on the viewport checkerboard behind the image, but there is no automated verification that transparency inside the document reveals the expected pattern. See `apps/pixel-design/src-tauri/src/ui/canvas.rs:28` and `apps/pixel-design/src-tauri/src/ui/canvas.rs:35`.

The `pd.auto.checkerboard` automation node has no value describing checkerboard cell size, origin, document rect, or whether transparent document pixels are currently visible. See `apps/pixel-design/src-tauri/src/ui/mod.rs:1333`.

The current E2E coverage only asserts the checkerboard node exists and performs an eraser stroke by status message. It does not verify checkerboard pixels after opening/creating a transparent document, erasing pixels, zooming, switching personas, or resizing. See `apps/pixel-design/src-tauri/tests/pixel_design_e2e.rs:170` and `apps/pixel-design/src-tauri/tests/pixel_design_e2e.rs:391`.

## Plan Requirements Not Met
- Tests must verify checkerboard is visible through transparent document pixels, not only behind an empty viewport.
- Tests must verify checkerboard cell alignment across zoom in/out.
- Tests must verify erasing pixels reveals checkerboard non-destructively.
- Tests must verify checkerboard remains correct after persona switches and viewport resize.
- Automation metadata must expose enough checkerboard layout data to assert cell size/origin and document intersection.
- Visual captures must verify checkerboard pixels and nonblank rendering.

## Required Test Shape
- Add a Pixel Design UI automation test that creates or seeds transparent pixels, captures the canvas, and samples alternating checkerboard colors inside the document rect.
- Erase a known area and assert the document pixels become transparent while the captured canvas shows checkerboard behind that area.
- Zoom in and out, then assert checkerboard cells remain aligned according to the product-defined origin/cell-size rule.
- Switch personas and resize the viewport, then assert checkerboard metadata and sampled pixels remain correct.
- Use `CaptureAssertions` helpers to verify PNG validity, size, nonblank output, and expected visual changes.

## Required Changes
- Expose checkerboard cell size, origin, document rect, and transparent-region state through automation metadata.
- Add checkerboard transparency render E2E tests with capture sampling.

## Verification
- `cargo test -p tench-pixel-design --test pixel_design_e2e automatic_checkerboard_transparency_render`
- `cargo test -p tench-pixel-design`
