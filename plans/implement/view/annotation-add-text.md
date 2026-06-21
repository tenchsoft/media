# Implementation Plan: Annotation Add Text

## Feature ID
view/annotation-add-text

## Spec Reference
`plans/spec/view/annotation-add-text.md`

## Design Reference
`plans/design/view/annotation-add-text.md`

## Implementation Strategy

### Locate
- File: `apps/view/src-tauri/src/ui/tools.rs`
- State: `apps/view/src-tauri/src/ui/state.rs` — `ViewState::annotation_tool`

### Change Recipe
1. 텍스트 도구 버튼 렌더링
2. 클릭 시 annotation_tool = Text
3. 캔버스 클릭 위치에 텍스트 입력 필드 표시

### Find Strategies
- `grep -rn "annotation_tool" apps/view/src-tauri/src/ui/state.rs`

### Debug ID
`annotation-add-text`

### Verification
- cargo check -p view
