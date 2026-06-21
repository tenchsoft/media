# Implementation Plan: Keyboard Shortcut Ctrl+S

## Feature ID
view/keyboard-shortcut-ctrl-s

## Spec Reference
`plans/spec/view/keyboard-shortcut-ctrl-s.md`

## Design Reference
`plans/design/view/keyboard-shortcut-ctrl-s.md`

## Implementation Strategy

### Locate
- File: `apps/view/src-tauri/src/ui/mod.rs`
- State: `apps/view/src-tauri/src/ui/state.rs` — `ViewState::current_image`

### Change Recipe
1. Ctrl+S 키 감지
2. 현재 이미지를 원본 경로에 저장

### Find Strategies
- `grep -rn "Ctrl.*S" apps/view/src-tauri/src/ui/mod.rs`

### Debug ID
`keyboard-shortcut-ctrl-s`

### Verification
- cargo check -p view
