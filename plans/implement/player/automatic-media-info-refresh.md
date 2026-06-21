# Implementation Plan: Automatic Media Info Refresh

## Feature ID
player/automatic-media-info-refresh

## Spec Reference
`plans/spec/player/automatic-media-info-refresh.md`

## Design Reference
`plans/design/player/automatic-media-info-refresh.md`
## Background Reference
`plans/background/player/automatic-media-info-refresh.md`

## Implementation Strategy

### Locate
- File: `apps/player/src-tauri/src/gst_backend.rs`
- Search: `grep -n "media_info\|duration\|metadata" apps/player/src-tauri/src/gst_backend.rs`
- State: `apps/player/src-tauri/src/ui/state.rs` — `PlayerState::media_info`

### Change Recipe
1. gst_backend.rs에서 미디어 로드 시 메타데이터 추출
2. 해상도, 코덱, 비트레이트, 재생 시간 등 파싱
3. state.media_info에 저장
4. 새 파일 로드 시 자동 갱신

### Find Strategies
- `grep -rn "media_info" apps/player/src-tauri/src/ui/state.rs`
- `grep -rn "duration\|metadata" apps/player/src-tauri/src/gst_backend.rs`

### Debug ID
`automatic-media-info-refresh`

### Verification
- cargo check -p player
- 파일 열 때 미디어 정보 자동 갱신
