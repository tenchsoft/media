# Implementation Plan: Clip Crop

## Feature ID
composer/clip-crop

## Spec Reference
`plans/spec/composer/clip-crop.md`

## Design Reference
`plans/design/composer/clip-crop.md`

## Implementation Strategy

### Locate
- File: `apps/composer/src-tauri/src/ui/inspector.rs`
- Search: `grep -n "crop" apps/composer/src-tauri/src/ui/inspector.rs`
- State: `apps/composer/src-tauri/src/ui/state.rs` — `Clip::crop`

### Change Recipe
1. inspector.rs에서 crop 값 (top, bottom, left, right) 입력 hit-test
2. 값 변경 → Clip::crop 업데이트
3. 프리뷰에서 크롭 영역 표시
4. 리렌더

### Find Strategies
- `grep -rn "crop" apps/composer/src-tauri/src/ui/state.rs`
- `grep -rn "crop" apps/composer/src-tauri/src/ui/inspector.rs`

### Debug ID
`clip-crop`

### Verification
- cargo check -p composer
- 크롭 값 변경 시 프리뷰 반영
