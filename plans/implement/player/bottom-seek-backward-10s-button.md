# Implementation Plan: Bottom Seek Backward 10s Button

## Feature ID
player/bottom-seek-backward-10s-button

## Spec Reference
`plans/spec/player/bottom-seek-backward-10s-button.md`

## Design Reference
`plans/design/player/bottom-seek-backward-10s-button.md`

## Implementation Strategy

### Locate
- File: `apps/player/src-tauri/src/ui/paint_controls.rs`
- Search: `grep -n "seek_back\|backward" apps/player/src-tauri/src/ui/paint_controls.rs`
- State: `apps/player/src-tauri/src/ui/state.rs` — `PlayerState::position`

### Change Recipe
1. paint_controls.rs에서 뒤로 10초 버튼 렌더링
2. 클릭 시 position = max(0, position - 10s)
3. gst_backend.rs에서 seek 호출
4. 아이콘: 왼쪽 화살표 + "10"

### Find Strategies
- `grep -rn "position" apps/player/src-tauri/src/ui/state.rs`
- `grep -rn "seek_back" apps/player/src-tauri/src/ui/paint_controls.rs`

### Debug ID
`bottom-seek-backward-10s-button`

### Verification
- cargo check -p player
- 10초 뒤로 이동 동작
