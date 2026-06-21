# Implementation Plan: Bottom Toolbar Settings

## Feature ID
view/bottom-toolbar-settings

## Spec Reference
`plans/spec/view/bottom-toolbar-settings.md`

## Design Reference
`plans/design/view/bottom-toolbar-settings.md`

## Implementation Strategy

### Locate
- File: `apps/view/src-tauri/src/ui/controls.rs`
- State: `apps/view/src-tauri/src/ui/state.rs` — `ViewState::show_settings`

### Change Recipe
1. 하단 도구 모음에서 설정 버튼 렌더링
2. 클릭 시 show_settings 토글
3. 설정 패널 열림/닫힘

### Find Strategies
- `grep -rn "show_settings" apps/view/src-tauri/src/ui/state.rs`

### Debug ID
`bottom-toolbar-settings`

### Verification
- cargo check -p view
