# Implementation Plan: Context Menu Rename

## Feature ID
view/context-menu-rename

## Spec Reference
`plans/spec/view/context-menu-rename.md`

## Design Reference
`plans/design/view/context-menu-rename.md`

## Implementation Strategy

### Locate
- File: `apps/view/src-tauri/src/ui/controls.rs`
- State: `apps/view/src-tauri/src/ui/state.rs` — `ViewState::current_image`

### Change Recipe
1. 우클릭 컨텍스트 메뉴에 "이름 바꾸기" 항목 렌더링
2. 클릭 시 인라인 이름 편집 또는 대화상자 표시
3. 입력 완료 시 파일 시스템에서 이름 변경

### Find Strategies
- `grep -rn "rename" apps/view/src-tauri/src/ui/controls.rs`

### Debug ID
`context-menu-rename`

### Verification
- cargo check -p view
