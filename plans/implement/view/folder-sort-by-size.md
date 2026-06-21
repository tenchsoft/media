# Implementation Plan: Folder Sort By Size

## Feature ID
view/folder-sort-by-size

## Spec Reference
`plans/spec/view/folder-sort-by-size.md`

## Design Reference
`plans/design/view/folder-sort-by-size.md`

## Implementation Strategy

### Locate
- File: `apps/view/src-tauri/src/ui/controls.rs`
- State: `apps/view/src-tauri/src/ui/state.rs` — `ViewState::folder_images`, `ViewState::sort_mode`

### Change Recipe
1. 크기순 정렬 선택 시 sort_mode = BySize
2. folder_images를 파일 크기 기준으로 재정렬
3. 현재 이미지 인덱스 업데이트

### Find Strategies
- `grep -rn "sort_mode" apps/view/src-tauri/src/ui/state.rs`

### Debug ID
`folder-sort-by-size`

### Verification
- cargo check -p view
