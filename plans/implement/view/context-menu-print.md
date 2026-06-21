# Implementation Plan: Context Menu Print

## Feature ID
view/context-menu-print

## Spec Reference
`plans/spec/view/context-menu-print.md`

## Design Reference
`plans/design/view/context-menu-print.md`

## Implementation Strategy

### Locate
- File: `apps/view/src-tauri/src/ui/controls.rs`
- State: `apps/view/src-tauri/src/ui/state.rs` — `ViewState::current_image`

### Change Recipe
1. 우클릭 컨텍스트 메뉴에 "인쇄" 항목 렌더링
2. 클릭 시 시스템 인쇄 대화상자 호출

### Find Strategies
- `grep -rn "print" apps/view/src-tauri/src/ui/controls.rs`

### Debug ID
`context-menu-print`

### Verification
- cargo check -p view
