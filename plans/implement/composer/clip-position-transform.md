# Implementation Plan: Clip Position Transform

## Feature ID
composer/clip-position-transform

## Spec Reference
`plans/spec/composer/clip-position-transform.md`

## Design Reference
`plans/design/composer/clip-position-transform.md`

## Implementation Strategy

### Locate
- File: `apps/composer/src-tauri/src/ui/inspector.rs`, `apps/composer/src-tauri/src/ui/preview.rs`
- Search: `grep -n "position" apps/composer/src-tauri/src/ui/inspector.rs`
- State: `apps/composer/src-tauri/src/ui/state.rs` — `Clip::transform_x`, `Clip::transform_y`

### Change Recipe
1. inspector.rs에서 X/Y 위치 입력 hit-test
2. 값 변경 → Clip::transform_x, transform_y 업데이트
3. 프리뷰에서 클립 위치 이동
4. 프리뷰에서 직접 드래그도 지원

### Find Strategies
- `grep -rn "transform_x\|transform_y" apps/composer/src-tauri/src/ui/state.rs`
- `grep -rn "position" apps/composer/src-tauri/src/ui/inspector.rs`

### Debug ID
`clip-position-transform`

### Verification
- cargo check -p composer
- 위치 변경 시 프리뷰 반영
