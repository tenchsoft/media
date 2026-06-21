# Implementation Plan: Set Wallpaper

## Feature ID
view/set-wallpaper

## Spec Reference
`plans/spec/view/set-wallpaper.md`

## Design Reference
`plans/design/view/set-wallpaper.md`

## Implementation Strategy

### Locate
- File: `apps/view/src-tauri/src/platform_util.rs`
- State: `apps/view/src-tauri/src/ui/state.rs` — `ViewState::current_image`

### Change Recipe
1. platform_util을 통해 시스템 배경화면으로 설정
2. OS별 네이티브 API 호출

### Find Strategies
- `grep -rn "wallpaper" apps/view/src-tauri/src/platform_util.rs`

### Debug ID
`set-wallpaper`

### Verification
- cargo check -p view
