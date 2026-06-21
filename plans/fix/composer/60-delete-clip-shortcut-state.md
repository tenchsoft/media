# Delete Clip Shortcut State Fix Plan

## Source Plan

- `plans/composer/delete-clip-shortcut-control-work-plan.md`

## Gap Analysis

Delete/Backspace routes to the selected-clip delete path, but the shortcut is
not covered by E2E. Missing-selection no-op behavior is silent, and repeat,
modifier, and text-focus boundaries are unverified.

## Plan Requirements Not Met

- Pressing Delete or Backspace with no selected clip returns `false` without an
  actionable notice.
- There is no E2E test that Delete removes the selected clip through the same
  delete state path as the toolbar.
- There is no E2E test that Backspace removes the selected clip when subtitle
  editing is not focused.
- There is no test that Backspace while subtitle editing edits text instead of
  deleting a clip.
- There is no test that Delete while subtitle editing follows the documented
  text-focus policy.
- There is no test for modifier combinations with Delete or Backspace.
- There is no test that repeated Delete/Backspace presses remain deterministic
  after selection clears.

## Code Review

- `apps/composer/src-tauri/src/ui/mod.rs:979` handles Delete and Backspace
  shortcut keys.
- `apps/composer/src-tauri/src/ui/mod.rs:981` suppresses global deletion only
  when `subtitle_focused` is true.
- `apps/composer/src-tauri/src/ui/mod.rs:983` calls `delete_selected_clip`
  directly.
- `apps/composer/src-tauri/src/ui/mod.rs:211` routes toolbar/context Delete
  through `ClickAction::DeleteClip`.
- `apps/composer/src-tauri/src/ui/state.rs:789` returns `false` without a notice
  when no clip is selected.
- `apps/composer/src-tauri/src/ui/state.rs:819` clears selection on successful
  deletion.

## Test Review

- `apps/composer/src-tauri/tests/plan_ui_e2e.rs:82` only asserts
  `composer.toolbar.delete` is present.
- There is no E2E test pressing Delete or Backspace for clip deletion.
- Context-menu Delete gaps are tracked separately in
  `plans/fix/composer/40-clip-context-menu-delete-state.md`.

## Required Test Shape

- Select a clip, press Delete, and assert the clip is removed, selection clears,
  undo stack grows, and notice text is correct.
- Repeat with Backspace while subtitle editing is not focused.
- Clear selection, press Delete and Backspace, and assert no mutation plus an
  actionable no-op notice.
- Focus the subtitle editor, press Backspace, and assert subtitle text changes
  while timeline data remains unchanged.
- Press Delete/Backspace repeatedly and assert stable no-op behavior after the
  first successful deletion.
- Exercise modifier combinations and assert the shortcut policy is enforced.

## Required Changes

- Add a shared delete command wrapper that maps delete results to success,
  missing-target, and invalid-target notices.
- Use the wrapper from toolbar/context actions and Delete/Backspace shortcuts.
- Expose selected clip id, clip count, undo count, subtitle text, and notice
  text through automation or focused test helpers.

## Verification

- `cargo test -p tench-composer delete_clip_shortcut`
- `cargo test -p tench-composer composer_plan_keyboard_shortcuts_and_automatic_playback_use_real_events_ui_e2e`
- `git diff --check`
