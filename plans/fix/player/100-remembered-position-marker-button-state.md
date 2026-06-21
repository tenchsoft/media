# Remembered Position Marker Button State

## Source Plan
- `plans/player/remembered-position-marker-button-work-plan.md`

## Gap Analysis
The current E2E clicks `player.seekbar.remembered` and only asserts the capture changed. It does not assert `current_time`, backend seek target, toast text, saved-position restore after reopening a file, overlap behavior, or duration-clamp behavior. See `apps/player/src-tauri/tests/plan_ui_e2e.rs:155`.

The marker is rendered at a clamped position when `remembered_position` exceeds duration, but `JumpToRememberedPosition` sends the unclamped remembered value to the backend. `PlayerState::seek_to` clamps state, so backend and UI can diverge after duration changes. See `apps/player/src-tauri/src/ui/paint_controls.rs:179` and `apps/player/src-tauri/src/ui/app.rs:1049`.

There is no deterministic persistent-state test hook for reopening a file and restoring a saved remembered position. The real path uses `PersistentState::load_position`, which is not asserted in UI automation. See `apps/player/src-tauri/src/ui/state.rs:895`.

Automation exposes only the marker selector, not marker time/value or clamped position, so tests cannot assert the marker represents the restored position through the UI tree.

## Plan Requirements Not Met
- Tests must verify marker click sets `current_time` to the remembered position.
- Tests must verify backend seek receives the same clamped target as `PlayerState`.
- Tests must verify the toast confirms the remembered-position jump.
- Tests must verify reopening a file restores the saved remembered position.
- Tests must verify overlap with the current handle does not cause an unexpected jump.
- Tests must verify remembered positions clamp inside the track after duration changes.
- Automation must expose marker value or clamped position for assertions.

## Required Test Shape
- Set `remembered_position`, click `player.seekbar.remembered`, and assert `current_time`, toast, and backend seek target.
- Set `remembered_position > duration`, click the marker, and assert both state and backend use the clamped duration.
- Set `current_time == remembered_position`, click the marker, and assert no unrelated playback state changes.
- Use a temp persistent-state store or injection hook, reopen a file, and assert the marker appears at the saved time.
- Assert the automation node value/bounds corresponds to the clamped remembered position.

## Required Changes
- Clamp the backend seek target to the same value used by `PlayerState::seek_to`.
- Add deterministic persistent-position injection for tests.
- Expose remembered marker value/clamped position through automation.
- Extend seekbar E2E coverage for remembered jump, restore, overlap, and duration-clamp cases.

## Verification
- `cargo test -p tench-player --test plan_ui_e2e remembered_position_marker`
- `cargo test -p tench-player`
