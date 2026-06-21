# Fill Tool Button State

## Source Plan
- `plans/pixel-design/tool-strip-fill-button-work-plan.md`

## Gap Analysis
Tool buttons are exposed as generic automation buttons without selected state. `pd.auto.active_control_highlight` reports the active tool label, but its bounds are fixed to the first tool slot instead of the active tool's slot, so automation cannot verify the Fill button highlight location. See `apps/pixel-design/src-tauri/src/ui/mod.rs:998` and `apps/pixel-design/src-tauri/src/ui/mod.rs:1369`.

`set_active_tool` closes text input, blend dropdowns, and layer context menus, but it does not close modal state such as the color picker. Because modal hit testing can fall through outside the modal, selecting Fill while a modal-like state is open is not guaranteed to close incompatible transient UI cleanly. See `apps/pixel-design/src-tauri/src/ui/state.rs:640` and `apps/pixel-design/src-tauri/src/ui/mod.rs:181`.

The current E2E coverage clicks Fill and only asserts `active_tool == Tool::Fill`. It does not verify status text, selected/highlight state, context chips, transient state cleanup, persona return behavior, or that the next canvas gesture performs flood fill rather than using the previous tool. See `apps/pixel-design/src-tauri/tests/pixel_design_e2e.rs:193`.

## Plan Requirements Not Met
- Fill tool automation must expose selected/highlight state on the actual Fill button.
- Active control highlight bounds must follow the active tool's slot.
- Selecting Fill must close or block incompatible modal-like transient state according to product rules.
- Tests must verify Fill context chips appear after selection.
- Tests must verify the next canvas gesture performs Fill behavior after switching from another tool.
- Tests must verify Fill state renders consistently after switching away and back to Edit.

## Required Test Shape
- Click `pd.tool.fill` from another tool and assert active tool, status, selected button metadata, active highlight bounds, and context chips.
- Open text input, dropdown, and modal-like states, select Fill, and assert incompatible transient state is closed or blocked according to product rules.
- Switch from a non-fill tool to Fill, click the canvas, and assert fill pixels/history/dirty state are produced.
- Switch personas away from Edit and back, then assert Fill selection and highlight render consistently if Fill remains active.

## Required Changes
- Expose selected state on tool-strip automation nodes.
- Make `pd.auto.active_control_highlight` report the active tool's actual bounds.
- Define and implement modal/transient cleanup behavior for tool selection.
- Add Fill Tool Button E2E tests for highlight, status, context chips, transient cleanup, post-selection fill gesture, and persona return.

## Verification
- `cargo test -p tench-pixel-design --test pixel_design_e2e tool_strip_fill`
- `cargo test -p tench-pixel-design`
