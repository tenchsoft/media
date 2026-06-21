# Bottom Shuffle Toggle Button State

## Source Plan
- `plans/player/bottom-shuffle-toggle-button-work-plan.md`

## Gap Analysis
Shuffle playback does not maintain an explicit shuffled order. `next_track()` derives a next index from a hash of the current index each time, which avoids the current item but does not provide a stable shuffled queue, history, or deterministic off-ramp back to sequential order. See `apps/player/src-tauri/src/ui/state.rs:1058`.

The existing E2E clicks `player.controls.shuffle` once and only asserts that `shuffle_enabled` changed. It does not verify active style, visible label, toast text, Next behavior while shuffle is on, sequential behavior after shuffle is off, or the one-item playlist case. See `apps/player/src-tauri/tests/plan_ui_e2e.rs:207`.

The visible shuffle label and active fill are drawn from `shuffle_enabled`, but automation exposes only the generic action node. Tests cannot assert current label or active state through the control. See `apps/player/src-tauri/src/ui/paint_controls.rs:425`.

## Plan Requirements Not Met
- Shuffle playback must define and maintain a shuffled order or explicit random-selection rule.
- Tests must verify active style/label and toast for shuffle on/off.
- Tests must verify Next uses shuffle behavior while enabled.
- Tests must verify sequential order resumes when shuffle is disabled.
- Tests must verify one-item playlist behavior does not produce an invalid index.
- Automation must expose shuffle active state or label.

## Required Test Shape
- Click `player.controls.shuffle`, assert `shuffle_enabled == true`, active label/value, and toast `Shuffle on`.
- With a multi-item playlist, click Next and assert the selected index follows the documented shuffled order/rule and remains valid.
- Click Shuffle off, then Next and assert sequential index behavior resumes.
- With one playlist item and shuffle on, click Next and assert `current_playlist_index` remains valid and no load error occurs.

## Required Changes
- Implement a persistent shuffled order/history or document and expose the exact random-selection rule.
- Expose shuffle active state or label through `player.controls.shuffle`.
- Extend `plan_ui_e2e` Shuffle coverage for active style, toast, shuffled Next, sequential resume, and one-item playlists.

## Verification
- `cargo test -p tench-player --test plan_ui_e2e bottom_shuffle_toggle`
- `cargo test -p tench-player`
