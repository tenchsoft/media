# Implementation Plan: Annotation Clear All

## Feature ID
view/annotation-clear-all

## Spec Reference
`plans/spec/view/annotation-clear-all.md`

## Design Reference
`plans/design/view/annotation-clear-all.md`

## Implementation Strategy

### Locate
- File: `apps/view/src-tauri/src/ui/tools.rs`
- State: `apps/view/src-tauri/src/ui/state.rs` — `ViewState::annotations`

### Change Recipe
1. 전체 삭제 버튼 렌더링
2. 클릭 시 annotations 벡터 클리어
3. 캔버스에서 모든 주석 제거

### Find Strategies
- `grep -rn "annotations" apps/view/src-tauri/src/ui/state.rs`

### Debug ID
`annotation-clear-all`

### Verification
- cargo check -p view
