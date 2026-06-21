# Implementation Plan: Context Menu Fullscreen Item

## Feature ID
player/context-menu-fullscreen-item

## Spec Reference
`plans/spec/player/context-menu-fullscreen-item.md`

## Design Reference
`plans/design/player/context-menu-fullscreen-item.md`

## Implementation Strategy

### Locate
- File: `apps/player/src-tauri/src/ui/paint_overlays.rs`
- Search: `grep -n "context_menu\|fullscreen" apps/player/src-tauri/src/ui/paint_overlays.rs`
- State: `apps/player/src-tauri/src/ui/state.rs` — `PlayerState::fullscreen`

### Change Recipe
1. paint_overlays.rs에서 컨텍스트 메뉴에 전체화면 항목 추가
2. 클릭 시 state.fullscreen 토글
3. Tauri 창 API로 전체화면 전환

### Find Strategies
- `grep -rn "fullscreen" apps/player/src-tauri/src/ui/state.rs`
- `grep -rn "context_menu" apps/player/src-tauri/src/ui/paint_overlays.rs`

### Debug ID
`context-menu-fullscreen-item`

### Verification
- cargo check -p player
- 컨텍스트 메뉴에서 전체화면 전환
