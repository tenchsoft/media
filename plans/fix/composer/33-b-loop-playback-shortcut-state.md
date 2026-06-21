# B Loop Playback Shortcut State Fix Plan

## Source Plan

- `plans/composer/b-loop-playback-shortcut-control-work-plan.md`

## Gap Analysis

The `B` shortcut toggles loop playback, but it does not validate loop
preconditions and does not guard against modifier-key combinations before
invoking the global shortcut. Existing tests only check that pressing `b`
changes the capture.

## Plan Requirements Not Met

- Pressing `B` without valid in/out points does not report an actionable no-op;
  it enables loop state even though playback cannot loop meaningfully.
- The global `B` shortcut does not reject modifier combinations such as
  Ctrl+B or Alt+B.
- There is no state method for loop toggle behavior, so shortcut validation,
  notice text, and future visible controls cannot share one backend path.
- There is no test that `B` toggles loop only when valid in/out preconditions
  are met.
- There is no test that Ctrl+B, Alt+B, or focused text-editing contexts do not
  toggle global loop playback.
- There is no test that repeated `B` presses deterministically alternate loop
  state and notices.

## Code Review

- `apps/composer/src-tauri/src/ui/mod.rs:952` gates shortcut handling on
  pressed keyboard events.
- `apps/composer/src-tauri/src/ui/mod.rs:941` routes plain subtitle text input
  before global shortcuts, but modified character events can still continue to
  global shortcut handling.
- `apps/composer/src-tauri/src/ui/mod.rs:1075` toggles `loop_playback` directly
  for `c == "b"` without modifier or precondition checks.
- `apps/composer/src-tauri/src/ui/state.rs:729` loop playback only has an
  effect when both in/out points exist.

## Test Review

- `apps/composer/src-tauri/tests/plan_ui_e2e.rs:377` includes `"b"` in a loop of
  keyboard smoke tests.
- `apps/composer/src-tauri/tests/plan_ui_e2e.rs:384` only asserts the capture
  changed after pressing `b`.
- Existing loop boundary tests are tracked separately by
  `plans/fix/composer/21-automatic-loop-playback-boundary-state.md`; they do
  not cover shortcut routing.

## Required Test Shape

- Set valid in/out points, press `B`, and assert `loop_playback` toggles with
  the expected notice.
- Clear one or both in/out points, press `B`, and assert deterministic no-op
  behavior with an actionable notice.
- Press Ctrl+B and Alt+B and assert loop state does not change.
- Focus the subtitle editor, press plain `b`, and assert text editing takes
  precedence while loop state remains unchanged.
- Press `B` repeatedly and assert loop state and notice text alternate
  predictably.

## Required Changes

- Add a shared `toggle_loop_playback` state method that validates in/out
  preconditions, updates state, and returns a result for UI notices.
- Route the `B` shortcut through the shared method only when no disallowed
  modifiers are active and no text input target should receive the key.
- Expose loop state and notice values through automation for selector-based
  assertions.

## Verification

- `cargo test -p tench-composer b_loop_playback_shortcut`
- `cargo test -p tench-composer automatic_loop_playback_boundary`
- `cargo test -p tench-composer composer_plan_keyboard_shortcuts_and_automatic_playback_use_real_events_ui_e2e`
- `git diff --check`
