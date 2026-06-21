# Implementation Plan: Folder Navigate Next

## Feature ID
view/folder-navigate-next

## Spec Reference
`plans/spec/view/folder-navigate-next.md`

## Design Reference
`plans/design/view/folder-navigate-next.md`

## Implementation Strategy

### Locate
- File: `apps/view/src-tauri/src/ui/controls.rs`
- State: `apps/view/src-tauri/src/ui/state.rs` — `ViewState::folder_images`, `ViewState::current_index`

### Change Recipe
1. 다음 이미지 버튼 또는 오른쪽 화살표 클릭
2. current_index += 1 (최대 folder_images.len() - 1)
3. 다음 이미지 로드

### Find Strategies
- `grep -rn "current_index" apps/view/src-tauri/src/ui/state.rs`

### Debug ID
`folder-navigate-next`

### Verification
- cargo check -p view
