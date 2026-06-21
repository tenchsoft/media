# Implementation Plan: Bottom Toolbar Delete

## Feature ID
view/bottom-toolbar-delete

## Spec Reference
`plans/spec/view/bottom-toolbar-delete.md`

## Design Reference
`plans/design/view/bottom-toolbar-delete.md`

## Implementation Strategy

### Locate
- File: `apps/view/src-tauri/src/ui/controls.rs`
- State: `apps/view/src-tauri/src/ui/state.rs` — `ViewState::current_image`

### Change Recipe
1. 하단 도구 모음에서 삭제 버튼 렌더링
2. 클릭 시 확인 대화상자 표시
3. 확인 시 파일 시스템에서 삭제, 다음 이미지 로드

### Find Strategies
- `grep -rn "current_image" apps/view/src-tauri/src/ui/state.rs`

### Debug ID
`bottom-toolbar-delete`

### Verification
- cargo check -p view
