# Implementation Plan: Crop Apply

## Feature ID
view/crop-apply

## Spec Reference
`plans/spec/view/crop-apply.md`

## Design Reference
`plans/design/view/crop-apply.md`

## Implementation Strategy

### Locate
- File: `apps/view/src-tauri/src/ui/tools.rs`
- State: `apps/view/src-tauri/src/ui/state.rs` — `ViewState::crop_rect`, `ViewState::crop_mode`

### Change Recipe
1. 크롭 적용 버튼 렌더링
2. 클릭 시 crop_rect 영역으로 이미지 잘라내기
3. 결과 이미지를 current_image에 반영, crop_mode = false

### Find Strategies
- `grep -rn "crop_rect" apps/view/src-tauri/src/ui/state.rs`

### Debug ID
`crop-apply`

### Verification
- cargo check -p view
