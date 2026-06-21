# Implementation Plan: Keyboard Shortcut Left Arrow

## Feature ID
view/keyboard-shortcut-left-arrow

## Spec Reference
`plans/spec/view/keyboard-shortcut-left-arrow.md`

## Design Reference
`plans/design/view/keyboard-shortcut-left-arrow.md`

## Implementation Strategy

### Locate
- File: `apps/view/src-tauri/src/ui/mod.rs`
- State: `apps/view/src-tauri/src/ui/state.rs` — `ViewState::current_index`

### Change Recipe
1. 왼쪽 화살표 키 감지
2. current_index가 0보다 크면 이전 이미지 로드

### Find Strategies
- `grep -rn "Key::ArrowLeft" apps/view/src-tauri/src/ui/mod.rs`

### Debug ID
`keyboard-shortcut-left-arrow`

### Verification
- cargo check -p view
