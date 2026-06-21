# Automatic Media Metadata Display State Fix Plan

## Source Plan

- `plans/composer/automatic-media-metadata-display-behavior-work-plan.md`

## Gap Analysis

Composer stores optional media metadata and paints a metadata line, but the
metadata contract is incomplete. Actual media FPS and audio-specific metadata
are not extracted, automation hides row metadata, and tests only check import
count.

## Plan Requirements Not Met

- Imported media rows do not expose resolution, duration, FPS, or type badge
  metadata through automation.
- Tests do not verify video metadata display.
- Tests do not verify audio rows omit dimensions and show audio-relevant
  metadata.
- Tests do not verify metadata-extraction failure still renders the row with
  name and type badge.
- Tests do not verify multiple imported files keep independent metadata.
- Actual media FPS is not extracted from the media runtime; the stored FPS uses
  project timeline FPS.
- Audio channels and sample rate are not populated from the media runtime.

## Code Review

- `apps/composer/src-tauri/src/ui/state.rs:596` calls
  `extract_media_metadata` during import and stores optional fields on
  `MediaAsset`.
- `apps/composer/src-tauri/src/ui/state.rs:631` reads metadata from
  `tench_media_runtime::composer::import_media_files`, but the runtime
  `MediaInfo` only exposes duration, width, height, and file size.
- `apps/composer/src-tauri/src/ui/state.rs:639` stores `Some(project_fps)` as the
  asset FPS instead of a media-derived FPS.
- `apps/composer/src-tauri/src/ui/left_panel.rs:157` renders the media type
  badge and `:200` builds metadata text from present optional fields.
- `apps/composer/src-tauri/src/ui/mod.rs:1301` exposes generic
  `composer.media.asset.{idx}` nodes without metadata values.

## Test Review

- `apps/composer/src-tauri/tests/plan_ui_e2e.rs:125` imports media and asserts
  only that the media-bin count increased.
- `apps/composer/src-tauri/src/ui/state.rs:1160` unit-tests import count and
  active left tab only.
- There is no fixture for video metadata, audio metadata, extraction failure, or
  multiple independent rows.

## Required Test Shape

- Import a video fixture with known width, height, duration, and FPS and assert
  the row automation value contains those fields plus the video badge.
- Import an audio fixture and assert the row shows duration/audio metadata but no
  dimensions.
- Simulate metadata extraction failure and assert the row still exposes name and
  type badge.
- Import multiple files with different metadata and assert each row value is
  independent.

## Required Changes

- Extend media runtime metadata to include FPS and audio channel/sample-rate data
  when available.
- Keep metadata stored on `MediaAsset` and avoid recomputing it during paint.
- Add stable automation values for media asset name, type badge, and metadata
  text.
- Add metadata display tests for video, audio, failure, and multiple files.

## Verification

- `cargo test -p tench-composer automatic_media_metadata_display`
- `cargo test -p tench-composer composer_plan_timeline_controls_use_real_events_ui_e2e`
- `cargo test -p tench-ui-test ui_automation`
- `git diff --check`
