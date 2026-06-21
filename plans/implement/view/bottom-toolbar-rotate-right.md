# Implementation Plan: Bottom Toolbar Rotate Right

## Feature ID
view/bottom-toolbar-rotate-right

## Spec Reference
`plans/spec/view/bottom-toolbar-rotate-right.md`

## Design Reference
`plans/design/view/bottom-toolbar-rotate-right.md`

## Implementation Strategy

### Locate
- File: `apps/view/src-tauri/src/ui/controls.rs`
- State: `apps/view/src-tauri/src/ui/state.rs` — `ViewState::rotation`

### Change Recipe
1. 하단 도구 모음에서 오른쪽 회전 버튼 렌더링
2. 클릭 시 rotation += 90도
3. 캔버스 이미지 즉시 회전 반영

### Find Strategies
- `grep -rn "rotation" apps/view/src-tauri/src/ui/state.rs`

### Debug ID
`bottom-toolbar-rotate-right`

### Verification
- cargo check -p view
