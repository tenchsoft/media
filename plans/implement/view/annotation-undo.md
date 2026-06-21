# Implementation Plan: Annotation Undo

## Feature ID
view/annotation-undo

## Spec Reference
`plans/spec/view/annotation-undo.md`

## Design Reference
`plans/design/view/annotation-undo.md`

## Implementation Strategy

### Locate
- File: `apps/view/src-tauri/src/ui/tools.rs`
- State: `apps/view/src-tauri/src/ui/state.rs` — `ViewState::annotation_undo_stack`

### Change Recipe
1. 주석 실행 취소 버튼 렌더링
2. 클릭 시 annotation_undo_stack에서 이전 상태 복원
3. 마지막 주석 요소 제거

### Find Strategies
- `grep -rn "annotation_undo" apps/view/src-tauri/src/ui/state.rs`

### Debug ID
`annotation-undo`

### Verification
- cargo check -p view
