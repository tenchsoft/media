# Implementation Plan: Bottom Seek Forward 10s Button

## Feature ID
player/bottom-seek-forward-10s-button

## Spec Reference
`plans/spec/player/bottom-seek-forward-10s-button.md`

## Design Reference
`plans/design/player/bottom-seek-forward-10s-button.md`

## Implementation Strategy

### Locate
- File: `apps/player/src-tauri/src/ui/paint_controls.rs`
- Search: `grep -n "seek_fwd\|forward" apps/player/src-tauri/src/ui/paint_controls.rs`
- State: `apps/player/src-tauri/src/ui/state.rs` — `PlayerState::position`, `PlayerState::duration`

### Change Recipe
1. paint_controls.rs에서 앞으로 10초 버튼 렌더링
2. 클릭 시 position = min(duration, position + 10s)
3. gst_backend.rs에서 seek 호출
4. 아이콘: 오른쪽 화살표 + "10"

### Find Strategies
- `grep -rn "position\|duration" apps/player/src-tauri/src/ui/state.rs`
- `grep -rn "seek_fwd" apps/player/src-tauri/src/ui/paint_controls.rs`

### Debug ID
`bottom-seek-forward-10s-button`

### Verification
- cargo check -p player
- 10초 앞으로 이동 동작
