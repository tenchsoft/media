# Implementation Plan: Info Audio Device Row

## Feature ID
player/info-audio-device-row

## Spec Reference
`plans/spec/player/info-audio-device-row.md`

## Design Reference
`plans/design/player/info-audio-device-row.md`

## Implementation Strategy

### Locate
- File: `apps/player/src-tauri/src/ui/paint_panels.rs`
- Search: `grep -n "audio_device\|info_panel" apps/player/src-tauri/src/ui/paint_panels.rs`
- State: `apps/player/src-tauri/src/ui/state.rs` — `PlayerState::media_info`

### Change Recipe
1. paint_panels.rs에서 정보 패널에 오디오 장치 행 렌더링
2. 현재 오디오 출력 장치 이름 표시
3. gst_backend.rs에서 오디오 장치 정보 조회

### Find Strategies
- `grep -rn "media_info" apps/player/src-tauri/src/ui/state.rs`
- `grep -rn "audio_device" apps/player/src-tauri/src/ui/paint_panels.rs`

### Debug ID
`info-audio-device-row`

### Verification
- cargo check -p player
