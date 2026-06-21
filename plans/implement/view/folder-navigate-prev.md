# Implementation Plan: Folder Navigate Prev

## Feature ID
view/folder-navigate-prev

## Spec Reference
`plans/spec/view/folder-navigate-prev.md`

## Design Reference
`plans/design/view/folder-navigate-prev.md`

## Implementation Strategy

### Locate
- File: `apps/view/src-tauri/src/ui/controls.rs`
- State: `apps/view/src-tauri/src/ui/state.rs` — `ViewState::folder_images`, `ViewState::current_index`

### Change Recipe
1. 이전 이미지 버튼 또는 왼쪽 화살표 클릭
2. current_index -= 1 (최소 0)
3. 이전 이미지 로드

### Find Strategies
- `grep -rn "current_index" apps/view/src-tauri/src/ui/state.rs`

### Debug ID
`folder-navigate-prev`

### Verification
- cargo check -p view
