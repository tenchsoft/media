# Bottom A-B Loop Button State

## Source Plan
- `plans/player/bottom-ab-loop-button-work-plan.md`

## Gap Analysis
The second A-B Loop click clamps B with `current_time.max(a + 1.0)` but does not clamp the result to `duration`. If A is near the end of the media, B can become greater than the duration, which is not a valid loop point. See `apps/player/src-tauri/src/ui/state.rs:1411`.

State unit coverage verifies the basic three-click cycle, but it does not cover the near-A clamp case, toast text, seekbar highlight rendering, or automatic loop enforcement stopping after clear. See `apps/player/src-tauri/src/ui/state.rs:1551`.

The existing E2E clicks `player.controls.ab_loop` once and only asserts that `ab_loop` changed. It does not verify A stage, B stage, third-click clear, toast text, highlighted range, B clamp behavior, or loop enforcement after clear. See `apps/player/src-tauri/tests/plan_ui_e2e.rs:215`.

The button automation exposes only a generic button with no stage/value indicating whether the control is waiting for A, waiting for B, or clearing an active loop. See `apps/player/src-tauri/src/ui/paint_controls.rs:482`.

## Plan Requirements Not Met
- B-point clamp must keep the loop endpoint within media duration.
- Tests must verify first click marks A, second marks B, and third clears the loop.
- Tests must verify toast text for A mark, B mark, and clear.
- Tests must verify the seekbar A-B highlight appears and disappears.
- Tests must verify automatic loop enforcement stops after clearing the loop.
- Automation must expose A-B loop stage or active loop range.

## Required Test Shape
- Seek to a deterministic time, click `player.controls.ab_loop`, and assert `ab_stage == 1`, `ab_loop == Some((a, a))`, toast text, and no invalid highlight range.
- Seek to B, click again, and assert `ab_stage == 2`, clamped `(a, b)` within duration, toast text, and highlighted seekbar range.
- Click a third time and assert `ab_stage == 0`, `ab_loop == None`, clear toast, highlight absent, and playback no longer loops at the old B point.

## Required Changes
- Clamp B to a valid duration-bounded loop endpoint or reject impossible near-end loops with an actionable toast.
- Expose A-B stage/range through `player.controls.ab_loop` or `player.automatic.ab_loop`.
- Extend `plan_ui_e2e` A-B Loop coverage for full cycle, clamp, toast, highlight, and loop enforcement clear.

## Verification
- `cargo test -p tench-player --test plan_ui_e2e bottom_ab_loop`
- `cargo test -p tench-player`
