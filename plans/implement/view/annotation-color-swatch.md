# Implementation Plan: Annotation Color Swatch

## Feature ID
view/annotation-color-swatch

## Spec Reference
`plans/spec/view/annotation-color-swatch.md`

## Design Reference
`plans/design/view/annotation-color-swatch.md`

## Implementation Strategy

### Locate
- File: `apps/view/src-tauri/src/ui/tools.rs`
- State: `apps/view/src-tauri/src/ui/state.rs` — `ViewState::annotation_color`

### Change Recipe
1. 주석 도구 모음에서 색상 견본 렌더링
2. 클릭 시 annotation_color 업데이트
3. 활성 주석 도구에 새 색상 적용

### Find Strategies
- `grep -rn "annotation_color" apps/view/src-tauri/src/ui/state.rs`

### Debug ID
`annotation-color-swatch`

### Verification
- cargo check -p view
