# Spacebar Play Pause Shortcut State

## Source Plan

- `plans/composer/spacebar-play-pause-shortcut-control-work-plan.md`

## Gap Analysis

The global Space shortcut does not check modifiers before toggling playback. Ctrl+Space or Alt+Space can run the global play/pause path. See `apps/composer/src-tauri/src/ui/mod.rs:955`.

Subtitle text input consumes plain `LogicalKey::Character(" ")`, but `LogicalKey::Named(NamedKey::Space)` is not consumed by the subtitle editor and can fall through to global playback toggle while the editor is focused. See `apps/composer/src-tauri/src/ui/mod.rs:910` and `apps/composer/src-tauri/src/ui/mod.rs:955`.

The shortcut toggles playback and repaints, but it does not set a notice or explicit status beyond the preview label. See `apps/composer/src-tauri/src/ui/mod.rs:955`.

The current keyboard E2E test presses Space and only asserts that the capture changed. It does not assert transport state, preview label, second-press pause behavior, modifier handling, or subtitle-editor precedence. See `apps/composer/src-tauri/tests/plan_ui_e2e.rs:357`.

## Plan Requirements Not Met

- The global Space shortcut must respect modifier keys before invoking play/pause.
- Text-focused controls must consume Space consistently for both character and named-key event shapes when text editing takes precedence.
- Tests must verify playback state and preview label, not only a changed capture.
- Repeated Space presses must be tested for deterministic play/pause toggling.
- The shortcut must expose the required notice or equivalent user-visible status.

## Required Test Shape

- Add a Composer UI automation test that presses Space and asserts `is_playing == true`, `shuttle_direction == 1`, and the preview label shows forward playback.
- Press Space again and assert `is_playing == false`, `shuttle_direction == 0`, and the preview label returns to `Paused`.
- Press Ctrl+Space and Alt+Space and assert they do not run the global play/pause shortcut unless explicitly intended.
- Focus the subtitle editor, press Space through both named and character event shapes if supported, and assert text editing takes precedence without losing subtitle text.
- Assert the required status or notice node after the shortcut.

## Required Changes

- Normalize Space shortcut routing through a helper that rejects unsupported modifiers and honors text focus consistently.
- Ensure subtitle text input consumes both named Space and character Space when focused.
- Expose preview label or transport state through automation for direct assertions.

## Verification

- `cargo test -p tench-composer --test plan_ui_e2e spacebar_play_pause`
- `cargo test -p tench-composer`
