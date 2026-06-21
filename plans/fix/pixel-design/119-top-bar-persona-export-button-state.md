# Export Persona Button State

## Source Plan
- `plans/pixel-design/top-bar-persona-export-button-work-plan.md`

## Gap Analysis
Persona buttons are exposed as generic automation buttons without selected state. The renderer highlights the active persona visually, but tests cannot verify that only Export is active through stable UI metadata. See `apps/pixel-design/src-tauri/src/ui/mod.rs:966` and `apps/pixel-design/src-tauri/src/ui/toolbar.rs:20`.

The left tool strip has Edit and AI contexts only. When Export is active, the tool strip does not expose an Export-specific context, even though the plan requires the left tool strip to switch to the Export context. See `apps/pixel-design/src-tauri/src/ui/toolbar.rs:156`.

The current E2E coverage clicks Export and asserts the export panel controls are present. It does not verify status text, active persona highlight, absence of other persona panel controls, dirty/history/zoom/document preservation, repeated-click stability, or tool strip Export context. See `apps/pixel-design/src-tauri/tests/pixel_design_e2e.rs:628`.

## Plan Requirements Not Met
- Export persona automation must expose selected/highlight state.
- The left tool strip must define and render an Export workspace context.
- Tests must verify status is `Export workspace`.
- Tests must verify only Export-specific right-panel content is visible.
- Tests must verify dirty state, history, zoom, and active document remain unchanged.
- Tests must verify repeated Export clicks do not duplicate panels or reset export settings.

## Required Test Shape
- Click `pd.top.persona.export` from each other persona and assert persona, status, selected button metadata, Export panel nodes, absence of other persona-only nodes, and Export tool-strip context.
- Create dirty state and non-default zoom/history, click Export, and assert document, dirty state, history, zoom, active layer, and export settings remain unchanged.
- Click Export repeatedly and assert UI node counts, selected state, active document, and export settings remain stable.

## Required Changes
- Expose selected state on persona automation nodes.
- Define and render the Export left tool-strip context.
- Add Export persona E2E tests for selected state, status, right panel, left context, repeated-click stability, and document-state preservation.

## Verification
- `cargo test -p tench-pixel-design --test pixel_design_e2e top_bar_persona_export`
- `cargo test -p tench-pixel-design`
