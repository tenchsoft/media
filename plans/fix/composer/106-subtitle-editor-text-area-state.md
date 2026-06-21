# Subtitle Editor Text Area State

## Source Plan

- `plans/composer/subtitle-editor-text-area-control-work-plan.md`

## Gap Analysis

Escape clears `subtitle_focused`, but it does not reset `input_focus` from `ComposerInputFocus::Subtitle` back to `None`. See `apps/composer/src-tauri/src/ui/mod.rs:915` and `apps/composer/src-tauri/src/ui/mod.rs:363`.

The subtitle editor consumes plain character events while focused, but named Space events can fall through to the global Space shortcut instead of inserting text. See `apps/composer/src-tauri/src/ui/mod.rs:910` and `apps/composer/src-tauri/src/ui/mod.rs:955`.

The current E2E coverage types one subtitle line and only asserts the editor selector plus a capture change. It does not assert placeholder disappearance, stored subtitle text, multiline Enter behavior, Backspace deletion, Escape focus removal, or global shortcuts working again after Escape. See `apps/composer/src-tauri/tests/plan_ui_e2e.rs:247`.

The renderer draws `subtitle_text` as a single text call, and multiline rendering is not covered by tests. See `apps/composer/src-tauri/src/ui/left_panel.rs:123`.

## Plan Requirements Not Met

- Escape must fully leave subtitle focus, including clearing the tracked input focus state.
- Space must be handled consistently as subtitle text while the editor is focused.
- Multiline subtitle text must be tested for storage and rendering.
- Backspace must be tested to delete only the last subtitle character.
- Escape must be tested to keep existing text and restore global shortcut behavior.

## Required Test Shape

- Add a Composer UI automation test that clicks `composer.subtitle.editor`, asserts placeholder text disappears, types text, and asserts `subtitle_text`.
- Press Enter between two lines and assert the stored text contains a newline and the field renders both lines acceptably.
- Press Backspace and assert only the final subtitle character is removed.
- Press Escape and assert `subtitle_focused == false`, `input_focus == None`, and existing text remains.
- After Escape, press Space and assert global play/pause works again.
- While focused, press Space through both named and character event shapes if supported and assert subtitle text receives a space rather than toggling playback.

## Required Changes

- Clear `input_focus` when Escape leaves the subtitle editor.
- Ensure focused subtitle input consumes named Space and character Space consistently.
- Add multiline rendering support or assertions for newline handling in the subtitle field.
- Expose subtitle text, focus state, and rendered value through automation if current state access is insufficient.

## Verification

- `cargo test -p tench-composer --test plan_ui_e2e subtitle_editor`
- `cargo test -p tench-composer`
