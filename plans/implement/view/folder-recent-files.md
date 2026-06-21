# Implementation Plan: Folder Recent Files

## Feature ID
view/folder-recent-files

## Spec Reference
`plans/spec/view/folder-recent-files.md`

## Design Reference
`plans/design/view/folder-recent-files.md`

## Implementation Strategy

### Locate
- File: `apps/view/src-tauri/src/ui/panels.rs`
- State: `apps/view/src-tauri/src/ui/state.rs` — `ViewState::recent_files`

### Change Recipe
1. 최근 파일 목록 패널 렌더링
2. 파일 열 때마다 recent_files 업데이트
3. 항목 클릭 시 해당 파일 로드

### Find Strategies
- `grep -rn "recent_files" apps/view/src-tauri/src/ui/state.rs`

### Debug ID
`folder-recent-files`

### Verification
- cargo check -p view
