# Subtitle Search Modal Prev Button State

## Source Plan
- `plans/player/subtitle-search-modal-prev-button-work-plan.md`

## Gap Analysis
Typing in the subtitle search input only updates `subtitle_search_text`; it does not run `search_subtitles`. `SearchSubtitlePrev` moves only within existing `subtitle_search_results`, so pressing Prev after typing a fresh query can be a no-op. See `apps/player/src-tauri/src/ui/app.rs:2038` and `apps/player/src-tauri/src/ui/state.rs:1347`.

`SearchSubtitlePrev` calls `backend.seek(self.state.current_time)` even when no previous result was selected, so a no-op search can still send a backend seek to the old time. See `apps/player/src-tauri/src/ui/app.rs:1100`.

The current E2E clicks Prev after Find Next, but it does not assert search results, current result index, playback time, backend seek target, result text, or first-result/no-result behavior. See `apps/player/src-tauri/tests/plan_ui_e2e.rs:377`.

Repeated Prev behavior at the first result is undocumented and untested. The current state method stops at the first result without wrapping or user-visible feedback.

Automation does not expose subtitle search result text/current index as a stable value, so UI assertions must rely on internal state.

## Plan Requirements Not Met
- Prev must operate on a documented search result set from the current input.
- Tests must verify Prev selects the previous matching cue.
- Tests must verify playback seeks to the selected cue time.
- Tests must verify backend seek occurs only when a result is selected.
- Tests must verify result text/current index updates.
- Tests must define and verify behavior at the first result and with no results.
- Tests must verify reopen/Escape behavior after using Prev.

## Required Test Shape
- Seed subtitle cues with multiple matches, create a result set, move to a later result, click Prev, and assert current index, `current_time`, result text, and backend seek target.
- Click Prev at the first result and assert the documented first-result behavior.
- Search for a query with no matches and assert no backend seek and a documented no-result message/state.
- Close/reopen or press Escape after Prev and assert modal state remains consistent.

## Required Changes
- Decide whether Prev runs search from current text or is disabled until Submit.
- Avoid backend seek when no result was selected.
- Expose subtitle search result/current index through automation.
- Extend subtitle search E2E coverage for Prev success, no-result, first-result, backend dispatch, and modal lifecycle.

## Verification
- `cargo test -p tench-player --test plan_ui_e2e subtitle_search_prev`
- `cargo test -p tench-player`
