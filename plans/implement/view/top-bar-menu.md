# Implementation Plan: Top Bar Menu

## Feature ID
view/top-bar-menu

## Spec Reference
`plans/spec/view/top-bar-menu.md`

## Design Reference
`plans/design/view/top-bar-menu.md`

## Implementation Strategy

### Locate
- File: `apps/view/src-tauri/src/ui/controls.rs`
- State: `apps/view/src-tauri/src/ui/state.rs` — `ViewState::menu_open`

### Change Recipe
1. 상단 바 메뉴 버튼 렌더링
2. 클릭 시 드롭다운 메뉴 표시 (파일, 편집, 보기, 도구)
3. 메뉴 항목 클릭 시 해당 기능 실행

### Find Strategies
- `grep -rn "menu_open" apps/view/src-tauri/src/ui/state.rs`

### Debug ID
`top-bar-menu`

### Verification
- cargo check -p view
