# Implementation Plan: Bottom Fullscreen Button

## Feature ID
player/bottom-fullscreen-button

## Spec Reference
`plans/spec/player/bottom-fullscreen-button.md`

## Design Reference
`plans/design/player/bottom-fullscreen-button.md`

## Implementation Strategy

### Locate
- File: `apps/player/src-tauri/src/ui/paint_controls.rs`
- Search: `grep -n "fullscreen" apps/player/src-tauri/src/ui/paint_controls.rs`
- State: `apps/player/src-tauri/src/ui/state.rs` — `PlayerState::fullscreen`

### Change Recipe
1. paint_controls.rs에서 전체화면 버튼 렌더링
2. 클릭 시 state.fullscreen 토글
3. Tauri 창 API로 전체화면 전환
4. 컨트롤 바 자동 숨김 타이머 시작

### Find Strategies
- `grep -rn "fullscreen" apps/player/src-tauri/src/ui/state.rs`
- `grep -rn "fullscreen" apps/player/src-tauri/src/ui/paint_controls.rs`

### Debug ID
`bottom-fullscreen-button`

### Verification
- cargo check -p player
- 전체화면 전환/해제 동작
