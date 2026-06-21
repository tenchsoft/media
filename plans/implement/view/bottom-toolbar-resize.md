# Implementation Plan: Bottom Toolbar Resize

## Feature ID
view/bottom-toolbar-resize

## Spec Reference
`plans/spec/view/bottom-toolbar-resize.md`

## Design Reference
`plans/design/view/bottom-toolbar-resize.md`

## Implementation Strategy

### Locate
- File: `apps/view/src-tauri/src/ui/controls.rs`
- State: `apps/view/src-tauri/src/ui/state.rs` — `ViewState::show_resize_dialog`

### Change Recipe
1. 하단 도구 모음에서 크기 조정 버튼 렌더링
2. 클릭 시 show_resize_dialog = true
3. 크기 조정 대화상자 표시

### Find Strategies
- `grep -rn "show_resize_dialog" apps/view/src-tauri/src/ui/state.rs`

### Debug ID
`bottom-toolbar-resize`

### Verification
- cargo check -p view
