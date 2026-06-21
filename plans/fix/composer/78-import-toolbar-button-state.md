# Import Toolbar Button State

## Source Plan

- `plans/composer/import-toolbar-button-work-plan.md`

## Gap Analysis

Dialog cancellation is ignored by `process_dialog_results`, so a previous success notice or import status can remain visible after a later canceled picker. See `apps/composer/src-tauri/src/ui/mod.rs:178` and `apps/composer/src-tauri/src/ui/mod.rs:190`.

The current E2E coverage injects a single test media path and asserts only that the media-bin count increased. This bypasses the dialog result channel and does not verify metadata, import status, notice text, active Media tab, multiple selected files, cancel behavior, or unsupported-extension fallback. See `apps/composer/src-tauri/tests/plan_ui_e2e.rs:118`.

The UI test hook supports only one pending media path through `test_next_media`, so the multi-file confirmation scenario cannot be covered through the same toolbar path today. See `apps/composer/src-tauri/src/ui/mod.rs:115` and `apps/composer/src-tauri/src/ui/mod.rs:250`.

## Plan Requirements Not Met

- Canceling the native picker must leave the media bin unchanged and must not leave a stale success notice or status.
- Confirming multiple files through the toolbar import flow must be testable and must import each accepted path exactly once.
- Toolbar import tests must assert `import_status`, `has_media`, active Media tab, media metadata, and user notice after a successful import.
- Unsupported-extension fallback must be covered through the importer path when the file is allowed.

## Required Test Shape

- Add a Composer UI automation test that simulates a single confirmed dialog path and asserts media-bin entry metadata, `import_status`, active Media tab, `has_media`, and notice.
- Add a multi-file dialog result test that asserts every accepted path imports exactly once.
- Add a cancel result test after a successful import and assert no new media is added and stale success messaging is cleared or replaced.
- Add an unsupported-extension import test and assert the fallback media asset entry remains usable.

## Required Changes

- Handle `DialogResult::Cancelled` explicitly by preserving data while clearing or replacing stale import-success messaging.
- Extend the test dialog injection path so tests can drive multi-file and cancel results through `process_dialog_results`.
- Expose enough automation or state access to verify active Media tab, import status, notice, and imported metadata.

## Verification

- `cargo test -p tench-composer --test plan_ui_e2e import_toolbar`
- `cargo test -p tench-composer`
