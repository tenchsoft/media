# Implementation Plan: Bottom Toolbar Set Wallpaper

## Feature ID
view/bottom-toolbar-set-wallpaper

## Spec Reference
`plans/spec/view/bottom-toolbar-set-wallpaper.md`

## Design Reference
`plans/design/view/bottom-toolbar-set-wallpaper.md`

## Implementation Strategy

### Locate
- File: `apps/view/src-tauri/src/ui/controls.rs`
- State: `apps/view/src-tauri/src/ui/state.rs` — `ViewState::current_image`

### Change Recipe
1. 하단 도구 모음에서 배경화면 설정 버튼 렌더링
2. 클릭 시 platform_util을 통해 시스템 배경화면으로 설정

### Find Strategies
- `grep -rn "set_wallpaper" apps/view/src-tauri/src/`

### Debug ID
`bottom-toolbar-set-wallpaper`

### Verification
- cargo check -p view
