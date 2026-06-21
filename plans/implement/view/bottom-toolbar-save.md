# Implementation Plan: Bottom Toolbar Save

## Feature ID
view/bottom-toolbar-save

## Spec Reference
`plans/spec/view/bottom-toolbar-save.md`

## Design Reference
`plans/design/view/bottom-toolbar-save.md`

## Implementation Strategy

### Locate
- File: `apps/view/src-tauri/src/ui/controls.rs`
- State: `apps/view/src-tauri/src/ui/state.rs` — `ViewState::current_image`

### Change Recipe
1. 하단 도구 모음에서 저장 버튼 렌더링
2. 클릭 시 현재 이미지를 원본 경로에 저장
3. 변경 사항 없으면 버튼 비활성화

### Find Strategies
- `grep -rn "current_image" apps/view/src-tauri/src/ui/state.rs`

### Debug ID
`bottom-toolbar-save`

### Verification
- cargo check -p view
