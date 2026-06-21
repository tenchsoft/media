# Implementation Plan: Context Menu Rotate Left

## Feature ID
view/context-menu-rotate-left

## Spec Reference
`plans/spec/view/context-menu-rotate-left.md`

## Design Reference
`plans/design/view/context-menu-rotate-left.md`

## Implementation Strategy

### Locate
- File: `apps/view/src-tauri/src/ui/controls.rs`
- State: `apps/view/src-tauri/src/ui/state.rs` — `ViewState::rotation`

### Change Recipe
1. 우클릭 컨텍스트 메뉴에 "왼쪽으로 회전" 항목 렌더링
2. 클릭 시 rotation -= 90도
3. 캔버스 즉시 반영

### Find Strategies
- `grep -rn "rotation" apps/view/src-tauri/src/ui/state.rs`

### Debug ID
`context-menu-rotate-left`

### Verification
- cargo check -p view
