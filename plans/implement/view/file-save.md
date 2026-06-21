# Implementation Plan: File Save

## Feature ID
view/file-save

## Spec Reference
`plans/spec/view/file-save.md`

## Design Reference
`plans/design/view/file-save.md`

## Implementation Strategy

### Locate
- File: `apps/view/src-tauri/src/ui/controls.rs`
- State: `apps/view/src-tauri/src/ui/state.rs` — `ViewState::current_image`

### Change Recipe
1. 저장 시 현재 이미지를 원본 경로에 저장
2. 변경 사항 없으면 no-op

### Find Strategies
- `grep -rn "save" apps/view/src-tauri/src/commands.rs`

### Debug ID
`file-save`

### Verification
- cargo check -p view
