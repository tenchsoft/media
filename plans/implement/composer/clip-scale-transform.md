# Implementation Plan: Clip Scale Transform

## Feature ID
composer/clip-scale-transform

## Spec Reference
`plans/spec/composer/clip-scale-transform.md`

## Design Reference
`plans/design/composer/clip-scale-transform.md`

## Implementation Strategy

### Locate
- File: `apps/composer/src-tauri/src/ui/inspector.rs`
- Search: `grep -n "scale" apps/composer/src-tauri/src/ui/inspector.rs`
- State: `apps/composer/src-tauri/src/ui/state.rs` — `Clip::scale_x`, `Clip::scale_y`

### Change Recipe
1. inspector.rs에서 scale 슬라이더/입력 hit-test
2. 값 변경 → Clip::scale_x, scale_y 업데이트
3. 비율 잠금 옵션
4. 프리뷰 실시간 반영

### Find Strategies
- `grep -rn "scale_x\|scale_y" apps/composer/src-tauri/src/ui/state.rs`
- `grep -rn "scale" apps/composer/src-tauri/src/ui/inspector.rs`

### Debug ID
`clip-scale-transform`

### Verification
- cargo check -p composer
- 스케일 변경 시 프리뷰 반영
