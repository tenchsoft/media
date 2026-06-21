# Implementation Plan: Automatic GIF Recording Indicator

## Feature ID
player/automatic-gif-recording-indicator

## Spec Reference
`plans/spec/player/automatic-gif-recording-indicator.md`

## Design Reference
`plans/design/player/automatic-gif-recording-indicator.md`
## Background Reference
`plans/background/player/automatic-gif-recording-indicator.md`

## Implementation Strategy

### Locate
- File: `apps/player/src-tauri/src/ui/paint_overlays.rs`
- Search: `grep -n "gif\|recording" apps/player/src-tauri/src/ui/paint_overlays.rs`
- State: `apps/player/src-tauri/src/ui/state.rs` — `PlayerState::gif_recording`

### Change Recipe
1. paint_overlays.rs에서 gif_recording이 true일 때 인디케이터 렌더링
2. 빨간 원 + "REC" 텍스트 + 경과 시간 표시
3. 깜빡임 애니메이션 적용
4. 녹화 종료 시 인디케이터 제거

### Find Strategies
- `grep -rn "gif_recording" apps/player/src-tauri/src/ui/state.rs`
- `grep -rn "recording" apps/player/src-tauri/src/ui/paint_overlays.rs`

### Debug ID
`automatic-gif-recording-indicator`

### Verification
- cargo check -p player
- GIF 녹화 중 REC 인디케이터 표시
