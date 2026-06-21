# Implementation Plan: Bottom Aspect Mode Button

## Feature ID
player/bottom-aspect-mode-button

## Spec Reference
`plans/spec/player/bottom-aspect-mode-button.md`

## Design Reference
`plans/design/player/bottom-aspect-mode-button.md`

## Implementation Strategy

### Locate
- File: `apps/player/src-tauri/src/ui/paint_controls.rs`
- Search: `grep -n "aspect\|aspect_mode" apps/player/src-tauri/src/ui/paint_controls.rs`
- State: `apps/player/src-tauri/src/ui/state.rs` — `PlayerState::aspect_mode`

### Change Recipe
1. paint_controls.rs에서 화면 비율 버튼 렌더링
2. 클릭 시 aspect_mode 토글 (fit / fill / stretch / original)
3. paint_video.rs에서 aspect_mode에 따라 비디오 스케일링
4. 현재 모드 아이콘 표시

### Find Strategies
- `grep -rn "aspect_mode" apps/player/src-tauri/src/ui/state.rs`
- `grep -rn "aspect" apps/player/src-tauri/src/ui/paint_controls.rs`

### Debug ID
`bottom-aspect-mode-button`

### Verification
- cargo check -p player
- 화면 비율 모드 전환 동작
