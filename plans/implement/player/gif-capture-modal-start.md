# Implementation Plan: GIF Capture Modal Start

## Feature ID
player/gif-capture-modal-start

## Spec Reference
`plans/spec/player/gif-capture-modal-start.md`

## Design Reference
`plans/design/player/gif-capture-modal-start.md`

## Implementation Strategy

### Locate
- File: `apps/player/src-tauri/src/ui/paint_panels.rs`
- Search: `grep -n "gif_capture\|gif_start" apps/player/src-tauri/src/ui/paint_panels.rs`
- State: `apps/player/src-tauri/src/ui/state.rs` — `PlayerState::gif_recording`

### Change Recipe
1. paint_panels.rs에서 GIF 캡처 시작 버튼 렌더링
2. 클릭 시 gif_recording = true
3. gst_backend.rs에서 프레임 캡처 시작
4. REC 인디케이터 표시

### Find Strategies
- `grep -rn "gif_recording" apps/player/src-tauri/src/ui/state.rs`
- `grep -rn "gif" apps/player/src-tauri/src/ui/paint_panels.rs`

### Debug ID
`gif-capture-modal-start`

### Verification
- cargo check -p player
