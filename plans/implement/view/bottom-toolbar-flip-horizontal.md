# Implementation Plan: Bottom Toolbar Flip Horizontal

## Feature ID
view/bottom-toolbar-flip-horizontal

## Spec Reference
`plans/spec/view/bottom-toolbar-flip-horizontal.md`

## Design Reference
`plans/design/view/bottom-toolbar-flip-horizontal.md`

## Implementation Strategy

### Locate
- File: `apps/view/src-tauri/src/ui/controls.rs`
- State: `apps/view/src-tauri/src/ui/state.rs` — `ViewState::flip_h`

### Change Recipe
1. 하단 도구 모음에서 좌우 반전 버튼 렌더링
2. 클릭 시 flip_h 토글
3. 캔버스 이미지 즉시 반전 반영

### Find Strategies
- `grep -rn "flip_h" apps/view/src-tauri/src/ui/state.rs`

### Debug ID
`bottom-toolbar-flip-horizontal`

### Verification
- cargo check -p view
