# Implementation Plan: File Copy

## Feature ID
view/file-copy

## Spec Reference
`plans/spec/view/file-copy.md`

## Design Reference
`plans/design/view/file-copy.md`

## Implementation Strategy

### Locate
- File: `apps/view/src-tauri/src/ui/controls.rs`
- State: `apps/view/src-tauri/src/ui/state.rs` — `ViewState::current_image`

### Change Recipe
1. 현재 이미지를 시스템 클립보드에 복사

### Find Strategies
- `grep -rn "clipboard" apps/view/src-tauri/src/`

### Debug ID
`file-copy`

### Verification
- cargo check -p view
