# Implementation Plan: Bottom Toolbar Open File

## Feature ID
view/bottom-toolbar-open-file

## Spec Reference
`plans/spec/view/bottom-toolbar-open-file.md`

## Design Reference
`plans/design/view/bottom-toolbar-open-file.md`

## Implementation Strategy

### Locate
- File: `apps/view/src-tauri/src/ui/controls.rs`
- State: `apps/view/src-tauri/src/ui/state.rs` — `ViewState::current_image`

### Change Recipe
1. 하단 도구 모음에서 열기 버튼 렌더링
2. 클릭 시 파일 열기 대화상자 표시
3. 선택된 파일을 current_image에 로드

### Find Strategies
- `grep -rn "current_image" apps/view/src-tauri/src/ui/state.rs`

### Debug ID
`bottom-toolbar-open-file`

### Verification
- cargo check -p view
