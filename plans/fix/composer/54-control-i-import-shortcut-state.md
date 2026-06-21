# Control I Import Shortcut State Fix Plan

## Source Plan

- `plans/composer/control-i-import-shortcut-control-work-plan.md`

## Gap Analysis

Ctrl+I does not reuse the toolbar Import action path. It calls
`request_media_import()` directly, which bypasses the test import injection path
and any shared result handling attached to `ClickAction::ImportMedia`.

## Plan Requirements Not Met

- Ctrl+I does not route through the same import path as the Import toolbar
  button.
- Ctrl+I cannot use `inject_test_media`, so the shortcut is not testable through
  the same deterministic headless path as the toolbar button.
- Ctrl+I silently no-ops when no Tauri `APP_HANDLE` or dialog sender is
  available.
- Ctrl+I does not reject extra modifiers such as Ctrl+Alt+I.
- There is no E2E test that pressing Ctrl+I imports injected media or opens the
  dialog route.
- There is no test that repeated Ctrl+I requests are deterministic and do not
  spawn duplicate uncontrolled dialogs.
- There is no test for Ctrl+I behavior while text input or subtitle editing is
  focused.

## Code Review

- `apps/composer/src-tauri/src/ui/mod.rs:250` handles toolbar Import through
  `ClickAction::ImportMedia`.
- `apps/composer/src-tauri/src/ui/mod.rs:251` consumes `test_next_media` for
  deterministic tests.
- `apps/composer/src-tauri/src/ui/mod.rs:255` falls back to
  `request_media_import`.
- `apps/composer/src-tauri/src/ui/mod.rs:1037` handles Ctrl+I directly in
  keyboard routing.
- `apps/composer/src-tauri/src/ui/mod.rs:1039` calls `request_media_import`
  directly instead of dispatching the shared import action.
- `apps/composer/src-tauri/src/ui/mod.rs:63` returns silently when dialog
  infrastructure is unavailable.

## Test Review

- `apps/composer/src-tauri/tests/plan_ui_e2e.rs:122` injects a media file for
  the toolbar import path.
- `apps/composer/src-tauri/tests/plan_ui_e2e.rs:125` clicks
  `composer.toolbar.import`.
- There is no Ctrl+I shortcut test.

## Required Test Shape

- Inject test media, press Ctrl+I, and assert the media bin grows exactly as it
  does through the toolbar Import button.
- Press Ctrl+I without dialog infrastructure and assert an actionable no-op or
  deterministic test-visible status instead of silent failure.
- Press Ctrl+Alt+I and Ctrl+Shift+I and assert the shortcut policy is enforced.
- Focus subtitle/text input, press Ctrl+I, and assert the documented focus
  precedence.
- Press Ctrl+I repeatedly and assert dialog/import state remains deterministic.

## Required Changes

- Route Ctrl+I through the same command helper or `ClickAction::ImportMedia`
  path used by the toolbar.
- Return a result from import request dispatch so missing dialog infrastructure
  can be surfaced through notices or test state.
- Add modifier and focus guards consistent with the shortcut policy.
- Expose import request status through automation for headless assertions.

## Verification

- `cargo test -p tench-composer control_i_import_shortcut`
- `cargo test -p tench-composer composer_plan_project_timeline_controls_use_real_events_ui_e2e`
- `git diff --check`
