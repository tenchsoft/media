# Implementation Plan: Annotation Draw Arrow

## Feature ID
view/annotation-draw-arrow

## Spec Reference
`plans/spec/view/annotation-draw-arrow.md`

## Design Reference
`plans/design/view/annotation-draw-arrow.md`

## Implementation Strategy

### Locate
- File: `apps/view/src-tauri/src/ui/tools.rs`
- State: `apps/view/src-tauri/src/ui/state.rs` — `ViewState::annotation_tool`

### Change Recipe
1. 화살표 도구 버튼 렌더링
2. 클릭 시 annotation_tool = Arrow
3. 캔버스에서 마우스 드래그로 화살표 그리기

### Find Strategies
- `grep -rn "annotation_tool" apps/view/src-tauri/src/ui/state.rs`

### Debug ID
`annotation-draw-arrow`

### Verification
- cargo check -p view
