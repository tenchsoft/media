# Track Header Hidden Button State

## Source Plan

- `plans/composer/track-header-hidden-button-work-plan.md`

## Gap Analysis

The H button toggles `track.hidden` and renders header button styling from that boolean, but the timeline clip rendering path does not apply `track.hidden` to hide or dim clips. The only UI references to `track.hidden` are the header button color/text. See `apps/composer/src-tauri/src/ui/timeline_panel.rs:252` and `apps/composer/src-tauri/src/ui/mod.rs:309`.

Preview composition and export behavior do not appear to consult `track.hidden`, so the hidden-track rendering rule is undefined in output paths. See `crates/composer-core/src/timeline.rs:145`.

The current E2E coverage clicks `composer.track.hidden` once and asserts the boolean changed. It does not click again, verify clip visibility/dimming, isolate one track among many, or assert export behavior while hidden. See `apps/composer/src-tauri/tests/plan_ui_e2e.rs:170`.

## Plan Requirements Not Met

- Hidden tracks must visually hide or dim their clips according to the product contract.
- Hidden state must be applied consistently in preview composition and export if hidden means disabled output.
- Clicking H again must be tested to restore visible track content.
- Hiding one track among multiple tracks must be tested to leave other tracks visible.
- Export while a track is hidden must be tested against the chosen rendering rule.

## Required Test Shape

- Add a Composer UI automation test that clicks `composer.track.hidden` and asserts the target track's clips become hidden or dimmed.
- Click H again and assert the clips return to visible styling.
- Add a second track, hide only one track, and assert only that track's content changes.
- Queue export with a hidden track and assert the render job or export composition follows the hidden-track rule.
- Assert the H button active/inactive state through automation.

## Required Changes

- Define the hidden-track output rule for timeline, preview, and export.
- Apply `track.hidden` in timeline clip rendering and preview/export composition as required.
- Expose track hidden state, clip visibility styling, and export inclusion state through automation.
- Add hidden-track behavior tests.

## Verification

- `cargo test -p tench-composer --test plan_ui_e2e track_hidden`
- `cargo test -p tench-composer`
