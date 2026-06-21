# Adjust Persona Button State

## Source Plan
- `plans/pixel-design/top-bar-persona-adjust-button-work-plan.md`

## Gap Analysis
Persona buttons are exposed as generic automation buttons without selected state. The renderer highlights the active persona visually, but tests cannot verify that only Adjust is active through stable UI metadata. See `apps/pixel-design/src-tauri/src/ui/mod.rs:966` and `apps/pixel-design/src-tauri/src/ui/toolbar.rs:20`.

The left tool strip has Edit and AI contexts only. When Adjust is active, the tool strip does not expose an Adjust-specific context, even though the plan requires the left tool strip to switch to the Adjust context. See `apps/pixel-design/src-tauri/src/ui/toolbar.rs:156`.

The current E2E coverage clicks Adjust and asserts persona state in one loop, then separately asserts the Adjust panel controls are present. It does not verify status text, active persona highlight, absence of other persona panel controls, dirty/history/zoom/document preservation, repeated-click stability, or tool strip Adjust context. See `apps/pixel-design/src-tauri/tests/pixel_design_e2e.rs:183` and `apps/pixel-design/src-tauri/tests/pixel_design_e2e.rs:586`.

## Plan Requirements Not Met
- Adjust persona automation must expose selected/highlight state.
- The left tool strip must define and render an Adjust workspace context.
- Tests must verify status is `Adjust workspace`.
- Tests must verify only Adjust-specific right-panel content is visible.
- Tests must verify dirty state, history, zoom, and active document remain unchanged.
- Tests must verify repeated Adjust clicks do not duplicate panels or reset tool settings.

## Required Test Shape
- Click `pd.top.persona.adjust` from each other persona and assert persona, status, selected button metadata, Adjust panel nodes, absence of other persona-only nodes, and Adjust tool-strip context.
- Create dirty state and non-default zoom/history, click Adjust, and assert document, dirty state, history, zoom, and active layer remain unchanged.
- Click Adjust repeatedly and assert UI node counts, selected state, active document, and tool settings remain stable.

## Required Changes
- Expose selected state on persona automation nodes.
- Define and render the Adjust left tool-strip context.
- Add Adjust persona E2E tests for selected state, status, right panel, left context, repeated-click stability, and document-state preservation.

## Verification
- `cargo test -p tench-pixel-design --test pixel_design_e2e top_bar_persona_adjust`
- `cargo test -p tench-pixel-design`
