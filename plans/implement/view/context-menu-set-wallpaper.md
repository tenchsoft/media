# Implementation Plan: Context Menu Set Wallpaper

## Feature ID
view/context-menu-set-wallpaper

## Spec Reference
`plans/spec/view/context-menu-set-wallpaper.md`

## Design Reference
`plans/design/view/context-menu-set-wallpaper.md`

## Implementation Strategy

### Locate
- File: `apps/view/src-tauri/src/ui/controls.rs`
- State: `apps/view/src-tauri/src/ui/state.rs` — `ViewState::current_image`
- Platform: `apps/view/src-tauri/src/platform_util.rs`

### Change Recipe
1. 우클릭 컨텍스트 메뉴에 "배경화면으로 설정" 항목 렌더링
2. 클릭 시 platform_util을 통해 시스템 배경화면으로 설정

### Find Strategies
- `grep -rn "set_wallpaper" apps/view/src-tauri/src/platform_util.rs`

### Debug ID
`context-menu-set-wallpaper`

### Verification
- cargo check -p view
