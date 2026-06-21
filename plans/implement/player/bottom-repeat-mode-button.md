# Implementation Plan: Bottom Repeat Mode Button

## Feature ID
player/bottom-repeat-mode-button

## Spec Reference
`plans/spec/player/bottom-repeat-mode-button.md`

## Design Reference
`plans/design/player/bottom-repeat-mode-button.md`

## Implementation Strategy

### Locate
- File: `apps/player/src-tauri/src/ui/paint_controls.rs`
- Search: `grep -n "repeat" apps/player/src-tauri/src/ui/paint_controls.rs`
- State: `apps/player/src-tauri/src/ui/state.rs` — `PlayerState::repeat_mode`

### Change Recipe
1. paint_controls.rs에서 반복 모드 버튼 렌더링
2. 클릭 시 repeat_mode 순환 (none → one → all → none)
3. 아이콘으로 현재 모드 표시
4. gst_backend.rs에서 반복 설정 적용

### Find Strategies
- `grep -rn "repeat_mode" apps/player/src-tauri/src/ui/state.rs`
- `grep -rn "repeat" apps/player/src-tauri/src/ui/paint_controls.rs`

### Debug ID
`bottom-repeat-mode-button`

### Verification
- cargo check -p player
- 반복 모드 순환 동작
