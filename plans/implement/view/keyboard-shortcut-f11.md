# Implementation Plan: Keyboard Shortcut F11

## Feature ID
view/keyboard-shortcut-f11

## Spec Reference
`plans/spec/view/keyboard-shortcut-f11.md`

## Design Reference
`plans/design/view/keyboard-shortcut-f11.md`

## Implementation Strategy

### Locate
- File: `apps/view/src-tauri/src/ui/mod.rs`
- State: `apps/view/src-tauri/src/ui/state.rs` — `ViewState::fullscreen`

### Change Recipe
1. F11 키 감지
2. fullscreen 토글
3. 전체화면/창 모드 전환

### Find Strategies
- `grep -rn "Key::F11" apps/view/src-tauri/src/ui/mod.rs`

### Debug ID
`keyboard-shortcut-f11`

### Verification
- cargo check -p view
