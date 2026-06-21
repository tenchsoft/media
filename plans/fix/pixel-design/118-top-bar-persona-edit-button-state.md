# Edit Persona Button State

## Source Plan
- `plans/pixel-design/top-bar-persona-edit-button-work-plan.md`

## Gap Analysis
Persona buttons are exposed as generic automation buttons without selected state. The renderer highlights the active persona visually, but tests cannot verify that only Edit is active through stable UI metadata. See `apps/pixel-design/src-tauri/src/ui/mod.rs:966` and `apps/pixel-design/src-tauri/src/ui/toolbar.rs:20`.

Edit tool strip buttons are exposed as generic buttons without selected state for the active edit tool. The visual Edit tool strip exists, but automation cannot verify the active Edit context from the tool strip itself. See `apps/pixel-design/src-tauri/src/ui/mod.rs:990` and `apps/pixel-design/src-tauri/src/ui/toolbar.rs:156`.

The current E2E coverage clicks Edit in a persona loop and only asserts `persona == Persona::Edit`. It does not verify status text, active persona highlight, Edit panel content, absence of other persona panel controls, dirty/history/zoom/document preservation, repeated-click stability, or active edit tool-strip state. See `apps/pixel-design/src-tauri/tests/pixel_design_e2e.rs:183`.

## Plan Requirements Not Met
- Edit persona automation must expose selected/highlight state.
- Edit tool-strip automation must expose active tool state.
- Tests must verify status is `Edit workspace`.
- Tests must verify only Edit-specific right-panel content is visible.
- Tests must verify dirty state, history, zoom, and active document remain unchanged.
- Tests must verify repeated Edit clicks do not duplicate panels or reset tool settings.

## Required Test Shape
- Click `pd.top.persona.edit` from each other persona and assert persona, status, selected persona metadata, Edit panel nodes, absence of other persona-only nodes, and Edit tool-strip active metadata.
- Create dirty state and non-default zoom/history, click Edit, and assert document, dirty state, history, zoom, active layer, and active tool remain unchanged.
- Click Edit repeatedly and assert UI node counts, selected state, active document, active tool, and panel tab remain stable.

## Required Changes
- Expose selected state on persona automation nodes.
- Expose selected state on Edit tool-strip automation nodes.
- Add Edit persona E2E tests for selected state, status, right panel, tool strip, repeated-click stability, and document-state preservation.

## Verification
- `cargo test -p tench-pixel-design --test pixel_design_e2e top_bar_persona_edit`
- `cargo test -p tench-pixel-design`
