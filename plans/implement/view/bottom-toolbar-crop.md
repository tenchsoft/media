# Implementation Plan: Bottom Toolbar Crop

## Feature ID
view/bottom-toolbar-crop

## Spec Reference
`plans/spec/view/bottom-toolbar-crop.md`

## Design Reference
`plans/design/view/bottom-toolbar-crop.md`

## Implementation Strategy

### Locate
- File: `apps/view/src-tauri/src/ui/controls.rs`
- State: `apps/view/src-tauri/src/ui/state.rs` — `ViewState::crop_mode`

### Change Recipe
1. 하단 도구 모음에서 크롭 버튼 렌더링
2. 클릭 시 crop_mode = true
3. 캔버스에 크롭 오버레이 표시

### Find Strategies
- `grep -rn "crop_mode" apps/view/src-tauri/src/ui/state.rs`

### Debug ID
`bottom-toolbar-crop`

### Verification
- cargo check -p view
