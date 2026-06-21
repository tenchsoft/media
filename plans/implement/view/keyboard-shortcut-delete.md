# Implementation Plan: Keyboard Shortcut Delete

## Feature ID
view/keyboard-shortcut-delete

## Spec Reference
`plans/spec/view/keyboard-shortcut-delete.md`

## Design Reference
`plans/design/view/keyboard-shortcut-delete.md`

## Implementation Strategy

### Locate
- File: `apps/view/src-tauri/src/ui/mod.rs`
- State: `apps/view/src-tauri/src/ui/state.rs` — `ViewState::current_image`

### Change Recipe
1. Delete 키 감지
2. 삭제 확인 후 파일 시스템에서 삭제
3. 다음 이미지 자동 로드

### Find Strategies
- `grep -rn "Key::Delete" apps/view/src-tauri/src/ui/mod.rs`

### Debug ID
`keyboard-shortcut-delete`

### Verification
- cargo check -p view
