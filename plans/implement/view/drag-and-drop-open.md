# Implementation Plan: Drag and Drop Open

## Feature ID
view/drag-and-drop-open

## Spec Reference
`plans/spec/view/drag-and-drop-open.md`

## Design Reference
`plans/design/view/drag-and-drop-open.md`

## Implementation Strategy

### Locate
- File: `apps/view/src-tauri/src/ui/mod.rs`
- State: `apps/view/src-tauri/src/ui/state.rs` — `ViewState::current_image`

### Change Recipe
1. 파일 드롭 이벤트 감지
2. 이미지 파일인지 확장자 검증
3. current_image에 로드

### Find Strategies
- `grep -rn "drop" apps/view/src-tauri/src/ui/mod.rs`

### Debug ID
`drag-and-drop-open`

### Verification
- cargo check -p view
