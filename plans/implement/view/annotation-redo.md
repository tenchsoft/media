# Implementation Plan: Annotation Redo

## Feature ID
view/annotation-redo

## Spec Reference
`plans/spec/view/annotation-redo.md`

## Design Reference
`plans/design/view/annotation-redo.md`

## Implementation Strategy

### Locate
- File: `apps/view/src-tauri/src/ui/tools.rs`
- State: `apps/view/src-tauri/src/ui/state.rs` — `ViewState::annotation_redo_stack`

### Change Recipe
1. 주석 다시 실행 버튼 렌더링
2. 클릭 시 annotation_redo_stack에서 다음 상태 복원
3. 마지막에 실행 취소된 주석 요소 재추가

### Find Strategies
- `grep -rn "annotation_redo" apps/view/src-tauri/src/ui/state.rs`

### Debug ID
`annotation-redo`

### Verification
- cargo check -p view
