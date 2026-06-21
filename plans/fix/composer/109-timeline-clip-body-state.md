# Timeline Clip Body State

## Source Plan

- `plans/composer/timeline-clip-body-control-work-plan.md`

## Gap Analysis

Clip rectangles register `SelectClip(Some(clip_id))`, selection stroke renders from `selected_clip_id`, and the inspector reads `selected_clip()`, but current E2E coverage only clicks one clip and checks for selected/inspector selectors. It does not assert the selected clip id or the displayed inspector values. See `apps/composer/src-tauri/src/ui/timeline_panel.rs:337`, `apps/composer/src-tauri/src/ui/mod.rs:218`, `apps/composer/src-tauri/src/ui/right_panel.rs:124`, and `apps/composer/src-tauri/tests/plan_ui_e2e.rs:262`.

Clicking another clip and clearing or preserving selection from an empty timeline click are untested. The implementation currently clears selection when a timeline track is clicked at a frame without a clip, but that behavior is not documented by a test. See `apps/composer/src-tauri/src/ui/state.rs:744`.

When a context menu is open, a click outside the menu dismisses the menu and returns before normal clip selection runs. If the intended contract is to close the menu and then select the clicked clip in the same click, that path is not implemented. See `apps/composer/src-tauri/src/ui/mod.rs:781`.

## Plan Requirements Not Met

- Tests must assert `selected_clip_id` and inspector field values after clicking a clip.
- Clicking another clip must be tested to move the highlight and clear the old highlight.
- Empty timeline click selection behavior must be explicitly tested according to the product contract.
- Clicking a clip while a context menu is open must be tested and implemented to match the explicit close-before-selection contract.

## Required Test Shape

- Add a Composer UI automation test that clicks `composer.timeline.clip.0` and asserts `selected_clip_id` equals that clip plus inspector values match the selected clip.
- Create or use a second clip, click it, and assert the selected highlight moves and the first clip is no longer selected.
- Click an empty frame on a timeline track and assert the chosen empty-click contract.
- Open a clip context menu, click a different clip body, and assert the menu closes and selection behavior matches the contract.

## Required Changes

- Add the missing clip body selection scenario tests.
- Expose selected clip id, inspector values, and selected highlight state through automation if current selectors are insufficient.
- If the context-menu contract requires same-click selection, continue dispatching the clicked clip after dismissing the menu.

## Verification

- `cargo test -p tench-composer --test plan_ui_e2e timeline_clip_body`
- `cargo test -p tench-composer`
