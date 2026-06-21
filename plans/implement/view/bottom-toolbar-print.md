# Implementation Plan: Bottom Toolbar Print

## Feature ID
view/bottom-toolbar-print

## Spec Reference
`plans/spec/view/bottom-toolbar-print.md`

## Design Reference
`plans/design/view/bottom-toolbar-print.md`

## Implementation Strategy

### Locate
- File: `apps/view/src-tauri/src/ui/controls.rs`
- State: `apps/view/src-tauri/src/ui/state.rs` — `ViewState::current_image`

### Change Recipe
1. 하단 도구 모음에서 인쇄 버튼 렌더링
2. 클릭 시 시스템 인쇄 대화상자 호출
3. 현재 이미지를 인쇄 작업에 전달

### Find Strategies
- `grep -rn "current_image" apps/view/src-tauri/src/ui/state.rs`

### Debug ID
`bottom-toolbar-print`

### Verification
- cargo check -p view
