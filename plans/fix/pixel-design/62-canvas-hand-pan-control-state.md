# Canvas Hand Pan Control State

## Source Plan
- `plans/pixel-design/canvas-hand-pan-control-work-plan.md`

## Gap Analysis
Hand pan stores `viewport_offset_x` / `viewport_offset_y` in viewport screen units, but `move_canvas_action` adds deltas computed from document coordinates. At zoom levels where the document-to-screen scale is not 1:1, the same pointer movement produces the wrong pan distance and can skew subsequent pointer-to-document mapping. See `apps/pixel-design/src-tauri/src/ui/mod.rs:650` and `apps/pixel-design/src-tauri/src/ui/state.rs:901`.

Pointer down rejects positions outside the current document rectangle before the active tool is considered. That prevents the Hand tool from starting a pan from empty canvas viewport space after fit, zoom, or an existing pan, even though the control is the central canvas viewport. See `apps/pixel-design/src-tauri/src/ui/mod.rs:646`.

Temporary pan via Space can become sticky because keyboard handling returns immediately for key release events and never clears `space_held`. The Space path in `begin_canvas_action` also starts panning without updating status, so the immediate acknowledgement requirement is missing for this activation path. See `apps/pixel-design/src-tauri/src/ui/mod.rs:788` and `apps/pixel-design/src-tauri/src/ui/state.rs:802`.

Finishing a pan calls `refresh_flattened()` through the generic canvas pointer-up and leave paths even though panning does not change document pixels. This violates the expected flattened-canvas refresh rule for a viewport-only interaction. See `apps/pixel-design/src-tauri/src/ui/mod.rs:672`.

The current E2E coverage performs one Hand drag and only asserts `viewport_offset_x` changed. It does not verify normal zoom versus high zoom or scroll zoom, pointer-to-document coordinate accuracy after pan, viewport-background start positions, Space temporary pan release, unchanged history, unchanged document pixels, or unchanged flattened content. See `apps/pixel-design/src-tauri/tests/pixel_design_e2e.rs:473`.

## Plan Requirements Not Met
- Hand pan must use screen-space pointer deltas, or consistently convert units before updating viewport offsets.
- Hand pan must be startable from the product-defined canvas viewport bounds, not only from the currently visible document rectangle.
- Space temporary pan must clear on key release and acknowledge activation immediately.
- Viewport-only panning must repaint without rebuilding flattened document pixels.
- Tests must verify pan behavior at normal zoom, high zoom, and after scroll zoom.
- Tests must verify pointer-to-document coordinates remain accurate after panning.
- Tests must verify Hand pan leaves history, dirty state, document pixels, and flattened content unchanged.

## Required Test Shape
- Add a Pixel Design UI automation test that pans with Hand from the document area and from empty viewport background, then asserts the viewport offset changes as expected.
- Repeat Hand pan after zoom-in and scroll-zoom paths and assert screen-space drag distance maps predictably to viewport offset.
- After panning, perform a pixel-affecting tool action at a known canvas fraction and assert the affected document coordinate is correct.
- Press Space while another tool is active, pan, release Space, and assert the next drag returns to the original tool behavior.
- Capture history length, dirty state, representative pixels, and flattened capture before/after Hand pan and assert they do not change.

## Required Changes
- Pass viewport pointer positions into the panning path, or store pan starts in screen-space coordinates, so viewport offsets are updated in the same unit they are rendered with.
- Route Hand and Space temporary pan pointer-down hit testing through the canvas viewport bounds while keeping document-bound hit testing for document-editing tools.
- Handle Space key release and set a status message when temporary pan begins.
- Skip `refresh_flattened()` for pure pan completion while still requesting paint for the new viewport.

## Verification
- `cargo test -p tench-pixel-design --test pixel_design_e2e canvas_hand_pan`
- `cargo test -p tench-pixel-design`
