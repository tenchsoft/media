# Implementation Plan: Context Menu Shuffle Item

## Feature ID
player/context-menu-shuffle-item

## Spec Reference
`plans/spec/player/context-menu-shuffle-item.md`

## Design Reference
`plans/design/player/context-menu-shuffle-item.md`

## Implementation Strategy

### Locate
- File: `apps/player/src-tauri/src/ui/paint_overlays.rs`
- Search: `grep -n "context_menu\|shuffle" apps/player/src-tauri/src/ui/paint_overlays.rs`
- State: `apps/player/src-tauri/src/ui/state.rs` — `PlayerState::shuffle`

### Change Recipe
1. paint_overlays.rs에서 컨텍스트 메뉴에 셔플 항목 추가
2. 클릭 시 state.shuffle 토글
3. 체크 표시로 현재 상태 표시

### Find Strategies
- `grep -rn "shuffle" apps/player/src-tauri/src/ui/state.rs`
- `grep -rn "context_menu" apps/player/src-tauri/src/ui/paint_overlays.rs`

### Debug ID
`context-menu-shuffle-item`

### Verification
- cargo check -p player
- 컨텍스트 메뉴에서 셔플 토글
