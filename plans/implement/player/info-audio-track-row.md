# Implementation Plan: Info Audio Track Row

## Feature ID
player/info-audio-track-row

## Spec Reference
`plans/spec/player/info-audio-track-row.md`

## Design Reference
`plans/design/player/info-audio-track-row.md`

## Implementation Strategy

### Locate
- File: `apps/player/src-tauri/src/ui/paint_panels.rs`
- Search: `grep -n "audio_track\|info_panel" apps/player/src-tauri/src/ui/paint_panels.rs`
- State: `apps/player/src-tauri/src/ui/state.rs` — `PlayerState::media_info`

### Change Recipe
1. paint_panels.rs에서 정보 패널에 오디오 트랙 행 렌더링
2. 코덱, 채널, 샘플레이트 정보 표시
3. gst_backend.rs에서 오디오 트랙 메타데이터 조회

### Find Strategies
- `grep -rn "media_info" apps/player/src-tauri/src/ui/state.rs`
- `grep -rn "audio_track" apps/player/src-tauri/src/ui/paint_panels.rs`

### Debug ID
`info-audio-track-row`

### Verification
- cargo check -p player
