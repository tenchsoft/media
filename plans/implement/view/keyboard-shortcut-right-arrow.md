# Implementation Plan: Keyboard Shortcut Right Arrow

## Feature ID
view/keyboard-shortcut-right-arrow

## Spec Reference
`plans/spec/view/keyboard-shortcut-right-arrow.md`

## Design Reference
`plans/design/view/keyboard-shortcut-right-arrow.md`

## Implementation Strategy

### Locate
- File: `apps/view/src-tauri/src/ui/mod.rs`
- State: `apps/view/src-tauri/src/ui/state.rs` — `ViewState::current_index`

### Change Recipe
1. 오른쪽 화살표 키 감지
2. current_index가 마지막보다 작으면 다음 이미지 로드

### Find Strategies
- `grep -rn "Key::ArrowRight" apps/view/src-tauri/src/ui/mod.rs`

### Debug ID
`keyboard-shortcut-right-arrow`

### Verification
- cargo check -p view
