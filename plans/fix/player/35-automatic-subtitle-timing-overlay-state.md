# Automatic Subtitle Timing Overlay State

## Source Plan
- `plans/player/automatic-subtitle-timing-overlay-work-plan.md`

## Gap Analysis
`update_subtitle_for_position()` returns early when `subtitle_cues` is empty without clearing `subtitle_text`. If cues are removed or a new media state lacks cues after a subtitle was visible, the overlay can retain stale subtitle text instead of disappearing automatically. See `apps/player/src-tauri/src/ui/state.rs:974`.

The current E2E adjusts subtitle offset and asserts only `offset_ms`. It does not verify that the overlay appears inside a cue, disappears outside a cue, or updates immediately when offset changes. See `apps/player/src-tauri/tests/plan_ui_e2e.rs:339`.

State unit coverage verifies basic `seek_to` cue selection, but it does not cover backend `Position` events, `backend.tick()`, offset changes, active-track switching, stale cue clearing, or visual overlay placement. See `apps/player/src-tauri/src/ui/state.rs:1572`.

The automatic subtitle timing node is always emitted with no value for active subtitle text, cue id, timing state, or offset. Automation cannot assert that the overlay derives from canonical subtitle timing state. See `apps/player/src-tauri/src/ui/app.rs:2282`.

## Plan Requirements Not Met
- Empty cue state must clear `subtitle_text` so stale overlays disappear.
- Tests must verify overlay appearance and disappearance based on current playback time.
- Tests must verify offset changes update the visible overlay immediately.
- Tests must verify backend position/tick paths update subtitle timing, not only direct `seek_to`.
- Tests must verify overlay placement remains correct after resize or side-panel layout changes.
- Automation must expose active subtitle timing state or visible subtitle text.

## Required Test Shape
- Load deterministic subtitle cues, seek into a cue and assert `subtitle_text` plus visible overlay text; seek outside and assert both clear.
- Change an active track offset and assert the same current time now selects or clears the expected cue.
- Inject a backend `Position` event and a fake backend tick and assert subtitle timing updates through both paths.
- Clear cues after a visible subtitle and assert `subtitle_text == None` and no overlay is rendered.
- Open a drawer or resize with a subtitle visible and assert the overlay remains inside the video surface.

## Required Changes
- Clear `subtitle_text` when `subtitle_cues` is empty.
- Expose visible subtitle text, cue id, or active timing state in `player.automatic.subtitle_timing`.
- Extend E2E or targeted UI tests for seek timing, offset timing, backend position/tick timing, stale cue clearing, and layout changes.

## Verification
- `cargo test -p tench-player --test plan_ui_e2e automatic_subtitle_timing`
- `cargo test -p tench-player`
