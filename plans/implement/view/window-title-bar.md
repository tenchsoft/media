# Implementation Plan: Window Title Bar

## Feature ID
view/window-title-bar

## Spec Reference
`plans/spec/view/window-title-bar.md`

## Design Reference
`plans/design/view/window-title-bar.md`

## Implementation Strategy

### Locate
- File: `apps/view/src-tauri/src/ui/controls.rs`
- State: `apps/view/src-tauri/src/ui/state.rs` — `ViewState::current_image`

### Change Recipe
1. 제목 표시줄에 파일명 + 앱 이름 표시
2. 이미지 미열 시 앱 이름만 표시
3. 변경 사항 있으면 수정 표시자 (*) 추가

### Find Strategies
- `grep -rn "title" apps/view/src-tauri/src/ui/controls.rs`

### Debug ID
`window-title-bar`

### Verification
- cargo check -p view
