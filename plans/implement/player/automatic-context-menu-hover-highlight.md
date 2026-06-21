# Implementation Plan: Automatic Context Menu Hover Highlight

## Feature ID
player/automatic-context-menu-hover-highlight

## Spec Reference
`plans/spec/player/automatic-context-menu-hover-highlight.md`

## Design Reference
`plans/design/player/automatic-context-menu-hover-highlight.md`
## Background Reference
`plans/background/player/automatic-context-menu-hover-highlight.md`

## Implementation Strategy

### Locate
- File: `apps/player/src-tauri/src/ui/paint_overlays.rs`
- Search: `grep -n "context_menu\|hover" apps/player/src-tauri/src/ui/paint_overlays.rs`
- State: `apps/player/src-tauri/src/ui/state.rs` — `PlayerState::context_menu_hover_index`

### Change Recipe
1. paint_overlays.rs에서 컨텍스트 메뉴 아이템 렌더링
2. 마우스 위치 기반으로 hovered_item 인덱스 계산
3. hovered 아이템에 하이라이트 배경색 적용
4. 마우스 이동 시 실시간 업데이트

### Find Strategies
- `grep -rn "context_menu" apps/player/src-tauri/src/ui/state.rs`
- `grep -rn "hover" apps/player/src-tauri/src/ui/paint_overlays.rs`

### Debug ID
`automatic-context-menu-hover-highlight`

### Verification
- cargo check -p player
- 메뉴 아이템 호버 시 하이라이트
