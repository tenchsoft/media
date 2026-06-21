# Implementation Plan: Context Menu Open With

## Feature ID
view/context-menu-open-with

## Spec Reference
`plans/spec/view/context-menu-open-with.md`

## Design Reference
`plans/design/view/context-menu-open-with.md`

## Implementation Strategy

### Locate
- File: `apps/view/src-tauri/src/ui/controls.rs`
- State: `apps/view/src-tauri/src/ui/state.rs` — `ViewState::current_image`
- Platform: `apps/view/src-tauri/src/platform_util.rs`

### Change Recipe
1. 우클릭 컨텍스트 메뉴에 "다른 앱으로 열기" 항목 렌더링
2. 클릭 시 platform_util을 통해 시스템 앱 선택기 호출

### Find Strategies
- `grep -rn "open_with" apps/view/src-tauri/src/platform_util.rs`

### Debug ID
`context-menu-open-with`

### Verification
- cargo check -p view
