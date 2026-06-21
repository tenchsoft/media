# Implementation Plan: Redo

## Feature ID
view/redo

## Spec Reference
`plans/spec/view/redo.md`

## Design Reference
`plans/design/view/redo.md`

## Implementation Strategy

### Locate
- File: `apps/view/src-tauri/src/ui/mod.rs`
- State: `apps/view/src-tauri/src/ui/state.rs` — `ViewState::redo_stack`

### Change Recipe
1. Ctrl+Shift+Z 또는 Ctrl+Y 감지
2. redo_stack에서 다음 상태 복원
3. 이미지, 회전, 반전 상태 복원

### Find Strategies
- `grep -rn "redo" apps/view/src-tauri/src/ui/state.rs`

### Debug ID
`redo`

### Verification
- cargo check -p view
