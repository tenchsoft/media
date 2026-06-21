# Implementation Plan: GIF Options Modal Start Recording

## Feature ID
player/gif-options-modal-start-recording

## Spec Reference
`plans/spec/player/gif-options-modal-start-recording.md`

## Design Reference
`plans/design/player/gif-options-modal-start-recording.md`

## Implementation Strategy

### Locate
- File: `apps/player/src-tauri/src/ui/paint_panels.rs`
- Search: `grep -n "gif_options\|gif_start" apps/player/src-tauri/src/ui/paint_panels.rs`
- State: `apps/player/src-tauri/src/ui/state.rs` — `PlayerState::gif_options_modal_open`, `PlayerState::gif_recording`

### Change Recipe
1. paint_panels.rs에서 GIF 옵션 모달 "녹화 시작" 버튼 렌더링
2. 클릭 시 설정된 FPS/크기로 녹화 시작
3. gif_options_modal_open = false, gif_recording = true
4. gst_backend.rs에서 프레임 캡처 시작

### Find Strategies
- `grep -rn "gif_options\|gif_recording" apps/player/src-tauri/src/ui/state.rs`
- `grep -rn "gif" apps/player/src-tauri/src/ui/paint_panels.rs`

### Debug ID
`gif-options-modal-start-recording`

### Verification
- cargo check -p player
