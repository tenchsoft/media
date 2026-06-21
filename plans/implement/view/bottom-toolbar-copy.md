# Implementation Plan: Bottom Toolbar Copy

## Feature ID
view/bottom-toolbar-copy

## Spec Reference
`plans/spec/view/bottom-toolbar-copy.md`

## Design Reference
`plans/design/view/bottom-toolbar-copy.md`

## Implementation Strategy

### Locate
- File: `apps/view/src-tauri/src/ui/controls.rs`
- State: `apps/view/src-tauri/src/ui/state.rs` — `ViewState::current_image`

### Change Recipe
1. 하단 도구 모음에서 복사 버튼 렌더링
2. 클릭 시 현재 이미지를 시스템 클립보드에 복사

### Find Strategies
- `grep -rn "current_image" apps/view/src-tauri/src/ui/state.rs`

### Debug ID
`bottom-toolbar-copy`

### Verification
- cargo check -p view
