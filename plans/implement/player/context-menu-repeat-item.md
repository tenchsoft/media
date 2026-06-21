# Implementation Plan: Context Menu Repeat Item

## Feature ID
player/context-menu-repeat-item

## Spec Reference
`plans/spec/player/context-menu-repeat-item.md`

## Design Reference
`plans/design/player/context-menu-repeat-item.md`

## Implementation Strategy

### Locate
- File: `apps/player/src-tauri/src/ui/paint_overlays.rs`
- Search: `grep -n "context_menu\|repeat" apps/player/src-tauri/src/ui/paint_overlays.rs`
- State: `apps/player/src-tauri/src/ui/state.rs` — `PlayerState::repeat_mode`

### Change Recipe
1. paint_overlays.rs에서 컨텍스트 메뉴에 반복 모드 항목 추가
2. 하위 메뉴로 none/one/all 표시
3. 현재 모드 체크 표시
4. 선택 시 repeat_mode 변경

### Find Strategies
- `grep -rn "repeat_mode" apps/player/src-tauri/src/ui/state.rs`
- `grep -rn "context_menu" apps/player/src-tauri/src/ui/paint_overlays.rs`

### Debug ID
`context-menu-repeat-item`

### Verification
- cargo check -p player
- 컨텍스트 메뉴에서 반복 모드 설정
