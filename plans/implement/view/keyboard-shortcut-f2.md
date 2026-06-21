# Implementation Plan: Keyboard Shortcut F2

## Feature ID
view/keyboard-shortcut-f2

## Spec Reference
`plans/spec/view/keyboard-shortcut-f2.md`

## Design Reference
`plans/design/view/keyboard-shortcut-f2.md`

## Implementation Strategy

### Locate
- File: `apps/view/src-tauri/src/ui/mod.rs`
- State: `apps/view/src-tauri/src/ui/state.rs` — `ViewState::current_image`

### Change Recipe
1. F2 키 감지
2. 인라인 이름 편집 모드 활성화
3. Enter로 확정, Escape로 취소

### Find Strategies
- `grep -rn "Key::F2" apps/view/src-tauri/src/ui/mod.rs`

### Debug ID
`keyboard-shortcut-f2`

### Verification
- cargo check -p view
