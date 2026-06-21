# Implementation Plan: Keyboard Shortcut Ctrl+O

## Feature ID
view/keyboard-shortcut-ctrl-o

## Spec Reference
`plans/spec/view/keyboard-shortcut-ctrl-o.md`

## Design Reference
`plans/design/view/keyboard-shortcut-ctrl-o.md`

## Implementation Strategy

### Locate
- File: `apps/view/src-tauri/src/ui/mod.rs`
- State: `apps/view/src-tauri/src/ui/state.rs` — `ViewState::current_image`

### Change Recipe
1. Ctrl+O 키 감지
2. 파일 열기 대화상자 표시

### Find Strategies
- `grep -rn "Ctrl.*O" apps/view/src-tauri/src/ui/mod.rs`

### Debug ID
`keyboard-shortcut-ctrl-o`

### Verification
- cargo check -p view
