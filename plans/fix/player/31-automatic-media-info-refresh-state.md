# Automatic Media Info Refresh State

## Source Plan
- `plans/player/automatic-media-info-refresh-work-plan.md`

## Gap Analysis
`MediaEvent::Loaded` refreshes duration, resolution, selected metadata fields, and audio devices, but built-in subtitle labels are never populated from backend metadata. `n_builtin_subtitle_tracks` is polled from the backend, while `builtin_subtitle_labels` remains empty outside fixture state. See `apps/player/src-tauri/src/ui/app.rs:389`, `apps/player/src-tauri/src/ui/app.rs:548`, and `apps/player/src-tauri/src/ui/state.rs:624`.

Audio track rows in the Info drawer are queried directly from the backend during paint rather than being refreshed into canonical `PlayerState` on media load. This makes the loaded media info state hard to assert consistently and leaves no stored track count/current-track state for automation. See `apps/player/src-tauri/src/ui/paint_panels.rs:820`.

The existing E2E opens the Info drawer and asserts example fixture controls are present. It does not load multiple media files, inject different metadata/codecs/tracks, verify Info and Subtitles drawers refresh, or assert audio device refresh from backend events. See `apps/player/src-tauri/tests/plan_ui_e2e.rs:450`.

The automatic media info status node is always emitted with no value for file name, resolution, codec, audio track count, subtitle track count, or audio device count. See `apps/player/src-tauri/src/ui/app.rs:2318`.

## Plan Requirements Not Met
- Media load refresh must populate built-in subtitle labels, not only subtitle count.
- Audio track count/current state must be refreshable and assertable from canonical state or explicit backend event state.
- Tests must verify media info refresh after loading different media files with different metadata and tracks.
- Tests must verify Info and Subtitles drawers update after media load.
- Automation must expose enough media info refresh state for deterministic assertions.

## Required Test Shape
- Inject a `Loaded` event plus fake backend metadata for codec, resolution, frame rate, title/artist/album, audio track count, subtitle track count/labels, and audio devices.
- Assert `PlayerState.media_info`, audio device list, built-in subtitle labels, and drawer automation nodes match the injected metadata.
- Load a second media fixture with different metadata and assert stale values are replaced in Info and Subtitles drawers.
- Open a side panel or resize after refresh and assert the same media info selectors and values remain correct.

## Required Changes
- Add backend metadata plumbing for built-in subtitle labels and audio track state into `PlayerState` or explicit backend event state.
- Expose media info refresh values in `player.automatic.media_info_refresh`.
- Add deterministic fake backend/media metadata tests for Info and Subtitles drawer refresh.

## Verification
- `cargo test -p tench-player --test plan_ui_e2e automatic_media_info_refresh`
- `cargo test -p tench-player`
