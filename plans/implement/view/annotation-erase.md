# Implementation Plan: Annotation Erase

## Feature ID
view/annotation-erase

## Spec Reference
`plans/spec/view/annotation-erase.md`

## Design Reference
`plans/design/view/annotation-erase.md`

## Implementation Strategy

### Locate
- File: `apps/view/src-tauri/src/ui/tools.rs`
- State: `apps/view/src-tauri/src/ui/state.rs` — `ViewState::annotation_tool`

### Change Recipe
1. 지우개 도구 버튼 렌더링
2. 클릭 시 annotation_tool = Eraser
3. 캔버스에서 마우스 드래그로 주석 요소 제거

### Find Strategies
- `grep -rn "annotation_tool" apps/view/src-tauri/src/ui/state.rs`

### Debug ID
`annotation-erase`

### Verification
- cargo check -p view
