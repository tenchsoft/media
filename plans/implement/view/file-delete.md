# Implementation Plan: File Delete

## Feature ID
view/file-delete

## Spec Reference
`plans/spec/view/file-delete.md`

## Design Reference
`plans/design/view/file-delete.md`

## Implementation Strategy

### Locate
- File: `apps/view/src-tauri/src/ui/controls.rs`
- State: `apps/view/src-tauri/src/ui/state.rs` — `ViewState::current_image`

### Change Recipe
1. 삭제 확인 대화상자 표시
2. 확인 시 파일 시스템에서 삭제
3. 폴더 내 다음 이미지 자동 로드

### Find Strategies
- `grep -rn "delete" apps/view/src-tauri/src/commands.rs`

### Debug ID
`file-delete`

### Verification
- cargo check -p view
