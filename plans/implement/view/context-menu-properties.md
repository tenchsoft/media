# Implementation Plan: Context Menu Properties

## Feature ID
view/context-menu-properties

## Spec Reference
`plans/spec/view/context-menu-properties.md`

## Design Reference
`plans/design/view/context-menu-properties.md`

## Implementation Strategy

### Locate
- File: `apps/view/src-tauri/src/ui/controls.rs`
- State: `apps/view/src-tauri/src/ui/state.rs` — `ViewState::show_info_panel`

### Change Recipe
1. 우클릭 컨텍스트 메뉴에 "속성" 항목 렌더링
2. 클릭 시 show_info_panel = true
3. 파일 정보 + EXIF 패널 열림

### Find Strategies
- `grep -rn "show_info_panel" apps/view/src-tauri/src/ui/state.rs`

### Debug ID
`context-menu-properties`

### Verification
- cargo check -p view
