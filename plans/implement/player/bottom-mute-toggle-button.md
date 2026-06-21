# Implementation Plan: Bottom Mute Toggle Button

## Feature ID
player/bottom-mute-toggle-button

## Spec Reference
`plans/spec/player/bottom-mute-toggle-button.md`

## Design Reference
`plans/design/player/bottom-mute-toggle-button.md`

## Implementation Strategy

### Locate
- File: `apps/player/src-tauri/src/ui/paint_controls.rs`
- Search: `grep -n "mute\|volume" apps/player/src-tauri/src/ui/paint_controls.rs`
- State: `apps/player/src-tauri/src/ui/state.rs` — `PlayerState::muted`

### Change Recipe
1. paint_controls.rs에서 음소거 버튼 렌더링
2. 클릭 시 state.muted 토글
3. gst_backend.rs에서 mute 설정 적용
4. 음소거 상태에 따라 아이콘 변경 (스피커/음소거)

### Find Strategies
- `grep -rn "muted" apps/player/src-tauri/src/ui/state.rs`
- `grep -rn "mute" apps/player/src-tauri/src/ui/paint_controls.rs`

### Debug ID
`bottom-mute-toggle-button`

### Verification
- cargo check -p player
- 음소거 토글 동작
