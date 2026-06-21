# Add Track Button State Fix Plan

## Source Plan

- `plans/composer/add-track-button-work-plan.md`

## Gap Analysis

The Add Track button is registered and appends a video track, but the plan's
observable contract is not fully covered. Repeated-add uniqueness, sequential
labels, selection preservation, and compact timeline layout are not verified or
exposed clearly enough.

## Plan Requirements Not Met

- E2E coverage does not assert that exactly one new video track appears at the
  end of the track list after one click.
- Repeated Add Track clicks are not tested for unique `TrackId` values and
  sequential display names.
- Selected clip preservation is not tested.
- Small timeline height behavior is not tested for non-overlapping track lanes
  or controls.
- Automation exposes track lanes with a generic `Track lane` label and does not
  expose track id, kind, or display name.

## Code Review

- `apps/composer/src-tauri/src/ui/timeline_panel.rs:159` registers
  `ClickAction::AddTrack(TrackType::Video)` on the Add Track button.
- `apps/composer/src-tauri/src/ui/state.rs:964` appends a track through
  `project.timeline.add_track` and sets `Track added`.
- `apps/composer/src-tauri/src/ui/timeline.rs:18` divides the remaining timeline
  height by track count. With many tracks or a very small timeline, lane heights
  can become too small for labels and controls.
- `apps/composer/src-tauri/src/ui/mod.rs:1294` exposes automation track lanes
  with a generic label instead of the track name or id.

## Test Review

- `apps/composer/src-tauri/tests/plan_ui_e2e.rs:177` clicks
  `composer.timeline.add_track` and checks only that
  `composer.automatic.track_lanes` is present.
- `apps/composer/src-tauri/src/ui/state.rs:1214` unit-tests that track count
  changes, but does not assert generated id, name, kind, or selected-clip state.
- There is no compact-height layout test for lane bounds and overlap.

## Required Test Shape

- Add an E2E test that records the current track count, clicks Add Track once,
  and asserts the new last track is video with the expected id and display name.
- Click Add Track repeatedly and assert ids are unique and video names advance
  sequentially.
- Select a clip, add a track, and assert the selected clip remains selected
  unless the product intentionally changes that rule.
- Run the Add Track flow with a small timeline viewport and assert track lane
  automation bounds do not overlap.
- Assert the `Track added` notice appears.

## Required Changes

- Extend track lane automation with track id, kind, name, and bounds metadata.
- Add a minimum lane/control layout policy for compact timeline heights, such as
  internal scrolling or clamped visible controls.
- Add focused tests for repeated Add Track behavior and selection preservation.

## Verification

- `cargo test -p tench-composer add_track`
- `cargo test -p tench-composer composer_plan_timeline_controls_use_real_events_ui_e2e`
- `cargo test -p tench-ui-test ui_automation`
- `git diff --check`
