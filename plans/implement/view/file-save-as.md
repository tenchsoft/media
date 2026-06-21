# Implementation Plan: File Save As

## Feature ID
view/file-save-as

## Spec Reference
`plans/spec/view/file-save-as.md`

## Design Reference
`plans/design/view/file-save-as.md`

## Implementation Strategy

### Locate
- File: `apps/view/src-tauri/src/ui/controls.rs`
- State: `apps/view/src-tauri/src/ui/state.rs` — `ViewState::current_image`

### Change Recipe
1. 다른 이름으로 저장 대화상자 표시
2. 포맷 선택 (PNG, JPEG, BMP, WebP 등)
3. 선택된 경로 + 포맷으로 저장

### Find Strategies
- `grep -rn "save_as" apps/view/src-tauri/src/commands.rs`

### Debug ID
`file-save-as`

### Verification
- cargo check -p view
