# Implementation Plan: Crop Cancel

## Feature ID
view/crop-cancel

## Spec Reference
`plans/spec/view/crop-cancel.md`

## Design Reference
`plans/design/view/crop-cancel.md`

## Implementation Strategy

### Locate
- File: `apps/view/src-tauri/src/ui/tools.rs`
- State: `apps/view/src-tauri/src/ui/state.rs` — `ViewState::crop_mode`

### Change Recipe
1. 크롭 취소 버튼 렌더링
2. 클릭 시 crop_mode = false
3. 크롭 오버레이 제거, 원본 이미지 유지

### Find Strategies
- `grep -rn "crop_mode" apps/view/src-tauri/src/ui/state.rs`

### Debug ID
`crop-cancel`

### Verification
- cargo check -p view
