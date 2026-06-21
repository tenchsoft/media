# Built-in Subtitle None Button State

## Source Plan
- `plans/player/builtin-subtitle-none-button-work-plan.md`

## Gap Analysis
The None action sets `active_builtin_subtitle_track = -1`, but when a backend exists it calls `backend.set_subtitle_track(0)`. Selecting built-in track 0 also calls `backend.set_subtitle_track(0)`, so disabling subtitles and selecting the first subtitle stream produce the same backend side effect. See `apps/player/src-tauri/src/ui/app.rs:988` and `crates/media-playback/src/lib.rs:695`.

The existing E2E clicks `player.subtitle.builtin.none` and only asserts `active_builtin_subtitle_track == -1`. It does not verify backend subtitle disabling, toast text, active row styling, no unrelated playback state changes, or repeated deterministic clicks. See `apps/player/src-tauri/tests/plan_ui_e2e.rs:353`.

The plan requires dynamic row hit testing against the currently rendered list, but tests do not cover built-in subtitle list changes after media reload, import/remove, drawer scroll, or first/middle/last row geometry. See `apps/player/src-tauri/src/ui/paint_panels.rs:632`.

Automation exposes the None row as a generic option without selected/active value, so tests cannot assert the active row state from the UI tree. See `apps/player/src-tauri/src/ui/app.rs:2420`.

## Plan Requirements Not Met
- Built-in None must call the backend disable value, not the first track value.
- Tests must verify backend subtitle disabling and toast text.
- Tests must verify no unrelated playback state changes occur.
- Tests must verify dynamic row hit testing after built-in subtitle list changes or scroll.
- Automation must expose selected/active state for the None row.

## Required Test Shape
- With a fake backend, click `player.subtitle.builtin.none` and assert `active_builtin_subtitle_track == -1`, backend disable call uses the documented disable index, toast text is `Built-in subtitles disabled`, and playback state is unchanged.
- Select track 0, then None, and assert the backend calls differ.
- Change built-in subtitle count/labels while the drawer is open, repaint, click None at the displayed bounds, and assert the correct action fires.

## Required Changes
- Use the backend's documented disable index for built-in subtitles.
- Expose selected state for `player.subtitle.builtin.none`.
- Add fake backend subtitle-track assertions.
- Extend `plan_ui_e2e` Built-in None coverage for backend disable, toast, active styling, dynamic rows, scroll, and no playback side effects.

## Verification
- `cargo test -p tench-player --test plan_ui_e2e builtin_subtitle_none`
- `cargo test -p tench-player`
