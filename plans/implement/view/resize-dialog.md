# Implementation Plan: Resize Dialog

## Feature ID
view/resize-dialog

## Spec Reference
`plans/spec/view/resize-dialog.md`

## Design Reference
`plans/design/view/resize-dialog.md`

## Implementation Strategy

### Locate
- File: `apps/view/src-tauri/src/ui/tools.rs`
- State: `apps/view/src-tauri/src/ui/state.rs` — `ViewState::show_resize_dialog`, `ViewState::resize_width`, `ViewState::resize_height`

### Change Recipe
1. 크기 조정 대화상자 렌더링 (가로/세로 입력)
2. 비율 유지 옵션 토글
3. 적용 시 이미지 리샘플링

### Find Strategies
- `grep -rn "resize" apps/view/src-tauri/src/ui/state.rs`

### Debug ID
`resize-dialog`

### Verification
- cargo check -p view
