# Implementation Plan: Context Menu Aspect Item

## Feature ID
player/context-menu-aspect-item

## Spec Reference
`plans/spec/player/context-menu-aspect-item.md`

## Design Reference
`plans/design/player/context-menu-aspect-item.md`

## Implementation Strategy

### Locate
- File: `apps/player/src-tauri/src/ui/paint_overlays.rs`
- Search: `grep -n "context_menu\|aspect" apps/player/src-tauri/src/ui/paint_overlays.rs`
- State: `apps/player/src-tauri/src/ui/state.rs` — `PlayerState::aspect_mode`

### Change Recipe
1. paint_overlays.rs에서 컨텍스트 메뉴에 화면 비율 항목 추가
2. 클릭 시 aspect_mode 순환
3. 현재 모드 체크 표시

### Find Strategies
- `grep -rn "aspect_mode" apps/player/src-tauri/src/ui/state.rs`
- `grep -rn "context_menu" apps/player/src-tauri/src/ui/paint_overlays.rs`

### Debug ID
`context-menu-aspect-item`

### Verification
- cargo check -p player
- 컨텍스트 메뉴에서 화면 비율 전환
