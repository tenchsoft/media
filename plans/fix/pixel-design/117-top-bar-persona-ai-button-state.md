# AI Persona Button State

## Source Plan
- `plans/pixel-design/top-bar-persona-ai-button-work-plan.md`

## Gap Analysis
Persona buttons are exposed as generic automation buttons without selected state. The renderer highlights the active persona visually, but tests cannot verify that only AI is active through stable UI metadata. See `apps/pixel-design/src-tauri/src/ui/mod.rs:966` and `apps/pixel-design/src-tauri/src/ui/toolbar.rs:20`.

AI tool strip buttons are exposed as generic buttons without active state for the currently expanded AI tool. The visual AI tool strip exists, but automation cannot verify the active AI context from the tool strip itself. See `apps/pixel-design/src-tauri/src/ui/mod.rs:1004` and `apps/pixel-design/src-tauri/src/ui/toolbar.rs:167`.

The current E2E coverage clicks AI and asserts AI panel/tool nodes are present. It does not verify status text, active persona highlight, absence of other persona panel controls, dirty/history/zoom/document preservation, repeated-click stability, or active AI tool-strip state. See `apps/pixel-design/src-tauri/tests/pixel_design_e2e.rs:529`.

## Plan Requirements Not Met
- AI persona automation must expose selected/highlight state.
- AI tool-strip automation must expose active AI tool state.
- Tests must verify status is `AI workspace`.
- Tests must verify only AI-specific right-panel content is visible.
- Tests must verify dirty state, history, zoom, and active document remain unchanged.
- Tests must verify repeated AI clicks do not duplicate panels or reset tool settings.

## Required Test Shape
- Click `pd.top.persona.ai` from each other persona and assert persona, status, selected persona metadata, AI panel nodes, absence of other persona-only nodes, and AI tool-strip active metadata.
- Create dirty state and non-default zoom/history, click AI, and assert document, dirty state, history, zoom, and active layer remain unchanged.
- Click AI repeatedly and assert UI node counts, selected state, active document, expanded AI tool, and prompt/job state remain stable.

## Required Changes
- Expose selected state on persona automation nodes.
- Expose selected state on AI tool-strip automation nodes.
- Add AI persona E2E tests for selected state, status, right panel, tool strip, repeated-click stability, and document-state preservation.

## Verification
- `cargo test -p tench-pixel-design --test pixel_design_e2e top_bar_persona_ai`
- `cargo test -p tench-pixel-design`
