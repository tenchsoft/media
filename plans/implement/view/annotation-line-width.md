# Implementation Plan: Annotation Line Width

## Feature ID
view/annotation-line-width

## Spec Reference
`plans/spec/view/annotation-line-width.md`

## Design Reference
`plans/design/view/annotation-line-width.md`

## Implementation Strategy

### Locate
- File: `apps/view/src-tauri/src/ui/tools.rs`
- State: `apps/view/src-tauri/src/ui/state.rs` — `ViewState::annotation_line_width`

### Change Recipe
1. 주석 도구 모음에서 선 두께 슬라이더 렌더링
2. 드래그 시 annotation_line_width 업데이트
3. 활성 주석 도구에 새 선 두께 적용

### Find Strategies
- `grep -rn "annotation_line_width" apps/view/src-tauri/src/ui/state.rs`

### Debug ID
`annotation-line-width`

### Verification
- cargo check -p view
