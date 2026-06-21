# Implementation Plan: Keyboard Shortcut Ctrl+C

## Feature ID
view/keyboard-shortcut-ctrl-c

## Spec Reference
`plans/spec/view/keyboard-shortcut-ctrl-c.md`

## Design Reference
`plans/design/view/keyboard-shortcut-ctrl-c.md`

## Implementation Strategy

### Locate
- File: `apps/view/src-tauri/src/ui/mod.rs`
- State: `apps/view/src-tauri/src/ui/state.rs` — `ViewState::current_image`

### Change Recipe
1. Ctrl+C 키 감지
2. 현재 이미지를 시스템 클립보드에 복사

### Find Strategies
- `grep -rn "Ctrl.*C" apps/view/src-tauri/src/ui/mod.rs`

### Debug ID
`keyboard-shortcut-ctrl-c`

### Verification
- cargo check -p view
