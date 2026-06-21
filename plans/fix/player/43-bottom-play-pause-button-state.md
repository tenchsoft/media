# Bottom Play Pause Button State

## Source Plan
- `plans/player/bottom-play-pause-button-work-plan.md`

## Gap Analysis
`ClickAction::PlayPause` calls backend play/pause before `PlayerState::toggle_playback()` applies the `has_media` guard. If a backend exists while `has_media == false`, the backend can still receive `play()` even though UI state refuses to start playback. See `apps/player/src-tauri/src/ui/app.rs:720` and `apps/player/src-tauri/src/ui/state.rs:996`.

Backend and UI state can diverge because the backend command is chosen from `backend.is_playing()` while `PlayerState` is toggled independently. If the backend is already playing but `state.is_playing` is false, clicking pauses the backend and toggles state to true, leaving them inverted. See `apps/player/src-tauri/src/ui/app.rs:721`.

The buffering scenario is not explicitly handled. A click while buffering can dispatch play/pause based on current backend state and then blindly toggle UI state, even though buffering events may immediately pause and later resume playback. See `apps/player/src-tauri/src/ui/app.rs:512`.

The existing E2E clicks Play/Pause and only asserts the capture changed. It does not verify `is_playing`, icon changes, backend play/pause call count, no-media behavior, already-playing backend behavior, or buffering synchronization. See `apps/player/src-tauri/tests/plan_ui_e2e.rs:146`.

## Plan Requirements Not Met
- Play/Pause must guard backend play when no media is loaded.
- Play/Pause must set PlayerState and backend to the same target playback state.
- Tests must verify backend already-playing pause happens exactly once and leaves UI paused.
- Tests must verify no-media click does not start backend playback.
- Tests must verify buffering click state matches backend state after buffering events.
- Automation must expose the current play/pause icon or playback state.

## Required Test Shape
- With `has_media == false` and a fake backend, click `player.controls.play_pause` and assert no backend play call and `is_playing == false`.
- With media loaded and backend paused, click and assert backend play called once, `is_playing == true`, and icon/value is pause.
- With backend already playing and UI state false, click and assert the implementation reconciles state before/after the command instead of leaving backend/UI inverted.
- Set buffering state, click Play/Pause, inject buffering events, and assert final UI/backend state matches the backend event result.

## Required Changes
- Compute a single target playback state from canonical media/UI/backend state and apply it consistently to backend and PlayerState.
- Guard backend commands when no media is loaded.
- Add fake backend play/pause call assertions.
- Expose play/pause icon or playback state through `player.controls.play_pause`.
- Extend `plan_ui_e2e` Play/Pause coverage for no-media, playing, paused, backend desync, and buffering scenarios.

## Verification
- `cargo test -p tench-player --test plan_ui_e2e bottom_play_pause`
- `cargo test -p tench-player`
