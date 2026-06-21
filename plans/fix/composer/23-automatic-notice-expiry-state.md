# Automatic Notice Expiry State Fix Plan

## Source Plan

- `plans/composer/automatic-notice-expiry-behavior-work-plan.md`

## Gap Analysis

Composer stores notice text and expiry time and clears expired notices during
paint, but the expiry contract is not fully implemented or tested. Expiry does
not report whether visibility changed, no repaint is explicitly requested for a
timed disappearance, and empty notices still receive expiry state.

## Plan Requirements Not Met

- There is no test that waits past the configured duration and verifies the
  toolbar notice disappears.
- `check_notice_expiry` does not return whether the notice changed, so paint
  cannot request a follow-up repaint specifically for notice visibility changes.
- No scheduled tick or animation frame is requested to clear a notice when there
  is no other UI activity.
- `set_notice("")` stores an empty notice with an expiry time instead of clearing
  both fields.
- Second-notice replacement is not tested for a fresh expiry.
- Automation exposes only a generic `composer.automatic.notice` status node, not
  the actual notice text or expiry state.

## Code Review

- `apps/composer/src-tauri/src/ui/state.rs:467` stores notice text and sets
  `notice_expires_at` to three seconds in the future.
- `apps/composer/src-tauri/src/ui/state.rs:473` clears notice text and expiry
  when `Instant::now()` passes the deadline, but returns no changed flag.
- `apps/composer/src-tauri/src/ui/mod.rs:653` calls `check_notice_expiry` during
  paint, but does not request repaint when a notice expires.
- `apps/composer/src-tauri/src/ui/toolbar.rs:65` paints notice text only when
  `composer_notice` is not empty.
- `apps/composer/src-tauri/src/ui/mod.rs:1341` exposes a generic notice
  automation node without value.

## Test Review

- `apps/composer/src-tauri/src/ui/state.rs:1197` only asserts that `set_notice`
  stores text and an expiry.
- `apps/composer/src-tauri/tests/plan_ui_e2e.rs:415` and `:425` assert the
  generic notice selector is present after undo/redo, but do not inspect text,
  expiry, replacement, or disappearance.
- There is no test for empty notice input.

## Required Test Shape

- Use a controllable clock or test helper to set a notice, advance past expiry,
  call the UI processing path, and assert text and expiry are both cleared.
- Trigger a second notice before the first expires and assert the text and
  expiry deadline are replaced.
- Call `set_notice("")` and assert no notice text or expiry remains.
- Add E2E automation that exposes the current toolbar notice value and asserts
  it disappears after the expiry tick.
- Verify that a repaint or scheduled tick occurs when notice visibility changes.

## Required Changes

- Make notice expiry clock injectable or pass `Instant` into expiry checking for
  deterministic tests.
- Change `check_notice_expiry` to return whether it cleared the notice.
- Request repaint or schedule a tick when notice visibility changes or when a
  notice is waiting to expire.
- Treat empty notice strings as clear/no-op and remove `notice_expires_at`.
- Expose notice text and expiry/presence through automation.

## Verification

- `cargo test -p tench-composer automatic_notice_expiry`
- `cargo test -p tench-composer composer_plan_inspector_render_queue_ai_and_context_menu_use_real_events_ui_e2e`
- `cargo test -p tench-ui-test ui_automation`
- `git diff --check`
