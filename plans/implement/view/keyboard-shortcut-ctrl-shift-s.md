# Implementation Plan: Keyboard Shortcut Ctrl+Shift+S

## Feature ID
view/keyboard-shortcut-ctrl-shift-s

## Spec Reference
`plans/spec/view/keyboard-shortcut-ctrl-shift-s.md`

## Design Reference
`plans/design/view/keyboard-shortcut-ctrl-shift-s.md`

## Implementation Strategy

### Locate
- File: `apps/view/src-tauri/src/ui/mod.rs`
- State: `apps/view/src-tauri/src/ui/state.rs` — `ViewState::current_image`

### Change Recipe
1. Ctrl+Shift+S 키 감지
2. 다른 이름으로 저장 대화상자 표시

### Find Strategies
- `grep -rn "Ctrl.*Shift.*S" apps/view/src-tauri/src/ui/mod.rs`

### Debug ID
`keyboard-shortcut-ctrl-shift-s`

### Verification
- cargo check -p view
