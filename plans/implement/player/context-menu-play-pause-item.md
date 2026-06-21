# Implementation Plan: Context Menu Play Pause Item

## Feature ID
player/context-menu-play-pause-item

## Spec Reference
`plans/spec/player/context-menu-play-pause-item.md`

## Design Reference
`plans/design/player/context-menu-play-pause-item.md`

## Implementation Strategy

### Locate
- File: `apps/player/src-tauri/src/ui/paint_overlays.rs`
- Search: `grep -n "context_menu\|play_pause" apps/player/src-tauri/src/ui/paint_overlays.rs`
- State: `apps/player/src-tauri/src/ui/state.rs` — `PlayerState::playing`

### Change Recipe
1. paint_overlays.rs에서 컨텍스트 메뉴에 재생/일시정지 항목 추가
2. playing 상태에 따라 텍스트 변경 (재생/일시정지)
3. 클릭 시 state.playing 토글

### Find Strategies
- `grep -rn "playing" apps/player/src-tauri/src/ui/state.rs`
- `grep -rn "context_menu" apps/player/src-tauri/src/ui/paint_overlays.rs`

### Debug ID
`context-menu-play-pause-item`

### Verification
- cargo check -p player
- 컨텍스트 메뉴에서 재생/일시정지
