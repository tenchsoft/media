# Implementation Plan: Context Menu Delete File

## Feature ID
view/context-menu-delete-file

## Spec Reference
`plans/spec/view/context-menu-delete-file.md`

## Design Reference
`plans/design/view/context-menu-delete-file.md`

## Implementation Strategy

### Locate
- File: `apps/view/src-tauri/src/ui/controls.rs`
- State: `apps/view/src-tauri/src/ui/state.rs` — `ViewState::current_image`

### Change Recipe
1. 우클릭 컨텍스트 메뉴에 "삭제" 항목 렌더링
2. 클릭 시 확인 대화상자 표시
3. 확인 시 파일 삭제, 다음 이미지 로드

### Find Strategies
- `grep -rn "context_menu" apps/view/src-tauri/src/ui/controls.rs`

### Debug ID
`context-menu-delete-file`

### Verification
- cargo check -p view
