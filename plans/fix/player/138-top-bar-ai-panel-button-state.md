# AI Panel Button State

## Source Plan
- `plans/player/top-bar-ai-panel-button-work-plan.md`

## Gap Analysis
The current E2E clicks `player.top.ai` and asserts AI panel controls are present, but it does not assert `ai_panel_open`, button highlight, close toggling, drawer closure, draft retention, or unrelated playback invariants. See `apps/player/src-tauri/tests/plan_ui_e2e.rs:506`.

Opening AI clears `drawer`, and opening a drawer clears `ai_panel_open`, but the mutual-exclusion scenario of AI -> Playlist -> AI is not tested. See `apps/player/src-tauri/src/ui/app.rs:793` and `apps/player/src-tauri/src/ui/app.rs:874`.

The AI button highlight is drawn from `ai_panel_open`, but automation does not expose active/pressed state for `player.top.ai`, so tests cannot assert highlight through the UI tree. See `apps/player/src-tauri/src/ui/paint_controls.rs:69`.

Closing AI with draft prompt text is not covered, and the product-defined retention behavior is not documented in tests.

## Plan Requirements Not Met
- Tests must verify the AI button toggles open and closed.
- Tests must verify the AI button exposes highlighted/active state while open.
- Tests must verify opening AI closes any drawer.
- Tests must verify opening a drawer closes AI, then opening AI again leaves only AI visible.
- Tests must define and verify AI draft retention or clearing after closing AI.
- Tests must verify AI toggling does not change media path, playback time, paused state, playlist, or subtitle state.

## Required Test Shape
- Click `player.top.ai`, assert `ai_panel_open == true`, AI controls present, and top button active.
- Click `player.top.ai` again and assert `ai_panel_open == false` and AI controls absent.
- Open Playlist, then AI, and assert `drawer == None` and only AI panel selectors are present.
- Enter AI draft text, close and reopen AI, and assert the documented draft behavior.
- Snapshot unrelated playback state before toggles and assert it remains unchanged.

## Required Changes
- Expose active/pressed state for top-bar buttons through automation.
- Add AI panel top-button coverage to `plan_ui_e2e` or a focused top-bar test.
- Document AI draft retention behavior when the panel closes.

## Verification
- `cargo test -p tench-player --test plan_ui_e2e top_bar_ai_panel`
- `cargo test -p tench-player`
