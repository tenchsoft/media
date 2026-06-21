# Implementation Plan: Clip Rotation Transform

## Feature ID
composer/clip-rotation-transform

## Spec Reference
`plans/spec/composer/clip-rotation-transform.md`

## Design Reference
`plans/design/composer/clip-rotation-transform.md`

## Implementation Strategy

### Locate
- File: `apps/composer/src-tauri/src/ui/inspector.rs`
- Search: `grep -n "rotation" apps/composer/src-tauri/src/ui/inspector.rs`
- State: `apps/composer/src-tauri/src/ui/state.rs` — `Clip::rotation`

### Change Recipe
1. inspector.rs에서 rotation 슬라이더/입력 hit-test
2. 값 변경 → Clip::rotation 업데이트 (0~360도)
3. 프리뷰 실시간 반영

### Find Strategies
- `grep -rn "rotation" apps/composer/src-tauri/src/ui/state.rs`
- `grep -rn "rotation" apps/composer/src-tauri/src/ui/inspector.rs`

### Debug ID
`clip-rotation-transform`

### Verification
- cargo check -p composer
- 회전 값 변경 시 프리뷰 반영
