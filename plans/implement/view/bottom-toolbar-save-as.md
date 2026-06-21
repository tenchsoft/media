# Implementation Plan: Bottom Toolbar Save As

## Feature ID
view/bottom-toolbar-save-as

## Spec Reference
`plans/spec/view/bottom-toolbar-save-as.md`

## Design Reference
`plans/design/view/bottom-toolbar-save-as.md`

## Implementation Strategy

### Locate
- File: `apps/view/src-tauri/src/ui/controls.rs`
- State: `apps/view/src-tauri/src/ui/state.rs` — `ViewState::current_image`

### Change Recipe
1. 하단 도구 모음에서 다른 이름으로 저장 버튼 렌더링
2. 클릭 시 다른 이름으로 저장 대화상자 표시
3. 선택된 경로 + 포맷으로 이미지 저장

### Find Strategies
- `grep -rn "current_image" apps/view/src-tauri/src/ui/state.rs`

### Debug ID
`bottom-toolbar-save-as`

### Verification
- cargo check -p view
