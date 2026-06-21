# Implementation Plan: Bottom Toolbar Flip Vertical

## Feature ID
view/bottom-toolbar-flip-vertical

## Spec Reference
`plans/spec/view/bottom-toolbar-flip-vertical.md`

## Design Reference
`plans/design/view/bottom-toolbar-flip-vertical.md`

## Implementation Strategy

### Locate
- File: `apps/view/src-tauri/src/ui/controls.rs`
- State: `apps/view/src-tauri/src/ui/state.rs` — `ViewState::flip_v`

### Change Recipe
1. 하단 도구 모음에서 상하 반전 버튼 렌더링
2. 클릭 시 flip_v 토글
3. 캔버스 이미지 즉시 반전 반영

### Find Strategies
- `grep -rn "flip_v" apps/view/src-tauri/src/ui/state.rs`

### Debug ID
`bottom-toolbar-flip-vertical`

### Verification
- cargo check -p view
