# Implementation Plan: Annotation Draw Freehand

## Feature ID
view/annotation-draw-freehand

## Spec Reference
`plans/spec/view/annotation-draw-freehand.md`

## Design Reference
`plans/design/view/annotation-draw-freehand.md`

## Implementation Strategy

### Locate
- File: `apps/view/src-tauri/src/ui/tools.rs`
- State: `apps/view/src-tauri/src/ui/state.rs` — `ViewState::annotation_tool`

### Change Recipe
1. 자유 그리기 도구 버튼 렌더링
2. 클릭 시 annotation_tool = Freehand
3. 캔버스에서 마우스 드래그로 자유 곡선 그리기

### Find Strategies
- `grep -rn "annotation_tool" apps/view/src-tauri/src/ui/state.rs`

### Debug ID
`annotation-draw-freehand`

### Verification
- cargo check -p view
