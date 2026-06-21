# Implementation Plan: Context Menu Stop Item

## Feature ID
player/context-menu-stop-item

## Spec Reference
`plans/spec/player/context-menu-stop-item.md`

## Design Reference
`plans/design/player/context-menu-stop-item.md`

## Implementation Strategy

### Locate
- File: `apps/player/src-tauri/src/ui/paint_overlays.rs`
- Search: `grep -n "context_menu\|stop" apps/player/src-tauri/src/ui/paint_overlays.rs`
- State: `apps/player/src-tauri/src/ui/state.rs` — `PlayerState::playing`, `PlayerState::position`

### Change Recipe
1. paint_overlays.rs에서 컨텍스트 메뉴에 정지 항목 추가
2. 클릭 시 재생 중지 + position = 0
3. gst_backend.rs에서 stop 호출

### Find Strategies
- `grep -rn "playing" apps/player/src-tauri/src/ui/state.rs`
- `grep -rn "context_menu" apps/player/src-tauri/src/ui/paint_overlays.rs`

### Debug ID
`context-menu-stop-item`

### Verification
- cargo check -p player
- 컨텍스트 메뉴에서 정지
