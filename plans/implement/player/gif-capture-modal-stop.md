# Implementation Plan: GIF Capture Modal Stop

## Feature ID
player/gif-capture-modal-stop

## Spec Reference
`plans/spec/player/gif-capture-modal-stop.md`

## Design Reference
`plans/design/player/gif-capture-modal-stop.md`

## Implementation Strategy

### Locate
- File: `apps/player/src-tauri/src/ui/paint_panels.rs`
- Search: `grep -n "gif_capture\|gif_stop" apps/player/src-tauri/src/ui/paint_panels.rs`
- State: `apps/player/src-tauri/src/ui/state.rs` — `PlayerState::gif_recording`

### Change Recipe
1. paint_panels.rs에서 GIF 캡처 중지 버튼 렌더링
2. 클릭 시 gif_recording = false
3. 캡처된 프레임을 GIF로 인코딩
4. 저장 완료 토스트 표시

### Find Strategies
- `grep -rn "gif_recording" apps/player/src-tauri/src/ui/state.rs`
- `grep -rn "gif" apps/player/src-tauri/src/ui/paint_panels.rs`

### Debug ID
`gif-capture-modal-stop`

### Verification
- cargo check -p player
