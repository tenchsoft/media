# Implementation Plan: Bottom Play Pause Button

## Feature ID
player/bottom-play-pause-button

## Spec Reference
`plans/spec/player/bottom-play-pause-button.md`

## Design Reference
`plans/design/player/bottom-play-pause-button.md`

## Implementation Strategy

### Locate
- File: `apps/player/src-tauri/src/ui/paint_controls.rs`
- Search: `grep -n "play_pause\|playing" apps/player/src-tauri/src/ui/paint_controls.rs`
- State: `apps/player/src-tauri/src/ui/state.rs` — `PlayerState::playing`

### Change Recipe
1. paint_controls.rs에서 재생/일시정지 버튼 렌더링
2. 클릭 시 state.playing 토글
3. gst_backend.rs에서 play/pause 호출
4. 아이콘: 재생 중일 때 일시정지, 일시정지일 때 재생

### Find Strategies
- `grep -rn "playing" apps/player/src-tauri/src/ui/state.rs`
- `grep -rn "play_pause" apps/player/src-tauri/src/ui/paint_controls.rs`

### Debug ID
`bottom-play-pause-button`

### Verification
- cargo check -p player
- 재생/일시정지 토글 동작
