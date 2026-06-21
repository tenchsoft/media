# Implementation Plan: Bottom Toolbar Info

## Feature ID
view/bottom-toolbar-info

## Spec Reference
`plans/spec/view/bottom-toolbar-info.md`

## Design Reference
`plans/design/view/bottom-toolbar-info.md`

## Implementation Strategy

### Locate
- File: `apps/view/src-tauri/src/ui/controls.rs`
- State: `apps/view/src-tauri/src/ui/state.rs` — `ViewState::show_info_panel`

### Change Recipe
1. 하단 도구 모음에서 정보 버튼 렌더링
2. 클릭 시 show_info_panel 토글
3. 이미지 정보 패널 열림/닫힘

### Find Strategies
- `grep -rn "show_info_panel" apps/view/src-tauri/src/ui/state.rs`

### Debug ID
`bottom-toolbar-info`

### Verification
- cargo check -p view
