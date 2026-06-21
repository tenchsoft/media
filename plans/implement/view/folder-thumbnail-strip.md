# Implementation Plan: Folder Thumbnail Strip

## Feature ID
view/folder-thumbnail-strip

## Spec Reference
`plans/spec/view/folder-thumbnail-strip.md`

## Design Reference
`plans/design/view/folder-thumbnail-strip.md`

## Implementation Strategy

### Locate
- File: `apps/view/src-tauri/src/ui/controls.rs`
- State: `apps/view/src-tauri/src/ui/state.rs` — `ViewState::folder_images`, `ViewState::current_index`

### Change Recipe
1. 하단 썸네일 스트립 렌더링
2. 각 이미지의 축소판 표시
3. 클릭 시 해당 이미지로 전환

### Find Strategies
- `grep -rn "folder_images" apps/view/src-tauri/src/ui/state.rs`

### Debug ID
`folder-thumbnail-strip`

### Verification
- cargo check -p view
