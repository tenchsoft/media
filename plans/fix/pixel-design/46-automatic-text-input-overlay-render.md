# Automatic Text Input Overlay Render

## Source Plan
- `plans/pixel-design/automatic-text-input-overlay-render-work-plan.md`

## Gap Analysis
The canvas paints an in-place text overlay from `show_text_input`, `text_pos`, and `text_input`, and Enter commits while Escape hides the overlay through `cancel_modal_action`. However, `pd.auto.text_input_overlay` uses the full document rect as its bounds instead of the actual overlay rect at the chosen text coordinate. See `apps/pixel-design/src-tauri/src/ui/canvas.rs:85` and `apps/pixel-design/src-tauri/src/ui/mod.rs:1422`.

The painted placeholder is `"Type text..."` when the draft is empty, but automation exposes an empty value instead of the placeholder or explicit placeholder state. See `apps/pixel-design/src-tauri/src/ui/canvas.rs:92` and `apps/pixel-design/src-tauri/src/ui/mod.rs:1427`.

Escape hides the overlay but `cancel_modal_action` does not clear `text_input`, so the cancelled draft is retained in hidden state until the next placement clears it. See `apps/pixel-design/src-tauri/src/ui/state.rs:1147`.

The current E2E coverage places text, asserts the overlay node exists, types characters, backspaces, presses Enter, and checks the final status. It does not verify draft text after typing/backspace, placeholder rendering, overlay disappearance after Enter/Escape, no document commit on Escape, overlay position alignment after zoom/pan, alternate paths, persona switches, or resize. See `apps/pixel-design/src-tauri/tests/pixel_design_e2e.rs:403`.

## Plan Requirements Not Met
- Automation metadata must expose the actual text overlay rect and document coordinate.
- Automation metadata must distinguish placeholder text from draft text.
- Escape cancellation must clear or explicitly define retained hidden draft state.
- Tests must verify typed and backspaced draft text in the overlay.
- Tests must verify Enter commits pixels and hides the overlay.
- Tests must verify Escape hides the overlay without committing document pixels.
- Tests must verify overlay alignment after zoom/pan, persona switches, and resize.

## Required Test Shape
- Add a Pixel Design UI automation test that places text and asserts overlay rect, document coordinate, placeholder value, and textbox metadata.
- Type characters and Backspace, then assert `pd.auto.text_input_overlay` value updates exactly.
- Press Enter and assert the overlay node disappears, document pixels change, and status is "Text added".
- Place text again, type a draft, press Escape, and assert overlay disappears, draft state is cleared or product-defined, and document pixels do not change.
- Zoom/pan, switch personas, and resize while the overlay is active, then assert overlay metadata and capture remain aligned.

## Required Changes
- Expose actual overlay bounds, text position, placeholder state, and draft text through automation.
- Clear or explicitly define `text_input` state on Escape cancellation.
- Add text input overlay render E2E and state tests.

## Verification
- `cargo test -p tench-pixel-design --test pixel_design_e2e automatic_text_input_overlay_render`
- `cargo test -p tench-pixel-design`
