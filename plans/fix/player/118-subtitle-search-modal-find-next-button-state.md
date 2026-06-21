# Subtitle Search Modal Find Next Button State

## Source Plan
- `plans/player/subtitle-search-modal-find-next-button-work-plan.md`

## Gap Analysis
Typing in the subtitle search input only updates `subtitle_search_text`; it does not run `search_subtitles`. `SearchSubtitleNext` advances only existing `subtitle_search_results`, so pressing Find Next after typing a fresh query can be a no-op. See `apps/player/src-tauri/src/ui/app.rs:2038` and `apps/player/src-tauri/src/ui/state.rs:1336`.

`SearchSubtitleNext` calls `backend.seek(self.state.current_time)` even when no next result was selected, so a no-op search can still send a backend seek to the old time. See `apps/player/src-tauri/src/ui/app.rs:1093`.

The current E2E types `gate` and clicks Find Next, but it does not assert search results, current result index, playback time, backend seek target, result text, or no-result behavior. See `apps/player/src-tauri/tests/plan_ui_e2e.rs:375`.

Repeated Find Next behavior at the last result is undocumented and untested. The current state method stops at the last result without wrapping or user-visible feedback.

Automation does not expose subtitle search result text/current index as a stable value, so UI assertions must rely on internal state.

## Plan Requirements Not Met
- Find Next must search the current input text or require a documented submit step.
- Tests must verify Find Next selects the next matching cue.
- Tests must verify playback seeks to the selected cue time.
- Tests must verify backend seek occurs only when a result is selected.
- Tests must verify result text/current index updates.
- Tests must define and verify behavior at the last result and with no results.
- Tests must verify reopen/Escape behavior after using Find Next.

## Required Test Shape
- Seed subtitle cues with multiple matches, type a query, click Find Next, and assert result list, current index, `current_time`, result text, and backend seek target.
- Click Find Next repeatedly and assert the documented last-result behavior.
- Search for a query with no matches and assert no backend seek and a documented no-result message/state.
- Close/reopen or press Escape after Find Next and assert modal state remains consistent.

## Required Changes
- Decide whether Find Next runs search from current text or is disabled until Submit.
- Avoid backend seek when no result was selected.
- Expose subtitle search result/current index through automation.
- Extend subtitle search E2E coverage for Find Next success, no-result, last-result, backend dispatch, and modal lifecycle.

## Verification
- `cargo test -p tench-player --test plan_ui_e2e subtitle_search_find_next`
- `cargo test -p tench-player`
