# Canvas Text Placement Control State

## Source Plan
- `plans/pixel-design/canvas-text-placement-control-work-plan.md`

## Gap Analysis
The text input overlay automation node is exposed with the full document bounds instead of the insertion overlay bounds or document-space text position. Tests can detect that the overlay exists and read draft text, but they cannot verify the insertion point after placement, zoom, pan, or a second click. See `apps/pixel-design/src-tauri/src/ui/mod.rs:1422` and `apps/pixel-design/src-tauri/src/ui/canvas.rs:86`.

Text commit writes directly to `active_layer_mut()` without checking whether the active layer is locked or hidden. A locked layer can still receive rasterized text, and hidden-layer behavior is not explicitly defined or surfaced. See `apps/pixel-design/src-tauri/src/ui/state.rs:1124` and `crates/pixel-core/src/document.rs:10`.

Text commit ignores the current selection. If text should be clipped to the selection, blocked outside the selection, or deliberately independent of selection, that rule is not defined or verified. See `apps/pixel-design/src-tauri/src/ui/state.rs:1129`.

The current E2E coverage clicks once, types `AB`, backspaces, presses Enter, and only asserts status text. It does not verify overlay coordinates, draft clearing on a second placement, committed pixels, foreground color, zoom/pan coordinate accuracy, locked or hidden layers, selection behavior, dirty state, flattened-canvas refresh, or undo restoration. See `apps/pixel-design/src-tauri/tests/pixel_design_e2e.rs:404`.

## Plan Requirements Not Met
- Text overlay automation must expose the actual insertion point and overlay bounds.
- Text commit must not mutate locked layers.
- Text commit must define and enforce behavior for hidden active layers.
- Text commit must define how current selection affects text placement.
- Tests must verify a second placement creates an independent insertion point and clears prior draft text.
- Tests must verify Text placement after zooming and panning.
- Tests must verify undo restores the previous pixel state after committed text.

## Required Test Shape
- Add a Pixel Design UI automation test that clicks a known canvas point and asserts the text overlay reports the matching insertion coordinates and draft value.
- Type draft text, click another canvas point, and assert the draft is cleared and the insertion point moves.
- Commit text with Enter and assert representative pixels change at the expected document coordinates using the foreground color.
- Lock the active layer, attempt text commit, and assert pixels, history, dirty state, status, and flattened output follow the blocked-action rule.
- Hide the active layer, attempt text commit, and assert the product-defined hidden-layer behavior.
- Create a selection, place text, and assert the product-defined selection behavior.
- Zoom and pan before placement, then assert pointer-to-document mapping through overlay metadata and committed pixels.
- Undo after committed text and assert pixels, history, dirty state, flattened output, and visible capture restore.

## Required Changes
- Expose text insertion coordinates and overlay bounds through `pd.auto.text_input_overlay`.
- Gate text rasterization on locked-layer and product-defined hidden-layer rules before creating history.
- Define and implement Text selection behavior.
- Add second-placement, coordinate, pixel, color, lock, visibility, selection, zoom/pan, undo, and flattened-refresh tests.

## Verification
- `cargo test -p tench-pixel-design --test pixel_design_e2e canvas_text_placement`
- `cargo test -p tench-pixel-design`
