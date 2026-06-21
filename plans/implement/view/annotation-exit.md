# Implementation Plan: Annotation Exit

## Feature ID
view/annotation-exit

## Spec Reference
`plans/spec/view/annotation-exit.md`

## Design Reference
`plans/design/view/annotation-exit.md`

## Implementation Strategy

### Locate
- File: `apps/view/src-tauri/src/ui/tools.rs`
- State: `apps/view/src-tauri/src/ui/state.rs` — `ViewState::annotation_mode`

### Change Recipe
1. 주석 모드 종료 버튼 렌더링
2. 클릭 시 annotation_mode = false
3. 주석 도구 모음 숨김, 이전 상태 복원

### Find Strategies
- `grep -rn "annotation_mode" apps/view/src-tauri/src/ui/state.rs`

### Debug ID
`annotation-exit`

### Verification
- cargo check -p view
