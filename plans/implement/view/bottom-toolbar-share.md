# Implementation Plan: Bottom Toolbar Share

## Feature ID
view/bottom-toolbar-share

## Spec Reference
`plans/spec/view/bottom-toolbar-share.md`

## Design Reference
`plans/design/view/bottom-toolbar-share.md`

## Implementation Strategy

### Locate
- File: `apps/view/src-tauri/src/ui/controls.rs`
- State: `apps/view/src-tauri/src/ui/state.rs` — `ViewState::current_image`

### Change Recipe
1. 하단 도구 모음에서 공유 버튼 렌더링
2. 클릭 시 시스템 공유 시트 표시
3. 현재 이미지를 공유 시트에 전달

### Find Strategies
- `grep -rn "share" apps/view/src-tauri/src/ui/controls.rs`

### Debug ID
`bottom-toolbar-share`

### Verification
- cargo check -p view
