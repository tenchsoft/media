# Implementation Plan: Annotation Draw Rectangle

## Feature ID
view/annotation-draw-rectangle

## Spec Reference
`plans/spec/view/annotation-draw-rectangle.md`

## Design Reference
`plans/design/view/annotation-draw-rectangle.md`

## Implementation Strategy

### Locate
- File: `apps/view/src-tauri/src/ui/tools.rs`
- State: `apps/view/src-tauri/src/ui/state.rs` — `ViewState::annotation_tool`

### Change Recipe
1. 사각형 도구 버튼 렌더링
2. 클릭 시 annotation_tool = Rectangle
3. 캔버스에서 마우스 드래그로 사각형 그리기

### Find Strategies
- `grep -rn "annotation_tool" apps/view/src-tauri/src/ui/state.rs`

### Debug ID
`annotation-draw-rectangle`

### Verification
- cargo check -p view
