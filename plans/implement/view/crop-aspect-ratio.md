# Implementation Plan: Crop Aspect Ratio

## Feature ID
view/crop-aspect-ratio

## Spec Reference
`plans/spec/view/crop-aspect-ratio.md`

## Design Reference
`plans/design/view/crop-aspect-ratio.md`

## Implementation Strategy

### Locate
- File: `apps/view/src-tauri/src/ui/tools.rs`
- State: `apps/view/src-tauri/src/ui/state.rs` — `ViewState::crop_aspect_ratio`

### Change Recipe
1. 크롭 모드에서 비율 선택 드롭다운 렌더링
2. 선택 시 crop_aspect_ratio 업데이트
3. 크롭 영역이 선택된 비율에 맞게 제한

### Find Strategies
- `grep -rn "crop_aspect" apps/view/src-tauri/src/ui/state.rs`

### Debug ID
`crop-aspect-ratio`

### Verification
- cargo check -p view
