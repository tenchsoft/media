# Implementation Plan: Automatic Buffering Progress Bar

## Feature ID
player/automatic-buffering-progress-bar

## Spec Reference
`plans/spec/player/automatic-buffering-progress-bar.md`

## Design Reference
`plans/design/player/automatic-buffering-progress-bar.md`
## Background Reference
`plans/background/player/automatic-buffering-progress-bar.md`

## Implementation Strategy

### Locate
- File: `apps/player/src-tauri/src/ui/paint_controls.rs`
- Search: `grep -n "buffering\|buffer_progress" apps/player/src-tauri/src/ui/paint_controls.rs`
- State: `apps/player/src-tauri/src/ui/state.rs` — `PlayerState::buffer_progress`

### Change Recipe
1. gst_backend.rs에서 버퍼링 이벤트 수신
2. state.rs의 buffer_progress 값 업데이트
3. paint_controls.rs에서 시크바 위에 버퍼링 진행률 렌더링
4. 100% 도달 시 버퍼링 표시 해제

### Find Strategies
- `grep -rn "buffer" apps/player/src-tauri/src/ui/state.rs`
- `grep -rn "buffer" apps/player/src-tauri/src/ui/paint_controls.rs`

### Debug ID
`automatic-buffering-progress-bar`

### Verification
- cargo check -p player
- 네트워크 스트리밍 시 버퍼링 바 표시
