# Implementation Plan: Keyframe Interpolation

## Feature ID
composer/keyframe-interpolation

## Spec Reference
`plans/spec/composer/keyframe-interpolation.md`

## Design Reference
`plans/design/composer/keyframe-interpolation.md`

## Implementation Strategy

### Locate
- File: `apps/composer/src-tauri/src/ui/inspector.rs`
- Search: `grep -n "interpolation" apps/composer/src-tauri/src/ui/inspector.rs`
- State: `apps/composer/src-tauri/src/ui/state.rs` — `Keyframe::interpolation`

### Change Recipe
1. inspector.rs에서 키프레임 우클릭 → 보간 메뉴
2. 옵션: Linear, Ease In, Ease Out, Bezier
3. 선택 → Keyframe::interpolation 값 변경
4. 프리뷰에 보간 적용

### Find Strategies
- `grep -rn "interpolation" apps/composer/src-tauri/src/ui/state.rs`
- `grep -rn "interpolation" apps/composer/src-tauri/src/ui/inspector.rs`

### Debug ID
`keyframe-interpolation`

### Verification
- cargo check -p composer
- 보간 변경 시 애니메이션 곡선 변경
