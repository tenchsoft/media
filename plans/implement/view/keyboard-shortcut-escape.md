# Implementation Plan: Keyboard Shortcut Escape

## Feature ID
view/keyboard-shortcut-escape

## Spec Reference
`plans/spec/view/keyboard-shortcut-escape.md`

## Design Reference
`plans/design/view/keyboard-shortcut-escape.md`

## Implementation Strategy

### Locate
- File: `apps/view/src-tauri/src/ui/mod.rs`
- State: `apps/view/src-tauri/src/ui/state.rs` — `ViewState::fullscreen`, `ViewState::crop_mode`, `ViewState::show_settings`

### Change Recipe
1. Escape 키 감지
2. 전체화면이면 창 모드로 복원
3. 크롭 모드면 크롭 취소
4. 설정/정보 패널이 열려있으면 닫기

### Find Strategies
- `grep -rn "Key::Escape" apps/view/src-tauri/src/ui/mod.rs`

### Debug ID
`keyboard-shortcut-escape`

### Verification
- cargo check -p view
