# Implementation Plan: File Print

## Feature ID
view/file-print

## Spec Reference
`plans/spec/view/file-print.md`

## Design Reference
`plans/design/view/file-print.md`

## Implementation Strategy

### Locate
- File: `apps/view/src-tauri/src/ui/controls.rs`
- State: `apps/view/src-tauri/src/ui/state.rs` — `ViewState::current_image`

### Change Recipe
1. 인쇄 시 시스템 인쇄 대화상자 호출
2. 현재 이미지를 인쇄 작업에 전달

### Find Strategies
- `grep -rn "print" apps/view/src-tauri/src/commands.rs`

### Debug ID
`file-print`

### Verification
- cargo check -p view
