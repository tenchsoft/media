# Implementation Plan: Automatic GIF Recording Frame Capture

## Feature ID
player/automatic-gif-recording-frame-capture

## Spec Reference
`plans/spec/player/automatic-gif-recording-frame-capture.md`

## Design Reference
`plans/design/player/automatic-gif-recording-frame-capture.md`
## Background Reference
`plans/background/player/automatic-gif-recording-frame-capture.md`

## Implementation Strategy

### Locate
- File: `apps/player/src-tauri/src/gst_backend.rs`
- Search: `grep -n "gif\|capture\|frame" apps/player/src-tauri/src/gst_backend.rs`
- State: `apps/player/src-tauri/src/ui/state.rs` — `PlayerState::gif_recording`

### Change Recipe
1. gst_backend.rs에서 GIF 녹화 모드 시 프레임 캡처
2. 설정된 FPS에 따라 프레임 간격 결정
3. 캡처된 프레임을 버퍼에 저장
4. 녹화 종료 시 프레임을 GIF로 인코딩

### Find Strategies
- `grep -rn "gif_recording" apps/player/src-tauri/src/ui/state.rs`
- `grep -rn "capture_frame" apps/player/src-tauri/src/gst_backend.rs`

### Debug ID
`automatic-gif-recording-frame-capture`

### Verification
- cargo check -p player
- GIF 녹화 중 프레임 캡처 동작
