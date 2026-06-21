# Implementation Plan: Context Menu Show in Files

## Feature ID
view/context-menu-show-in-files

## Spec Reference
`plans/spec/view/context-menu-show-in-files.md`

## Design Reference
`plans/design/view/context-menu-show-in-files.md`

## Implementation Strategy

### Locate
- File: `apps/view/src-tauri/src/ui/controls.rs`
- State: `apps/view/src-tauri/src/ui/state.rs` — `ViewState::current_image`
- Platform: `apps/view/src-tauri/src/platform_util.rs`

### Change Recipe
1. 우클릭 컨텍스트 메뉴에 "파일 탐색기에서 보기" 항목 렌더링
2. 클릭 시 platform_util을 통해 파일 탐색기에서 파일 위치 열기

### Find Strategies
- `grep -rn "show_in_files" apps/view/src-tauri/src/platform_util.rs`

### Debug ID
`context-menu-show-in-files`

### Verification
- cargo check -p view
