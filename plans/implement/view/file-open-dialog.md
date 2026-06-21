# Implementation Plan: File Open Dialog

## Feature ID
view/file-open-dialog

## Spec Reference
`plans/spec/view/file-open-dialog.md`

## Design Reference
`plans/design/view/file-open-dialog.md`

## Implementation Strategy

### Locate
- File: `apps/view/src-tauri/src/ui/controls.rs`
- State: `apps/view/src-tauri/src/ui/state.rs` — `ViewState::current_image`

### Change Recipe
1. 파일 열기 대화상자 호출 (지원 포맷 필터)
2. 선택된 파일을 current_image에 로드
3. 폴더 내 이미지 목록 구성하여 탐색 가능하게

### Find Strategies
- `grep -rn "open_file" apps/view/src-tauri/src/commands.rs`

### Debug ID
`file-open-dialog`

### Verification
- cargo check -p view
