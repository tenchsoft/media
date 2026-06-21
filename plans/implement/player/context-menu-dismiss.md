# Implementation Plan: Context Menu Dismiss

## Feature ID
player/context-menu-dismiss

## Spec Reference
`plans/spec/player/context-menu-dismiss.md`

## Design Reference
`plans/design/player/context-menu-dismiss.md`

## Implementation Strategy

### Locate
- File: `apps/player/src-tauri/src/ui/mod.rs`
- Search: `grep -n "context_menu\|dismiss" apps/player/src-tauri/src/ui/mod.rs`
- State: `apps/player/src-tauri/src/ui/state.rs` — `PlayerState::context_menu_open`

### Change Recipe
1. 컨텍스트 메뉴 외부 클릭 시 context_menu_open = false
2. Escape 키로 메뉴 닫기
3. 메뉴 항목 선택 후 자동 닫기

### Find Strategies
- `grep -rn "context_menu_open" apps/player/src-tauri/src/ui/state.rs`
- `grep -rn "context_menu" apps/player/src-tauri/src/ui/mod.rs`

### Debug ID
`context-menu-dismiss`

### Verification
- cargo check -p player
- 컨텍스트 메뉴 닫기 동작
