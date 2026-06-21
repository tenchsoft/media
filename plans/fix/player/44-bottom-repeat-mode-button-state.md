# Bottom Repeat Mode Button State

## Source Plan
- `plans/player/bottom-repeat-mode-button-work-plan.md`

## Gap Analysis
End-of-stream handling checks `has_next` before calling `next_track()`, so Repeat All at the final playlist item and Repeat One at the final item are not honored. The repeat-aware logic exists in `PlayerState::next_track()`, but the backend EOS path bypasses it when there is no next index. See `apps/player/src-tauri/src/ui/app.rs:452` and `apps/player/src-tauri/src/ui/state.rs:1044`.

The existing E2E clicks `player.controls.repeat` once and only asserts that `repeat_mode` changed. It does not verify the full Off -> All -> One -> Off cycle, visible label, toast text, Repeat One end-of-track behavior, or Repeat All end-of-playlist behavior. See `apps/player/src-tauri/tests/plan_ui_e2e.rs:203`.

The visible repeat label and active fill are drawn from `repeat_mode`, but automation exposes only the generic action node. Tests cannot assert the current repeat label or active state from automation. See `apps/player/src-tauri/src/ui/paint_controls.rs:399`.

## Plan Requirements Not Met
- Backend end-of-stream handling must respect Repeat One and Repeat All at playlist boundaries.
- Tests must verify the full repeat cycle returns to Off.
- Tests must verify label and toast for each repeat mode.
- Tests must verify Repeat One restarts the current item at end of track.
- Tests must verify Repeat All loops from the final playlist item to the first.
- Automation must expose current repeat mode or label.

## Required Test Shape
- Click `player.controls.repeat` three times and assert mode, label/value, and toast after each click.
- Configure Repeat One at the current item, inject `EndOfStream`, and assert playback restarts the same item at time 0.
- Configure Repeat All at the final playlist item, inject `EndOfStream`, and assert `current_playlist_index == Some(0)` and the first item loads.
- Configure Repeat Off at the final item and assert playback stops without wrapping.

## Required Changes
- Route EOS through repeat-aware `next_track()` or handle Repeat One/All before checking `has_next`.
- Expose repeat mode/label through `player.controls.repeat`.
- Extend `plan_ui_e2e` Repeat coverage for full cycle, toast, label, EOS Repeat One, EOS Repeat All, and Repeat Off boundary behavior.

## Verification
- `cargo test -p tench-player --test plan_ui_e2e bottom_repeat_mode`
- `cargo test -p tench-player`
