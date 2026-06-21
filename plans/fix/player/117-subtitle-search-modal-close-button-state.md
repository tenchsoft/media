# Subtitle Search Modal Close Button State

## Source Plan
- `plans/player/subtitle-search-modal-close-button-work-plan.md`

## Gap Analysis
The current E2E clicks `player.subtitle_search.close` and asserts the close selector is absent, but it does not assert `subtitle_search_open == false`, `subtitle_search_focused == false`, search draft/result cleanup policy, reopen behavior, Escape-after-close behavior, or unrelated playback invariants. See `apps/player/src-tauri/tests/plan_ui_e2e.rs:378`.

`CloseModal` clears the search open/focus flags but leaves `subtitle_search_text`, `subtitle_search_results`, `subtitle_search_current`, and `subtitle_search_result_time` untouched. Reopen currently clears some state through `ShowSubtitleSearch`, but the close-path behavior is undocumented and untested. See `apps/player/src-tauri/src/ui/app.rs:1345` and `apps/player/src-tauri/src/ui/app.rs:1314`.

The close button uses shared `CloseModal`, which clears several unrelated modal/focus flags. There is no focused test proving subtitle search Close only commits the expected cleanup scope. See `apps/player/src-tauri/src/ui/app.rs:1345`.

## Plan Requirements Not Met
- Tests must verify subtitle search focus clears after Close.
- Search draft/results cleanup behavior on Close must be documented and tested.
- Tests must verify reopening search after Close starts from the documented state.
- Tests must verify Escape after Close does not leave modal state half-open.
- Tests must verify Close does not change playback time, paused state, media path, playlist, drawer tab, or unrelated modal flags.

## Required Test Shape
- Open search, type text, navigate results, click Close, and assert open/focus flags plus documented draft/result state.
- Reopen search and assert the input/results state is clean or restored according to product rules.
- Press Escape after Close and assert no search modal flag reopens or remains half-open.
- Snapshot unrelated player state before Close and assert it remains unchanged.

## Required Changes
- Document whether subtitle search Close clears draft/results immediately or only on next open.
- Add focused subtitle search Close state coverage to `plan_ui_e2e` or a modal test.
- Expose search input value/focus/result state through automation if needed.

## Verification
- `cargo test -p tench-player --test plan_ui_e2e subtitle_search_modal_close`
- `cargo test -p tench-player`
