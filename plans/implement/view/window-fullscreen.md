# Implementation Plan: Window Fullscreen

## Feature ID
view/window-fullscreen

## Spec Reference
`plans/spec/view/window-fullscreen.md`

## Design Reference
`plans/design/view/window-fullscreen.md`

## Implementation Strategy

### Locate
- File: `apps/view/src-tauri/src/ui/mod.rs`
- State: `apps/view/src-tauri/src/ui/state.rs` — `ViewState::fullscreen`

### Change Recipe
1. F11 또는 전체화면 버튼 클릭 시 fullscreen 토글
2. 전체화면: 제목 표시줄, 도구 모음 숨김
3. 마우스 이동 시 일시적 표시

### Find Strategies
- `grep -rn "fullscreen" apps/view/src-tauri/src/ui/state.rs`

### Debug ID
`window-fullscreen`

### Verification
- cargo check -p view
