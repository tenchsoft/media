# Implementation Plan: Undo

## Feature ID
view/undo

## Spec Reference
`plans/spec/view/undo.md`

## Design Reference
`plans/design/view/undo.md`

## Implementation Strategy

### Locate
- File: `apps/view/src-tauri/src/ui/mod.rs`
- State: `apps/view/src-tauri/src/ui/state.rs` — `ViewState::undo_stack`

### Change Recipe
1. Ctrl+Z 감지
2. undo_stack에서 이전 상태 복원
3. 이미지, 회전, 반전 상태 복원

### Find Strategies
- `grep -rn "undo" apps/view/src-tauri/src/ui/state.rs`

### Debug ID
`undo`

### Verification
- cargo check -p view
