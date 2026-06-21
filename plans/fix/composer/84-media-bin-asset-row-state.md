# Media Bin Asset Row State

## Source Plan

- `plans/composer/media-bin-asset-row-control-work-plan.md`

## Gap Analysis

Media bin rows register `ClickAction::SelectClip(None)`, so clicking a row only clears the timeline clip selection and does not select a media source. See `apps/composer/src-tauri/src/ui/left_panel.rs:235` and `apps/composer/src-tauri/src/ui/mod.rs:221`.

`ComposerState` has clip selection fields but no selected media index or equivalent active media source state. See `apps/composer/src-tauri/src/ui/state.rs:282`.

`DragKind::MediaBin` exists and drop handling can create a clip from a media index, but no row path sets `DragKind::MediaBin`, so media rows cannot start the required drag-to-timeline flow. See `apps/composer/src-tauri/src/ui/state.rs:194`, `apps/composer/src-tauri/src/ui/mod.rs:617`, and `apps/composer/src-tauri/src/ui/mod.rs:488`.

The row renderer displays general metadata, but it does not render selected-row styling from media selection state and does not expose audio-specific metadata such as channels or sample rate. See `apps/composer/src-tauri/src/ui/left_panel.rs:200` and `crates/composer-core/src/project.rs:17`.

The current E2E coverage imports media and asserts the media-bin count. It does not click a media asset row, verify clip selection clearing, verify active media source state, drag media to the timeline, or test empty-area clicks. See `apps/composer/src-tauri/tests/plan_ui_e2e.rs:118`.

## Plan Requirements Not Met

- Each media asset row must register a row-level action carrying its media index or id.
- The app must maintain selected media state separately from `selected_clip_id`.
- Clicking a media row while a clip is selected must clear stale clip inspector state and set the media source.
- Media rows must start `DragKind::MediaBin` or an equivalent drag source for timeline drops.
- Selected-row styling and audio metadata must render from media-bin state.
- Empty media-bin area clicks must leave media selection unchanged.

## Required Test Shape

- Add a Composer UI automation test that clicks `composer.media.asset.0` and asserts selected media state is set while `selected_clip_id` is cleared.
- Select a timeline clip, click a media row, and assert the clip inspector no longer shows stale clip data.
- Create or import an audio asset, click its row, and assert audio metadata remains visible.
- Drag a media row to a timeline track and assert a clip is created from the selected media source.
- Click an empty media-bin area and assert selected media state is unchanged.

## Required Changes

- Add `selected_media_idx` or selected media id state to `ComposerState`.
- Add a dedicated media-row click action instead of using `SelectClip(None)`.
- Start `DragKind::MediaBin { media_idx }` from media row pointer down or an equivalent drag source action.
- Render selected media row styling and richer audio metadata from `MediaAsset`.
- Expose selected media state and row metadata through automation.

## Verification

- `cargo test -p tench-composer --test plan_ui_e2e media_bin_asset_row`
- `cargo test -p tench-composer`
