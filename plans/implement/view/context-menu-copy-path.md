# Implementation Plan: Context Menu Copy Path

## Feature ID
view/context-menu-copy-path

## Spec Reference
`plans/spec/view/context-menu-copy-path.md`

## Design Reference
`plans/design/view/context-menu-copy-path.md`

## Implementation Strategy

### Locate
- File: `apps/view/src-tauri/src/ui/controls.rs`
- State: `apps/view/src-tauri/src/ui/state.rs` — `ViewState::current_image`

### Change Recipe
1. 우클릭 컨텍스트 메뉴에 "경로 복사" 항목 렌더링
2. 클릭 시 현재 파일 경로를 클립보드에 복사

### Find Strategies
- `grep -rn "context_menu" apps/view/src-tauri/src/ui/controls.rs`

### Debug ID
`context-menu-copy-path`

### Verification
- cargo check -p view
