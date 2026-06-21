# Implementation Plan: Bottom GIF Toggle Button

## Feature ID
player/bottom-gif-toggle-button

## Spec Reference
`plans/spec/player/bottom-gif-toggle-button.md`

## Design Reference
`plans/design/player/bottom-gif-toggle-button.md`

## Implementation Strategy

### Locate
- File: `apps/player/src-tauri/src/ui/paint_controls.rs`
- Search: `grep -n "gif\|gif_toggle" apps/player/src-tauri/src/ui/paint_controls.rs`
- State: `apps/player/src-tauri/src/ui/state.rs` — `PlayerState::gif_recording`

### Change Recipe
1. paint_controls.rs에서 GIF 토글 버튼 렌더링
2. 클릭 시 GIF 옵션 모달 표시
3. 녹화 중일 때 빨간 아이콘으로 상태 표시
4. 녹화 중 클릭 시 녹화 중지

### Find Strategies
- `grep -rn "gif_recording" apps/player/src-tauri/src/ui/state.rs`
- `grep -rn "gif" apps/player/src-tauri/src/ui/paint_controls.rs`

### Debug ID
`bottom-gif-toggle-button`

### Verification
- cargo check -p player
- GIF 녹화 토글 동작
