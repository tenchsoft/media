# Brush Tool Button State

## Source Plan
- `plans/pixel-design/tool-strip-brush-button-work-plan.md`

## Gap Analysis
Tool buttons are exposed as generic automation buttons without selected state. `pd.auto.active_control_highlight` reports the active tool label, but its bounds are fixed to the first tool slot instead of the active tool's slot, so automation cannot verify the Brush button highlight location. See `apps/pixel-design/src-tauri/src/ui/mod.rs:998` and `apps/pixel-design/src-tauri/src/ui/mod.rs:1369`.

`set_active_tool` closes text input, blend dropdowns, and layer context menus, but it does not close modal state such as the color picker. Because modal hit testing can fall through outside the modal, selecting Brush while a modal-like state is open is not guaranteed to close incompatible transient UI cleanly. See `apps/pixel-design/src-tauri/src/ui/state.rs:640` and `apps/pixel-design/src-tauri/src/ui/mod.rs:181`.

The current E2E coverage clicks Brush and only asserts `active_tool == Tool::Brush`. It does not verify status text, selected/highlight state, brush context chips, brush presets/properties visibility, transient state cleanup, persona return behavior, or that the next canvas gesture uses Brush rather than the previous tool. See `apps/pixel-design/src-tauri/tests/pixel_design_e2e.rs:193`.

## Plan Requirements Not Met
- Brush tool automation must expose selected/highlight state on the actual Brush button.
- Active control highlight bounds must follow the active tool's slot.
- Selecting Brush must close or block incompatible modal-like transient state according to product rules.
- Tests must verify Brush context chips and brush presets/properties appear after selection.
- Tests must verify the next canvas gesture performs Brush stroke behavior after switching from another tool.
- Tests must verify Brush state renders consistently after switching away and back to Edit.

## Required Test Shape
- Click `pd.tool.brush` from another tool and assert active tool, status, selected button metadata, active highlight bounds, context chips, and brush presets/properties.
- Open text input, dropdown, and modal-like states, select Brush, and assert incompatible transient state is closed or blocked according to product rules.
- Switch from a non-brush tool to Brush, drag on canvas, and assert Brush pixels/history/dirty state are produced.
- Switch personas away from Edit and back, then assert Brush selection and highlight render consistently if Brush remains active.

## Required Changes
- Expose selected state on tool-strip automation nodes.
- Make `pd.auto.active_control_highlight` report the active tool's actual bounds.
- Define and implement modal/transient cleanup behavior for tool selection.
- Add Brush Tool Button E2E tests for highlight, status, context chips, presets/properties, transient cleanup, post-selection brush gesture, and persona return.

## Verification
- `cargo test -p tench-pixel-design --test pixel_design_e2e tool_strip_brush`
- `cargo test -p tench-pixel-design`
