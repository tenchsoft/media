# Automatic Track Lane Layout State Fix Plan

## Source Plan

- `plans/composer/automatic-track-lane-layout-behavior-work-plan.md`

## Gap Analysis

Track lane height is recomputed at paint time, but the layout behavior is not
fully implemented or verified. Empty tracks do not get value-bearing lane
automation, delete-track UI is not registered, and many-track layouts can shrink
below usable control height.

## Plan Requirements Not Met

- Delete-track behavior is not reachable from a rendered track header control.
- Empty tracks do not expose lane/header automation nodes, so adding a blank
  track cannot be verified by lane geometry.
- There is no test that adding a track shrinks all lane heights evenly and shows
  the new header.
- There is no UI/E2E test that deleting a track expands remaining lane heights
  evenly.
- There is no test that resizing the timeline splitter redraws lanes to fill
  the available height.
- Many-track layouts can make fixed-size track controls overlap or become
  unreachable.
- There is no test that hidden or muted state changes preserve valid lane
  layout and click regions.

## Code Review

- `apps/composer/src-tauri/src/ui/timeline.rs:18` divides available lane height
  by track count without a minimum usable row height or scrolling strategy.
- `apps/composer/src-tauri/src/ui/timeline_panel.rs:164` computes a single
  `track_h` for all rendered tracks.
- `apps/composer/src-tauri/src/ui/timeline_panel.rs:189` uses fixed
  `18x14` track control buttons regardless of the computed lane height.
- `apps/composer/src-tauri/src/ui/timeline_panel.rs:278` registers mute, lock,
  and hidden controls, but no delete-track header control is registered.
- `apps/composer/src-tauri/src/ui/mod.rs:554` clamps `timeline_h` without
  considering track count or minimum usable lane height.
- `apps/composer/src-tauri/src/ui/mod.rs:1239` creates track lane automation
  from clip nodes, so tracks with no clips do not get lane automation.
- `apps/composer/src-tauri/src/ui/state.rs:977` has a delete-track state method,
  but the UI has no matching reachable control.

## Test Review

- `apps/composer/src-tauri/tests/plan_ui_e2e.rs:177` clicks Add Track and only
  asserts the generic `composer.automatic.track_lanes` placeholder exists.
- Existing track control tests verify muted, locked, and hidden booleans, but
  not lane bounds, header bounds, overlap, or click-region rebuilding.

## Required Test Shape

- Expose automation nodes for every track header and lane, including empty
  tracks, with track id, index, bounds, hidden/muted state, and enabled controls.
- Add a track and assert all lane heights match the expected recalculated value
  and the new header node is present.
- Delete a track through a rendered control and assert remaining lane heights
  expand evenly.
- Drag the timeline splitter and assert lane bounds refill the new timeline
  area.
- Create many tracks and assert controls remain non-overlapping, reachable, and
  have valid hit targets.
- Toggle hidden and muted states, then assert lane bounds and click regions are
  still valid.

## Required Changes

- Add a reachable delete-track control or remove the unreachable UI action from
  this flow and define the intended deletion path.
- Create lane/header automation from `project.timeline.tracks`, not only from
  clips.
- Add a minimum lane-height policy with scrolling or adaptive controls for
  many-track timelines.
- Rebuild click regions from the same layout model used for lane rendering.

## Verification

- `cargo test -p tench-composer automatic_track_lane_layout`
- `cargo test -p tench-composer composer_plan_project_timeline_controls_use_real_events_ui_e2e`
- `cargo test -p tench-ui-test ui_automation`
- `git diff --check`
