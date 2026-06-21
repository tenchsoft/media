# Text Tool Button State

## Source Plan
- `plans/pixel-design/tool-strip-text-button-work-plan.md`

## Gap Analysis
Tool buttons are exposed as generic automation buttons without selected state. `pd.auto.active_control_highlight` reports the active tool label, but its bounds are fixed to the first tool slot instead of the active tool's slot, so automation cannot verify the Text button highlight location. See `apps/pixel-design/src-tauri/src/ui/mod.rs:998` and `apps/pixel-design/src-tauri/src/ui/mod.rs:1369`.

`set_active_tool` closes text input only when the next tool is not Text, and does not close modal state such as the color picker. Selecting Text while an existing text insertion overlay or modal-like state is open therefore has no clearly defined cleanup behavior. See `apps/pixel-design/src-tauri/src/ui/state.rs:640` and `apps/pixel-design/src-tauri/src/ui/mod.rs:181`.

The current E2E coverage clicks Text and only asserts `active_tool == Tool::Text`. It does not verify status text, selected/highlight state, context chips, transient state cleanup or preservation rules, persona return behavior, or that the next canvas gesture creates a text insertion overlay rather than using the previous tool. See `apps/pixel-design/src-tauri/tests/pixel_design_e2e.rs:193`.

## Plan Requirements Not Met
- Text tool automation must expose selected/highlight state on the actual Text button.
- Active control highlight bounds must follow the active tool's slot.
- Selecting Text must define and enforce cleanup or preservation behavior for existing text overlay and modal-like transient state.
- Tests must verify Text context chips appear after selection.
- Tests must verify the next canvas gesture performs Text placement after switching from another tool.
- Tests must verify Text state renders consistently after switching away and back to Edit.

## Required Test Shape
- Click `pd.tool.text` from another tool and assert active tool, status, selected button metadata, active highlight bounds, and context chips.
- Open text insertion, dropdown, and modal-like states, select Text, and assert transient state follows the product-defined cleanup or preservation rule.
- Switch from a non-text tool to Text, click the canvas, and assert text insertion overlay position/value/status follow Text behavior.
- Switch personas away from Edit and back, then assert Text selection and highlight render consistently if Text remains active.

## Required Changes
- Expose selected state on tool-strip automation nodes.
- Make `pd.auto.active_control_highlight` report the active tool's actual bounds.
- Define and implement transient cleanup/preservation behavior for Text tool selection.
- Add Text Tool Button E2E tests for highlight, status, context chips, transient state rules, post-selection text placement, and persona return.

## Verification
- `cargo test -p tench-pixel-design --test pixel_design_e2e tool_strip_text`
- `cargo test -p tench-pixel-design`
